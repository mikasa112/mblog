use crate::internal::core::config::BLOG_CONFIG;
use crate::internal::core::my_error::SearchEngineError;
use std::path::Path;
use std::sync::LazyLock;
use tantivy::collector::TopDocs;
use tantivy::directory::MmapDirectory;
use tantivy::query::QueryParser;
use tantivy::schema::{Schema, INDEXED, STORED, TEXT};
use tantivy::{Document, Index, ReloadPolicy, Searcher, TantivyDocument};

//延迟初始化
pub static SEARCH_ENGINE: LazyLock<Result<SearchEngine, SearchEngineError>> = LazyLock::new(|| {
    SearchEngine::new()
});


///搜索引擎
pub struct SearchEngine {
    pub schema: Schema,
    pub index: Index,
    searcher: Searcher,
    query_parser: QueryParser,
}

impl SearchEngine {
    pub fn new() -> Result<Self, SearchEngineError> {
        let mut builder = Schema::builder();
        builder.add_u64_field("id", INDEXED | STORED);
        builder.add_text_field("title", TEXT | STORED);
        builder.add_text_field("content", TEXT);
        builder.add_text_field("excerpt", TEXT);
        let schema = builder.build();
        let index = Index::open_or_create(MmapDirectory::open(Path::new(BLOG_CONFIG.application.search_engine_dir.as_str()))?, schema.clone())?;
        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()?;
        let searcher = reader.searcher();
        let title = schema.get_field("title")?;
        let content = schema.get_field("content")?;
        let excerpt = schema.get_field("excerpt")?;
        let query_parser = QueryParser::for_index(&index, vec![title, content, excerpt]);
        Ok(SearchEngine {
            schema,
            index,
            searcher,
            query_parser,
        })
    }

    pub fn search(&self, query: &str, limit: usize, offset: usize) -> Result<Vec<String>, SearchEngineError> {
        let query = self.query_parser.parse_query(query)?;
        let top_docs = self.searcher.search(&query, &TopDocs::with_limit(limit).and_offset(offset))?;
        let mut list = Vec::new();
        for (_score, doc_address) in top_docs {
            let retrieved_doc: TantivyDocument = self.searcher.doc(doc_address)?;
            list.push(retrieved_doc.to_json(&self.schema))
        }
        Ok(list)
    }
}