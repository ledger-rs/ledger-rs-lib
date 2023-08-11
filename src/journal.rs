/*!
 * Journal
 * The main model object. The journal files are parsed into the Journal structure.
 * Provides methods for fetching and iterating over the contained elements
 * (transactions, posts, accounts...).
 */
use std::io::Read;

use crate::{
    account::Account,
    commodity::Commodity,
    parser,
    pool::{CommodityIndex, CommodityPool},
    post::Post,
    xact::Xact,
};

// pub type XactIndex = usize;

pub struct Journal {
    pub master: Account,

    pub commodity_pool: CommodityPool,
    pub xacts: Vec<Xact>,
}

impl Journal {
    pub fn new() -> Self {
        Self {
            master: Account::new(""),

            commodity_pool: CommodityPool::new(),
            xacts: vec![],
            // sources: Vec<fileinfo?>
        }
    }

    pub fn add_xact(&mut self, xact: Xact) -> &Xact {
        self.xacts.push(xact);
        //self.xacts.len() - 1
        self.xacts.last().unwrap()
    }

    pub fn all_posts(&self) -> Vec<&Post> {
        self.xacts.iter().flat_map(|x| x.posts.iter()).collect()
    }

    pub fn get_account(&self, acct_ptr: *const Account) -> &Account {
        unsafe { &*acct_ptr }
    }

    pub fn get_account_mut(&self, acct_ptr: *const Account) -> &mut Account {
        unsafe {
            let mut_ptr = acct_ptr as *mut Account;
            &mut *mut_ptr
        }
    }

    pub fn get_commodity(&self, index: CommodityIndex) -> &Commodity {
        self.commodity_pool.get_by_index(index)
    }

    /// Called to create an account during Post parsing.
    ///
    /// account_t * journal_t::register_account(const string& name, post_t * post,
    ///                                         account_t * master_account)
    ///
    pub fn register_account(&mut self, name: &str) -> Option<*const Account> {
        if name.is_empty() {
            panic!("Invalid account name {:?}", name);
        }

        // todo: expand_aliases
        // account_t * result = expand_aliases(name);

        let master_account: &mut Account = &mut self.master;

        // Create the account object and associate it with the journal; this
        // is registering the account.

        let Some(account_ptr) = master_account.find_account(name)
        else { return None };

        // todo: add any validity checks here.

        let account = self.get_account(account_ptr);
        Some(account)
    }

    pub fn find_account(&mut self, name: &str) -> Option<*const Account> {
        self.master.find_account(name)
    }

    /// Read journal source (file or string).
    ///
    /// std::size_t journal_t::read(parse_context_stack_t& context)
    ///
    /// returns number of transactions parsed
    pub fn read<T: Read>(&mut self, source: T) -> usize {
        // read_textual
        parser::read_into_journal(source, self);

        self.xacts.len()
    }
}

#[cfg(test)]
mod tests {
    use core::panic;
    use std::io::Cursor;

    use super::Journal;
    use crate::{account::Account, parse_file};

    #[test]
    fn test_add_account() {
        const ACCT_NAME: &str = "Assets";
        let mut journal = Journal::new();
        let ptr = journal.register_account(ACCT_NAME).unwrap();
        let actual = journal.get_account(ptr);

        // There is master account
        // assert_eq!(1, i);
        assert_eq!(ACCT_NAME, actual.name);
    }

    #[test]
    fn test_add_account_to_master() {
        let mut journal = Journal::new();
        const NAME: &str = "Assets";

        let Some(ptr) = journal.register_account(NAME) else {panic!("unexpected")};
        let actual = journal.get_account(ptr);

        assert_eq!(&journal.master as *const Account, actual.parent);
    }

    #[test]
    fn test_find_account() {
        let mut journal = Journal::new();
        parse_file("tests/basic.ledger", &mut journal);

        let actual = journal.find_account("Assets:Cash");

        assert!(actual.is_some());
    }

    #[test]
    fn test_register_account() {
        const NAME: &str = "Assets:Investments:Broker";
        let mut journal = Journal::new();

        // act
        let new_acct = journal.register_account(NAME).unwrap();
        let actual = journal.get_account_mut(new_acct);

        // Asserts
        assert_eq!(4, journal.master.flatten_account_tree().len());
        assert_eq!(NAME, actual.fullname());

        // tree structure
        let master = &mut journal.master;
        assert_eq!("", master.name);

        let assets_ptr = master.find_account("Assets").unwrap();
        let assets = journal.get_account_mut(assets_ptr);
        assert_eq!("Assets", assets.name);
        assert_eq!(&journal.master as *const Account, assets.parent);

        let inv_ix = assets.find_account("Investments").unwrap();
        let inv = journal.get_account_mut(inv_ix);
        assert_eq!("Investments", inv.name);
        assert_eq!(assets_ptr, inv.parent);

        let broker_ix = inv.find_account("Broker").unwrap();
        let broker = journal.get_account(broker_ix);
        assert_eq!("Broker", broker.name);
        assert_eq!(inv_ix, broker.parent);
    }

    /// The master account needs to be created in the Journal automatically.
    #[test]
    fn test_master_gets_created() {
        let j = Journal::new();

        let actual = j.master;

        assert_eq!("", actual.name);
    }

    #[test]
    fn test_read() {
        let src = r#"2023-05-01 Test
  Expenses:Food  20 EUR
  Assets:Cash
"#;
        let mut j = Journal::new();

        // Act
        let num_xact = j.read(Cursor::new(src));

        // Assert
        assert_eq!(1, num_xact);
    }
}
