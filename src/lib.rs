#[macro_use] extern crate diesel;

extern crate dotenv;
extern crate futures;
extern crate futures_cpupool;
extern crate protobuf;
extern crate grpc;
extern crate tls_api;

// gRPC service
pub mod dots;
pub mod dots_grpc;

// Database
pub mod schema;
pub mod models;
pub mod database;
pub mod records;

use dots_grpc::*;

pub fn dots_connect() -> DotsClient {
    let client_conf = Default::default();
    let client = DotsClient::new_plain("::1", 10000, client_conf).unwrap();

    client
}
