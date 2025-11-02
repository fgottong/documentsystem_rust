use crate::category::Category;

pub struct DocumentSystem {
    cats: Vec<Category>,
}

impl DocumentSystem {
    pub fn new() -> Self {
        let mut _cats: Vec<Category> = Vec::new();
        return DocumentSystem { cats: _cats };
    }

    pub fn add_category(&mut self, new_cat: Category) {
        let _cats: &mut Vec<Category> = &mut self.cats;
        _cats.push(new_cat);
    }
}
