use crate::{xact::Xact, account::Account, post::Post, commodity::Commodity};

/**
 * Journal
 */

pub type CommodityIndex = usize;

pub struct Journal {
    // pub master: Account,
    
    pub commodities: Vec<Commodity>,
    pub xacts: Vec<Xact>,
    pub posts: Vec<Post>,
    pub accounts: Vec<Account>,
}

impl Journal {
    pub fn new() -> Self {
        Journal {
            // master: Account::new(),

            commodities: vec![],
            xacts: vec![],
            posts: vec![],
            accounts: vec![],

            // sources: Vec<fileinfo?>
        }
    }

    pub fn add_account(&mut self, acct: Account) -> usize {
        self.accounts.push(acct);
        self.accounts.len() - 1
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

    pub fn get_account(&self, index: usize) -> &Account {
        &self.accounts[index]
    }

    pub fn get_commodity(&self, index: CommodityIndex) -> &Commodity {
        &self.commodities[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::account::Account;

    use super::Journal;

    #[test]
    fn test_add_account_index() {
        let mut journal = Journal::new();
        let a = Account::new("Assets");
        let i = journal.add_account(a);

        assert_eq!(0, i);
    }

    #[test]
    fn test_add_account_data() {
        let mut journal = Journal::new();
        let a = Account::new("Assets");
        let expected = Account::new("Assets");
        let index = journal.add_account(a);

        let actual = journal.get_account(index);
        
        assert_eq!(expected, *actual);
    }
}