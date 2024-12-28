use diesel::prelude::*;
use diesel_mysql_demo::*;
use std::io::{stdin, Read};
// pub mod self::models;
// pub mod self::schema;

use self::models::{NewPost, Post};


pub fn create_post(conn: &mut MysqlConnection, title: &str, body: &str) -> Post {
    use self::schema::posts;

    let new_post = NewPost { title, body };

    conn.transaction(|conn| {
        diesel::insert_into(posts::table)
            .values(&new_post)
            .execute(conn)?;

        posts::table
            .order(posts::id.desc())
            .select(Post::as_select())
            .first(conn)
    })
    .expect("Error while saving post")
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";

fn main() {
    let connection = &mut establish_connection();

    let mut title = String::new();
    let mut body = String::new();

    println!("What would you like your title to be?");
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end(); // Remove the trailing newline

    println!("\nOk! Let's write {title} (Press {EOF} when finished)\n",);
    stdin().read_to_string(&mut body).unwrap();

    let post = create_post(connection, title, &body);
    println!("\nSaved draft {title} with id {}", post.id);
}
