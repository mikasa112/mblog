use std::path::Path;
use crate::internal::core::my_error::SearchEngineError;
use std::sync::LazyLock;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::{IndexRecordOption, Schema, TextFieldIndexing, TextOptions, INDEXED, STORED};
use tantivy::tokenizer::{LowerCaser, RemoveLongFilter, Stemmer, TextAnalyzer};
use tantivy::{Document, Index, ReloadPolicy, TantivyDocument};
use tantivy::directory::MmapDirectory;
use crate::internal::core::config::BLOG_CONFIG;

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
        let options = TextOptions::default()
            .set_indexing_options(
                TextFieldIndexing::default()
                    .set_tokenizer("jieba")
                    .set_index_option(IndexRecordOption::WithFreqsAndPositions),
            );
            // .set_stored();
        builder.add_u64_field("id", INDEXED | STORED);
        builder.add_text_field("title", options.clone() | STORED);
        builder.add_text_field("content", options.clone());
        builder.add_text_field("excerpt", options);
        let schema = builder.build();
        // let index = Index::open_or_create(MmapDirectory::open(Path::new(BLOG_CONFIG.application.search_engine_dir.as_str()))?, schema.clone())?;
        let index = Index::create_in_ram(schema.clone());
        let tokenizer = tantivy_jieba::JiebaTokenizer {};
        let analyzer = TextAnalyzer::builder(tokenizer)
            .filter(RemoveLongFilter::limit(40))
            .filter(LowerCaser)
            .filter(Stemmer::default())
            .build();
        index.tokenizers().register("jieba", analyzer);


        Ok(SearchEngine { index })
    }

    pub fn search(
        &self,
        query: &str,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<String>, SearchEngineError> {
        let reader = self
            .index
            .reader_builder()
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

// mod test {
//     #[test]
//     fn test_jieba() {
//         use tantivy::tokenizer::*;
//         let mut tokenizer = tantivy_jieba::JiebaTokenizer {};
//         let mut token_stream = tokenizer.token_stream("18测试标题测试标题18");
//         while token_stream.advance() {
//             let token_string = token_stream.next().unwrap().text.clone();
//             println!("Token: {}", token_string);
//         }
//     }
// }
