//use crate::document::Document;
pub mod document;

use crate::readinglist::ReadingList;
pub mod readinglist;

use crate::server::Server;
pub mod server;

fn main() {
    let reading_list = ReadingList::create_reading_list();

    for book in &reading_list.list {
        let output = &book.to_string();
        println!("{}", output);
        println!();
    }

    let server = Server::new("127.0.0.1:7878".to_string(), reading_list);
    println!("Starte Server");
    server.run();

}
