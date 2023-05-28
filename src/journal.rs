use std::collections::HashMap;

use crate::{xact::Xact, account::Account, post::Post, pool::CommodityPool};

/**
 * Journal
 */

pub type AccountIndex = usize;
// pub type CommodityIndex = usize;
pub type PostIndex = usize;
pub type XactIndex = usize;

pub struct Journal {
    // pub master: Account,
    
    // pub commodities: Vec<Commodity>,
    pub commodity_pool: CommodityPool,
    pub xacts: Vec<Xact>,
    pub posts: Vec<Post>,
    pub accounts: Vec<Account>,
    // key, account index
    pub accounts_map: HashMap<String, usize>,
}

impl Journal {
    pub fn new() -> Self {
        Journal {
            // master: Account::new(),

            commodity_pool: CommodityPool::new(),
            // commodities: vec![],
            xacts: vec![],
            posts: vec![],
            accounts: vec![],
            accounts_map: HashMap::new(),

            // sources: Vec<fileinfo?>
        }
    }

    pub fn add_account(&mut self, acct: Account) -> AccountIndex {
        self.accounts.push(acct);
        self.accounts.len() - 1
    }

    pub fn add_xact(&mut self, xact: Xact) -> XactIndex {
        self.xacts.push(xact);
        self.xacts.len() - 1
    }

    pub fn add_post(&mut self, post: Post) -> PostIndex {
        let i = self.posts.len();
        self.posts.push(post);
        i
    }

    pub fn get_account(&self, index: usize) -> &Account {
        &self.accounts[index]
    }

    /// A convenience method that returns a vector of Account references
    /// for a given vector of indices, ie from Posts.
    // pub fn get_accounts(&self, account_indices: &Vec<AccountIndex>) -> Vec<&Account> {

    // }

    pub fn get_posts(&self, indices: &Vec<PostIndex>) -> Vec<&Post> {
        indices.iter().map(|i| &self.posts[*i]).collect()
    }

    pub fn get_xact_posts(&self, index: XactIndex) -> Vec<&Post> {
        let xact = &self.xacts[index];
        self.get_posts(&xact.posts)
    }

    pub fn register_account(&mut self, name: &str) -> Option<usize> {
        let account_index = self.find_account(name, true);

        // todo: add any validity checks here.

        account_index
    }

    /// Create an account tree from the account full-name.
    pub fn find_account(&mut self, full_account_name: &str, auto_create: bool) -> Option<usize> {
        let mut has_account = self.accounts_map.get(full_account_name);
        if has_account.is_some() {
            return has_account.cloned();
        }
        
        let mut account_index: Option<usize> = None;
        let mut parent: Option<usize> = None;

        for part in full_account_name.split(':') {
            has_account = self.accounts_map.get(part);

            if has_account.is_none() {
                if !auto_create {
                    return None;
                }

                let mut new_account = Account::new(part);
                if parent.is_some() {
                    new_account.parent_index = account_index;
                }

                account_index = Some(self.add_account(new_account));
                self.accounts_map.insert(part.to_owned(), account_index.unwrap());

                parent = account_index;
            }
        }
        
        account_index
    }
}

#[cfg(test)]
mod tests {
    use crate::{account::Account, post::Post};
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

    #[test]
    fn test_getting_multiple_posts() {
        let mut journal = Journal::new();
        let p1 = Post::new(10, 11, None, None);
        let i1 = journal.add_post(p1);
        let p2 = Post::new(20, 11, None, None);
        let i2 = journal.add_post(p2);

        let actual = journal.get_posts(&vec![i1, i2]);

        assert_eq!(2, actual.len());
        assert_eq!(10, actual[0].account_index);
        assert_eq!(20, actual[1].account_index);
    }

    #[test]
    fn test_register_account() {
        let name = "Assets:Investments:Broker";
        let mut journal = Journal::new();

        let actual = journal.register_account(name);

        // Asserts
        assert_eq!(3, journal.accounts_map.len());
        assert_eq!(Some(2), actual);
    }

    #[test]
    fn test_find_account() {
        let name = "Assets:Investments:Broker";
        let mut journal = Journal::new();
        
        let actual = journal.find_account(name, true);

        // Assert

        assert_eq!(3, journal.accounts_map.len());

        let mut index = *journal.accounts_map.get("Assets").unwrap();
        let account = journal.get_account(index);
        assert_eq!("Assets", account.name);

        index = *journal.accounts_map.get("Investments").unwrap();
        let assets_account = journal.get_account(index);
        assert_eq!("Investments", assets_account.name);

        index = *journal.accounts_map.get("Broker").unwrap();
        let journal_account = journal.get_account(index);
        assert_eq!("Broker", journal_account.name);

        assert!(actual.is_some());
        assert_eq!(2, actual.unwrap());
    }
}