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
    pub master: AccountIndex,
    
    pub commodity_pool: CommodityPool,
    pub xacts: Vec<Xact>,
    pub posts: Vec<Post>,
    pub accounts: Vec<Account>,

    // key, account index
    pub accounts_map: HashMap<String, AccountIndex>,
}

impl Journal {
    pub fn new() -> Self {
        let mut j = Journal {
            master: 0,

            commodity_pool: CommodityPool::new(),
            xacts: vec![],
            posts: vec![],
            accounts: vec![],
            accounts_map: HashMap::new(),

            // sources: Vec<fileinfo?>
        };

        // Add master account
        j.add_account(Account::new("master"));

        j
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

    pub fn get_account(&self, index: AccountIndex) -> &Account {
        &self.accounts[index]
    }

    /// A convenience method that returns a vector of Account references
    /// for a given vector of indices, ie from Posts.
    // pub fn get_accounts(&self, account_indices: &Vec<AccountIndex>) -> Vec<&Account> {

    // }

    pub fn get_posts(&self, indices: &Vec<PostIndex>) -> Vec<&Post> {
        indices.iter().map(|i| &self.posts[*i]).collect()
    }

    pub fn get_master_account(&self) -> &Account {
        self.accounts.get(0).expect("master account")
    }

    pub fn get_master_account_mut(&mut self) -> &mut Account {
        self.accounts.get_mut(0).expect("master account")
    }

    pub fn get_xact_posts(&self, index: XactIndex) -> Vec<&Post> {
        let xact = &self.xacts[index];
        self.get_posts(&xact.posts)
    }

    pub fn register_account(&mut self, name: &str) -> Option<AccountIndex> {
        // todo: expand_aliases

        // let master = self.get_master_account_mut();
        // let account_index = master.find_account(name, self);
        
        // let account_index = self.find_account(name, true);
        let account_index = self.find_account(name, true, 0);

        // todo: add any validity checks here.

        account_index
    }

    /// Create an account tree from the account full-name.
    pub fn find_account(&mut self, acct_name: &str, auto_create: bool, parent_id: AccountIndex) -> Option<AccountIndex> {
        let parent = self.accounts.get(parent_id).unwrap();
        if parent.accounts.contains_key(acct_name) {
            return Some(*parent.accounts.get(acct_name).unwrap());
        }

        // if not found, try to break down
        let first: &str;
        let rest: &str;
        if let Some(separator_index) = acct_name.find(':') {
            // Contains separators
            first = &acct_name[..separator_index];
            rest = &acct_name[separator_index + 1..];
        } else {
            // take all
            first = acct_name;
            rest = "";
        }

        let mut account_index: AccountIndex;
        if !parent.accounts.contains_key(first) {
            // create and add to the store.
            account_index = self.add_account(Account::new(first));

            // Add to local map
            let root_mut = self.accounts.get_mut(parent_id).unwrap();
            root_mut.accounts.insert(first.into(), account_index);
        } else {
            account_index = *parent.accounts.get(first).unwrap();
        }

        // Search recursively.
        if !rest.is_empty() {
            account_index = self.find_account(rest, auto_create, account_index).unwrap()
        }

        Some(account_index)
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
        assert_eq!(4, journal.accounts.len());
        assert_eq!(Some(3), actual);

        // tree structure
        let master = journal.get_master_account();
        assert_eq!("master", master.name);

        let assets_id = master.accounts.get("Assets").unwrap();
        let assets = journal.get_account(*assets_id);
        assert_eq!("Assets", assets.name);

        let inv_ix = assets.get_account("Investments").unwrap();
        let inv = journal.get_account(inv_ix);
        assert_eq!("Investments", inv.name);

        let broker_ix = inv.get_account("Broker").unwrap();
        let broker = journal.get_account(broker_ix);
        assert_eq!("Broker", broker.name);
    }

    /// The master account needs to be created in the Journal automatically.
    #[test]
    fn test_master_gets_created() {
        let j = Journal::new();

        let actual = j.get_master_account();

        assert_eq!("master", actual.name);
    }

    #[test]
    fn test_find_account() {
        let name = "Assets:Investments:Broker";
        let mut journal = Journal::new();
        
        let actual = journal.find_account(name, true, 0);

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