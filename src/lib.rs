#[macro_use]
extern crate diesel;

extern crate dotenv;
extern crate futures;
extern crate futures_cpupool;
extern crate protobuf;
extern crate grpc;
extern crate tls_api;

pub mod dots;
pub mod dots_grpc;
pub mod schema;
pub mod models;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use ::dots_grpc::*;

use models::NewRepo;

pub fn dots_connect() -> DotsClient {
    let client_conf = Default::default();
    let client = DotsClient::new_plain("::1", 10000, client_conf).unwrap();

    client
}

pub fn db_connect() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_repo(conn: &SqliteConnection, name: &str, url: &str) -> usize {
    use schema::repos;

    let new_repo = NewRepo {
        name: name,
        url: url,
    };

    diesel::insert_into(repos::table)
        .values(&new_repo)
        .execute(conn)
        .expect("Error saving new repo")
}
