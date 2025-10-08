use crate::document::Document;
pub mod document;

fn main() {
    
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


    for book in &reading_list{
        // println!("{:?}",book);
        // println!("{}",book);

        let json = serde_json::to_string(&book).unwrap();
        println!("{}",json)

    }

}




// struct author{
//     // single author: name, surname, academic title, 
//}