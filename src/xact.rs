use chrono::NaiveDate;

use crate::post::Post;

pub struct Xact {
    date: Option<NaiveDate>,
    pub posts: Vec<Post>,
    payee: String,
    note: Option<String>, 
}

impl Xact {
    pub fn new(date: Option<NaiveDate>, payee: String, note: Option<String>) -> Self {
        // code: Option<String>

        Self {
            payee,
            note,
            posts: vec![],
            date,
        }
    }
}