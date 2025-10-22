//use crate::document::Document;
pub mod document;

use crate::readinglist::ReadingList;
pub mod readinglist;

use crate::server::Server;
pub mod server;

use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let addr = "localhost:7878".to_string();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    let reading_list = ReadingList::create_reading_list();

    for book in &reading_list.list {
        let output = &book.to_string();
        println!("{}", output);
        println!();
    }

    // let server = Server::new(addr.clone(), reading_list);
    // println!("Starte Server on : {}", addr);
    // server.run();
}
