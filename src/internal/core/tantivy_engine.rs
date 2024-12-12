use crate::internal::core::my_error::SearchEngineError;
use std::sync::LazyLock;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::{IndexRecordOption, Schema, TextFieldIndexing, TextOptions, INDEXED, STORED};
use tantivy::tokenizer::{LowerCaser, RemoveLongFilter, Stemmer, TextAnalyzer};
use tantivy::{Document, Index, ReloadPolicy, TantivyDocument};

//延迟初始化
pub static SEARCH_ENGINE: LazyLock<Result<SearchEngine, SearchEngineError>> =
    LazyLock::new(|| SearchEngine::new());

///搜索引擎
pub struct SearchEngine {
    pub index: Index,
}

impl SearchEngine {
    pub fn new() -> Result<Self, SearchEngineError> {
        let mut builder = Schema::builder();
        //分词器
        let options = TextOptions::default().set_indexing_options(
            TextFieldIndexing::default()
                .set_tokenizer("jieba")
                .set_index_option(IndexRecordOption::WithFreqsAndPositions),
        );
        //是否保存
        // .set_stored();
        //定义结构
        builder.add_u64_field("id", INDEXED | STORED);
        builder.add_text_field("title", options.clone() | STORED);
        builder.add_text_field("content", options.clone());
        builder.add_text_field("excerpt", options);
        let schema = builder.build();
        // let index = Index::open_or_create(MmapDirectory::open(Path::new(BLOG_CONFIG.application.search_engine_dir.as_str()))?, schema.clone())?;
        //内存中创建
        let index = Index::create_in_ram(schema.clone());
        let tokenizer = tantivy_jieba::JiebaTokenizer {};
        let analyzer = TextAnalyzer::builder(tokenizer)
            .filter(RemoveLongFilter::limit(40))
            .filter(LowerCaser)
            .filter(Stemmer::default())
            .build();
        //注册分词器
        index.tokenizers().register("jieba", analyzer);
        Ok(SearchEngine { index })
    }

    pub fn search(
        &self,
        query: &str,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<String>, SearchEngineError> {
        //索引读取器
        let reader = self
            .index
            .reader_builder()
            //每次提交后延迟
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()?;
        let searcher = reader.searcher();
        let title = self.index.schema().get_field("title")?;
        let content = self.index.schema().get_field("content")?;
        let excerpt = self.index.schema().get_field("excerpt")?;
        let query_parser = QueryParser::for_index(&self.index, vec![title, content, excerpt]);
        let query = query_parser.parse_query(query)?;
        let top_docs = searcher.search(&query, &TopDocs::with_limit(limit).and_offset(offset))?;
        let mut list = Vec::new();
        for (_score, doc_address) in top_docs {
            let retrieved_doc: TantivyDocument = searcher.doc(doc_address)?;
            list.push(retrieved_doc.to_json(&self.index.schema()))
        }
        Ok(list)
    }
}
