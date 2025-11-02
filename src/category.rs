use crate::document::Document;
pub struct Category {
    pub subcats: Vec<Category>,
    pub docs: Vec<Document>,
}

impl Category {
    pub fn new() -> Self {
        let mut _docs: Vec<Document> = Vec::new();
        let mut _subcats: Vec<Category> = Vec::new();
        return Category {
            subcats: _subcats,
            docs: _docs,
        };
    }

    pub fn add_document(&mut self, new_doc: Document) {
        let docs = &mut self.docs;
        docs.push(new_doc);
    }

    pub fn add_category(&mut self, new_cat: Category) {
        let subcats: &mut Vec<Category> = &mut self.subcats;
        subcats.push(new_cat);
    }
}
