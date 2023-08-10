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

pub type XactIndex = usize;

pub struct Journal {
    pub master: *const Account,

    pub commodity_pool: CommodityPool,
    pub xacts: Vec<Xact>,
    pub accounts: Vec<Account>
}

impl Journal {
    pub fn new() -> Self {
        let mut j = Journal {
            master: std::ptr::null(),

            commodity_pool: CommodityPool::new(),
            xacts: vec![],
            // posts: vec![],
            accounts: vec![],
            // sources: Vec<fileinfo?>
        };

        // Create master account
        let master = j.add_account(Account::new(""));
        j.master = master;

        j
    }

    /// Adds the account to the storage.
    /// Should be used only during account registration.
    fn add_account(&mut self, acct: Account) -> &Account {
        self.accounts.push(acct);
        // self.accounts.len() - 1
        self.accounts.last().unwrap()
    }

    pub fn add_xact(&mut self, xact: Xact) -> &Xact {
        self.xacts.push(xact);
        //self.xacts.len() - 1
        self.xacts.last().unwrap()
    }

    pub fn all_posts(&self) -> Vec<&Post> {
        self.xacts.iter().flat_map(|x| x.posts.iter()).collect()
    }

    pub fn create_account(&mut self, name: &str) -> *const Account {
        let acct = Account::new(name);
        let ptr = &acct as *const Account;
        self.add_account(acct);
        ptr
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

    pub fn get_master_account(&self) -> &Account {
        self.accounts.get(0).expect("master account")
    }

    pub fn get_master_account_mut(&mut self) -> &mut Account {
        self.accounts.get_mut(0).expect("master account")
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

        let master_account: &mut Account = self.get_master_account_mut();

        // Create the account object and associate it with the journal; this
        // is registering the account.

        // let account_ptr = self.create_sub_account(self.master, name, true);

        let account = master_account.find_account(name);

        // todo: add any validity checks here.

        account
    }

    pub fn find_account(&self, name: &str) -> Option<&Account> {
        let Some(ptr) = self.find_account(name)
        else {return None};

        Some(self.get_account(ptr))
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
    use std::io::Cursor;

    use super::Journal;
    use crate::{account::Account, parse_file};

    #[test]
    fn test_add_account() {
        const ACCT_NAME: &str = "Assets";
        let mut journal = Journal::new();
        let a = Account::new(ACCT_NAME);
        let actual = journal.add_account(a);

        // There is master account
        // assert_eq!(1, i);
        assert_eq!(ACCT_NAME, actual.name);
    }

    #[test]
    fn test_add_account_data() {
        let mut journal = Journal::new();
        let a = Account::new("Assets");
        let expected = Account::new("Assets");

        let actual = journal.add_account(a);

        assert_eq!(expected, *actual);
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
        let actual = journal.get_account(new_acct);

        // Asserts
        assert_eq!(4, journal.accounts.len());
        assert_eq!(NAME, actual.fullname());

        // tree structure
        let master = journal.get_master_account_mut();
        assert_eq!("", master.name);

        let assets_ptr = master.find_account("Assets").unwrap();
        let assets = journal.get_account_mut(assets_ptr);
        assert_eq!("Assets", assets.name);
        assert_eq!(journal.master, assets.parent);

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

        let actual = j.get_master_account();

        assert_eq!("", actual.name);
        assert_ne!(std::ptr::null(), actual);
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
