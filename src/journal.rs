use crate::{xact::Xact, account::Account, post::Post};

/**
 * Journal
 */

pub struct Journal {
    // pub master: Account,
    
    pub xacts: Vec<Xact>,
    pub posts: Vec<Post>,
    pub accounts: Vec<Account>,
}

impl Journal {
    pub fn new() -> Self {
        Journal {
            // master: Account::new(),

            xacts: vec![],
            posts: vec![],
            accounts: vec![],

            // sources: Vec<fileinfo?>
        }
    }

    pub fn add_xact(&mut self, xact: Xact) -> usize {
        let i = self.xacts.len();
        self.xacts.push(xact);
        i
    }

    pub fn add_post(&mut self, post: Post) -> usize {
        let i = self.posts.len();
        self.posts.push(post);
        i
    }
}