use crate::document::Document;

pub struct ReadingList {
    pub list: Vec<Document>,
}

impl ReadingList {
    pub fn create_reading_list() -> Self {
        let book1 = Document {
            title: String::from("20000 Meilen unter dem Meer"),
            author: String::from("Jules Vernes"),
            content: String::from(
                "STory about an famous Qcean-Scienties and the tragedy of Captn Nemo",
            ),
        };

        let book2: Document = Document {
            title: String::from("Der Seewolf"),
            author: String::from("Jack London"),
            content: String::from(
                "The philosophic telling of the dramatic journey of an Journalist on an Whale-Hunter in the late 1800s",
            ),
        };

        let book3 = Document {
            title: String::from("The Rust Programming Language"),
            author: String::from("Steve Klabnik, Carol Nichols"),
            content: String::from("The Standard Literature for the rust language"),
        };

        let mut _reading_list: Vec<Document> = Vec::new();
        _reading_list.push(book3);
        _reading_list.push(book2);
        _reading_list.push(book1);

        return ReadingList {
            list: (_reading_list),
        };
    }

    pub fn to_json(&self) -> String {
        let json = serde_json::to_string(&self.list).unwrap();
        return json;
    }
}
