##### sqlx 使用所需环境
```shell
#安装SQLX的预处理器
Cargo isntall sqlx-cli
#加载`DATABASE_URL`环境变量
$env:DATABASE_URL="mysql://root:mikasa520.@122.51.163.61/d_blog"
#预处理query!宏
cargo sqlx prepare
```