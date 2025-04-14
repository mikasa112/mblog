##### sqlx 使用所需环境
```shell
#安装SQLX的预处理器
cargo install sqlx-cli
#加载`DATABASE_URL`环境变量，或者在项目根目录创建.env进行键入配置
$env:DATABASE_URL="mysql://username:password@host/database"
#预处理query!宏，如果在.env中设置了环境变量，那么就不需要手动执行此命令
cargo sqlx prepare
```
##### tantivy 搜索引擎使用总结
目前的项目数据量很小很小，所以搜索引擎创建的索引都在内存里面，每次运行程序都会重新加载
```rust
let mut builder = Schema::builder();
builder.add_text_field("title", TEXT| STORED);
let index = Index::create_in_ram(schema.clone());
```
如果需要持久化，需要考虑数据同步和重复创建的问题
```rust
let index = Index::open_or_create(MmapDirectory::open(Path::new(BLOG_CONFIG.application.search_engine_dir.as_str()))?, schema.clone())?;
```