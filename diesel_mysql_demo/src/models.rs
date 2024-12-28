use diesel::prelude::*;

#[derive(Queryable)] // generate all of the code needed to load a User struct from a SQL query
#[derive(Selectable)]  // generate code to construct a matching select clause based on your model type
#[diesel(table_name = crate::schema::posts)]  // 指定表名,这里是 crate::schema::posts，和表定义关联起来
#[diesel(check_for_backend(diesel::mysql::Mysql))]  // 检查字段是否符合定义
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

use crate::schema::posts;

#[derive(Insertable)]
#[diesel(table_name = posts)]  // 指定表名
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}