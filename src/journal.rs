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

    pub fn add_xact(&mut self, xact: Xact) {
        // todo: xact.journal =
        
        xact.finalize();

        // todo: extend_xact()
        // todo: check_all_metadata())
        // todo: for each post - extend + check metadata

        self.xacts.push(xact);
    }
}