use crate::xact::Xact;

/**
 * Journal
 */

pub struct Journal {
    // pub master: Account,
    pub xacts: Vec<Xact>,
}

impl Journal {
    pub fn new() -> Self {
        Journal {
            // master: Account::new(),
            xacts: vec![],
            // current_context: ,
        }
    }
}