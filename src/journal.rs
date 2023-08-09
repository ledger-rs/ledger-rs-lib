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

pub type AccountIndex = usize;
pub type XactIndex = usize;

pub struct Journal {
    pub master: AccountIndex,

    pub commodity_pool: CommodityPool,
    pub xacts: Vec<Xact>,
    // pub posts: Vec<Post>,
    pub accounts: Vec<Account>,
}

impl Journal {
    pub fn new() -> Self {
        let mut j = Journal {
            master: 0,

            commodity_pool: CommodityPool::new(),
            xacts: vec![],
            // posts: vec![],
            accounts: vec![],
            // sources: Vec<fileinfo?>
        };

        // Add master account
        j.add_account(Account::new(""));

        j
    }

    /// Adds the account to the storage.
    /// Should be used only during account registration.
    fn add_account(&mut self, acct: Account) -> *const Account {
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
        self.xacts.iter().flat_map(|x| x.posts.iter())
        .collect()
    }

    pub fn create_account(&mut self, name: &str) -> *const Account {
        let acct = Account::new(name);
        let ptr = &acct as *const Account;
        self.add_account(acct);
        ptr
    }

    pub fn get_account(&self, acct_ptr: *const Account) -> &Account {
        unsafe {
            &*acct_ptr
        }
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

    /// 
    pub fn register_account(&mut self, name: &str) -> Option<*const Account> {
        if name.is_empty() {
            panic!("Invalid account name {:?}", name);
        }

        // todo: expand_aliases

        let account_ptr = self.create_sub_account(std::ptr::null(), name, true);

        // todo: add any validity checks here.

        account_ptr
    }

    pub fn find_account(&self, name: &str) -> Option<&Account> {
        let Some(index) = self.find_account_index(name)
        else {return None};

        Some(self.get_account(index))
    }

    pub fn find_account_index(&self, name: &str) -> Option<*const Account> {
        self.find_sub_account(std::ptr::null(), name)
    }

    /// Finds account by full name.
    /// i.e. "Assets:Cash"
    /// returns account index, if found
    pub fn find_sub_account(&self, parent_ptr: *const Account, name: &str) -> Option<*const Account> {
        //let parent = self.accounts.get(parent).unwrap();
        let parent = self.get_account(parent_ptr);
        if parent.accounts.contains_key(name) {
            return Some(*parent.accounts.get(name).unwrap());
        }

        let first: &str;
        let rest: &str;
        if let Some(separator_index) = name.find(':') {
            // Contains separators
            first = &name[..separator_index];
            rest = &name[separator_index + 1..];
        } else {
            // take all
            first = name;
            rest = "";
        }

        let mut account_ptr: Option<*const Account>;
        if !parent.accounts.contains_key(first) {
            return None;
        } else {
            account_ptr = Some(*parent.accounts.get(first).unwrap());
        }

        // Search recursively.
        if !rest.is_empty() {
            account_ptr = self
                .find_sub_account(account_ptr.unwrap(), rest);
        }

        account_ptr
    }

    /// Create an account tree from the account full-name.
    /// 
    /// In Ledger, this is
    /// account_t * account_t::find_account(
    /// but in order not to mix mutable and immutable access, the function is separated into
    /// find_account and create_account.
    fn create_sub_account(
        &mut self,
        root_ptr: *const Account,
        acct_name: &str,
        auto_create: bool,
    ) -> Option<*const Account> {
        // let parent = self.accounts.get(root_ptr).unwrap();
        let parent = self.get_account(root_ptr);
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

        let mut account_ptr: *const Account;
        if !parent.accounts.contains_key(first) {
            if !auto_create {
                return None;
            } // else

            // create and add to the store.
            let mut new_account = Account::new(first);
            new_account.parent = root_ptr;

            account_ptr = self.add_account(new_account);

            log::debug!("Created account {:?}, index {:?}", first, account_ptr);

            // Add to local map
            // let root_mut = self.accounts.get_mut(root_id).unwrap();
            let root_mut = self.get_account_mut(root_ptr);
            root_mut.accounts.insert(first.into(), account_ptr);
        } else {
            account_ptr = *parent.accounts.get(first).unwrap();
        }

        // Search recursively.
        if !rest.is_empty() {
            account_ptr = self
                .create_sub_account(account_ptr, rest, auto_create)
                .unwrap()
        }

        Some(account_ptr)
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
        let i = journal.add_account(a);
        let actual = journal.get_account(i);

        // There is master account
        // assert_eq!(1, i);
        assert_eq!(ACCT_NAME, actual.name);
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
    fn test_find_account() {
        let mut journal = Journal::new();
        parse_file("tests/basic.ledger", &mut journal);

        let actual = journal.find_account("Assets:Cash");

        assert!(actual.is_some());
    }

    #[test]
    fn test_register_account() {
        let name = "Assets:Investments:Broker";
        let mut journal = Journal::new();

        let acct_ptr = journal.register_account(name).unwrap();
        let actual = journal.get_account(acct_ptr);

        // Asserts
        assert_eq!(4, journal.accounts.len());
        assert_eq!(name, actual.fullname());

        // tree structure
        let master = journal.get_master_account();
        assert_eq!("", master.name);

        let assets_ptr = master.find_account("Assets").unwrap();
        let assets = journal.get_account(assets_ptr);
        assert_eq!("Assets", assets.name);
        // assert_eq!(Some(0), assets.parent_index);
        todo!("fix above");

        let inv_ix = assets.find_account("Investments").unwrap();
        let inv = journal.get_account(inv_ix);
        assert_eq!("Investments", inv.name);
        // assert_eq!(Some(assets_ptr), inv.parent_index);
        todo!("fix above");

        let broker_ix = inv.find_account("Broker").unwrap();
        let broker = journal.get_account(broker_ix);
        assert_eq!("Broker", broker.name);
        // assert_eq!(Some(inv_ix), broker.parent_index);
        todo!("fix above");
    }

    /// The master account needs to be created in the Journal automatically.
    #[test]
    fn test_master_gets_created() {
        let j = Journal::new();

        let actual = j.get_master_account();

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
