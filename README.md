# MySQL

```shell
$ docker run --name ai-mysql -p 3306:3306 -e MYSQL_ROOT_PASSWORD=my-secret-pw -d registry.cn-hangzhou.aliyuncs.com/zhangningboo_docker/arm64_mysql:9.1.0

$ apt install libncurses-dev
$ apt install libmysqlclient21 librust-mysqlclient-sys-dev libmysqlclient-dev default-libmysqlclient-dev
$ wget https://www.sqlite.org/2024/sqlite-autoconf-3470200.tar.gz
$ apt install librust-pq-sys-dev libpqxx-dev libpq-dev
$ curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.sh | sh
# or
$ cargo binstall diesel_cli
# or
$ cargo install diesel_cli --no-default-features --features mysql
```

```shell
$ cargo new --lib diesel_postgresql_demo
$ echo DATABASE_URL=mysql://root:my-secret-pw@172.17.0.3:3306/diesel_demo > .env
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