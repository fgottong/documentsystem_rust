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

    fn handle_connection(&self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);

        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|lines| !lines.is_empty())
            .collect();

        println!("Request:{:#?}", http_request);

        let json = format!(r#"{}"#, self.reading_list.to_json());

        //         let json = r#"
        // {
        //   "books": [
        //     "Journey to the Center of the Earth",
        //     "Twenty Thousand Leagues Under the Sea",
        //     "Around the World in Eighty Days"
        //   ]
        // }"#;

        println!("Content Ist : {}\n\n", json);

        let status_line = "HTTP/1.1 200 OK";
        let cors_allowed = "Access-Control-Allow-Origin: http://localhost:5173";
        let cont_type = "Content-Type: application/json; charset=utf-8";
        let length = json.len();

        let response = format!(
            "{}
{}
{}
Content-Length: {}

{}",
            status_line, cors_allowed, cont_type, length, json
        );

        stream.write_all(&response.as_bytes()).unwrap();
        println!("Habe Antwort gesendet:\n{}", response)
    }
}

// get /reading_list

// Regist multiple Endpoints to server
