```shell
$ cargo new --lib diesel_mysql_demo
$ cd diesel_mysql_demo
$ cargo add diesel
$ cargo add mysqlclient-sys dotenvy
$ echo DATABASE_URL=mysql://root:my-secret-pw@172.17.0.3:3306/diesel_demo > .env
$ diesel setup
$ diesel migration generate create_posts
$ # 填写sql语句在相应的up.sql / down.sql
$ diesel migration run  # 执行up.sql文件中的语句
$ diesel migration redo  # 执行down.sql文件中的语句

# Cargo.toml
[[bin]]
name = "query_posts"
path = "src/bin/query_posts.rs"
doc = false
...

$ cargo run --bin query_posts
$ cargo run --bin insert_posts
$ cargo run --bin update_posts 1
$ cargo run --bin query_by_id 1
$ cargo run --bin delete_posts haha
```