use std::{
    io::{BufRead, BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

use crate::readinglist::ReadingList;

pub struct Server {
    pub listener: TcpListener,
    pub reading_list: ReadingList,
}

impl Server {
    pub fn new(_addr: String, _reading_list: ReadingList) -> Server {
        Server {
            listener: (TcpListener::bind(_addr).unwrap()),
            reading_list: _reading_list,
        }
    }

    pub fn run(&self) {
        //wait for connections
        //handle_connections -> Validate request ; answer -> responsed ;

        for stream in self.listener.incoming() {
            let _stream = stream.unwrap();
            println!("Connection Established");
            self.handle_connection(_stream);
        }
    }

    fn handle_connection(&self,mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);

        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|lines| !lines.is_empty())
            .collect();
       
        println!("Request:{:#?}", http_request);

        let json = self.reading_list.to_json();

        let status_line = "HTTP/1.1 200 OK";
        let content = json;
        let length = content.len();

        let response = format!(
            "{status_line}\r\n
            Content-Length: {length}\r\n\r\n
            {content}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    }

    
}
