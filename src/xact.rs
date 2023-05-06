use chrono::NaiveDate;

use crate::post::Post;

pub struct Xact {
    pub date: Option<NaiveDate>,
    pub payee: String,
    pub posts: Vec<Post>,
    pub note: Option<String>, 
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

    pub fn add_post(&mut self, post: Post) {
        self.posts.push(post);
    }

}