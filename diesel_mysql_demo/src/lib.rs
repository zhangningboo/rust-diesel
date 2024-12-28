use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

/// Establishes a database connection using the `DATABASE_URL` environment
/// variable. The value of this variable should be a valid connection string
/// for a MySQL database.
///
/// Panics if a database connection cannot be established.
pub fn establish_connection() -> MysqlConnection {
    // Load environment variables from a `.env` file
    dotenv().ok();

    // Get the database connection string from the environment
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // Attempt to establish a connection to the database
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| {
            // If the connection attempt fails, panic with an error message
            panic!("Error connecting to {}", database_url)
        })
}