```shell
$ cargo new --lib diesel_postgresql_demo
$ echo DATABASE_URL=postgres://postgres:D1UDf3@localhost:10060/diesel_demo > .env
$ diesel setup
$ diesel migration generate create_posts
$ diesel migration run
$ diesel migration redo
$
$
$
$
$
$
```