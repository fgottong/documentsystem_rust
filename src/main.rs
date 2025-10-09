use crate::document::Document;
pub mod document;

use std::{
    fs,
    io::{prelude::*,BufRead, BufReader}, 
    net::{TcpListener, TcpStream}};

fn main() {
    
    let reading_list = create_reading_list();
    
    let json = get_reading_list_as_json(&reading_list);
    
    println!("{}",json);
    println!();
    for book in &reading_list{
        //let output = serde_json::to_string(&book).unwrap();
        let output = &book.to_string(); 
        println!("{}",output);
        println!();
    }

    let listener:TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap(); 
    println!("Starte Server");
    for stream in listener.incoming(){
        let _stream = stream.unwrap();
        println!("Connection Established");
        handle_connection(_stream,&json);
    }

}

fn create_reading_list()->Vec<Document>{
let book1 = Document{
        title: String::from("20000 Meilen unter dem Meer"),
        author: String::from("Jules Vernes"),
        content: String::from("STory about an famous Qcean-Scienties and the tragedy of Captn Nemo"),
    }; 

    let book2: Document = Document{
        title: String::from("Der Seewolf"),
        author: String::from("Jack London"),
        content: String::from("The philosophic telling of the dramatic journey of an Journalist on an Whale-Hunter in the late 1800s"),
    };

    let book3 = Document{ 
        title:String::from("The Rust Programming Language"),
        author:String::from("Steve Klabnik, Carol Nichols"), 
        content:String::from("The Standard Literature for the rust language"),
    } ;

    let mut reading_list : Vec<Document>= Vec::new(); 
    reading_list.push(book3);
    reading_list.push(book2);
    reading_list.push(book1);

    return reading_list;

}

fn get_reading_list_as_json(reading_list:&Vec<Document> )->String{
    let json = serde_json::to_string(&reading_list).unwrap();
    return json;
}


fn handle_connection(mut stream: TcpStream,json: &String){
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result|result.unwrap())
        .take_while(|lines|!lines.is_empty())
        .collect();
    println!("Request:{:#?}",http_request);


    let status_line = "HTTP/1.1 200 OK";
    let content = json;
    let length = content.len();

    let response = format!(
        "{status_line}\r\n
        Content-Length: {length}\r\n\r\n
        {content}");

        stream.write_all(response.as_bytes()).unwrap();


        
}

// struct author{
//     // single author: name, surname, academic title, 
//}