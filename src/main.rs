mod routes;
use crate::routes::index;

use actix_web::{App, HttpServer};

extern crate serde;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct ResponseData {
  message: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| (App::new().service(index::root).service(index::again)))
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
