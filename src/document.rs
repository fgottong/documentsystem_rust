use serde::{Deserialize, Serialize};
use core::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Document{
    pub title: String, 
    pub author: String,
    pub content: String,
    // created_date: date,
}


impl fmt::Display for Document {
    fn fmt(&self,f:&mut fmt::Formatter)-> fmt::Result{
        write!(f, "({} - {}; {})", self.author, self.title, self.content)
    } // Display-Trait -> Automatically implements to_string function// ToString Trait ! 
}
