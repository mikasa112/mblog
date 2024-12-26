use crate::internal::core::my_error::SearchEngineError;
use std::sync::{Mutex, OnceLock};
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::{
    Field, IndexRecordOption, Schema, TextFieldIndexing, TextOptions, INDEXED, STORED,
};
use tantivy::tokenizer::{LowerCaser, RemoveLongFilter, Stemmer, TextAnalyzer};
use tantivy::{
    doc, Document, Index, IndexReader, IndexWriter, ReloadPolicy, TantivyDocument, Term,
};

//初始化
pub static SEARCH_ENGINE: OnceLock<SearchEngine> = OnceLock::new();

///搜索引擎
pub struct SearchEngine {
    pub index: Index,
    pub reader: IndexReader,
    pub writer: Mutex<IndexWriter>,
    my_doc: MyDocument,
}

struct MyDocument {
    id: Field,
    title: Field,
    content: Field,
    excerpt: Field,
}

pub struct PostDocument {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub excerpt: String,
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
        let writer = index.writer(50_000_000)?;
        //索引读取器
        let reader = index
            .reader_builder()
            //每次提交后延迟
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()?;
        let id = index.schema().get_field("id")?;
        let title = index.schema().get_field("title")?;
        let content = index.schema().get_field("content")?;
        let excerpt = index.schema().get_field("excerpt")?;
        Ok(SearchEngine {
            index,
            reader,
            writer: Mutex::new(writer),
            my_doc: MyDocument {
                id,
                title,
                content,
                excerpt,
            },
        })
    }

    pub fn search(
        &self,
        query: &str,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<String>, SearchEngineError> {
        let searcher = self.reader.searcher();
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

    ///批量插入
    pub fn insert_batch(&self, posts: Vec<PostDocument>) -> Result<(), SearchEngineError> {
        if let Ok(mut writer) = self.writer.try_lock() {
            for post in posts.into_iter() {
                writer.add_document(doc!(
                      self.my_doc.id=>post.id,
                     self.my_doc.title=>post.title,
                     self.my_doc.content=>post.content,
                     self.my_doc.excerpt=>post.excerpt
                ))?;
            }
            writer.commit()?;
            Ok(())
        } else {
            Err(SearchEngineError::TryLockError)
        }
    }

    ///更新 = 删除 + 插入
    pub fn update(
        &self,
        id: u64,
        title: String,
        content: String,
        excerpt: Option<String>,
    ) -> Result<(), SearchEngineError> {
        if let Ok(mut writer) = self.writer.try_lock() {
            writer.delete_term(Term::from_field_u64(self.my_doc.id, id));
            writer.add_document(doc!(
                  self.my_doc.id=>id,
                 self.my_doc.title=>title,
                 self.my_doc.content=>content,
                 self.my_doc.excerpt=>excerpt.unwrap_or_default()
            ))?;
            writer.commit()?;
            Ok(())
        } else {
            Err(SearchEngineError::TryLockError)
        }
    }
}
