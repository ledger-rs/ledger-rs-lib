use crate::{xact::Xact, account::Account, post::Post, commodity::Commodity};

/**
 * Journal
 */

pub type AccountIndex = usize;
pub type CommodityIndex = usize;
pub type PostIndex = usize;
pub type XactIndex = usize;

pub struct Journal {
    // pub master: Account,
    
    pub commodities: Vec<Commodity>,
    // todo: commodity_pool
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

    pub fn add_account(&mut self, acct: Account) -> AccountIndex {
        self.accounts.push(acct);
        self.accounts.len() - 1
    }

    pub fn add_commodity(&mut self, commodity: Commodity) -> CommodityIndex {
        self.commodities.push(commodity);
        self.commodities.len() - 1
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

    pub fn find_commodity(&self, symbol: &str) -> Option<&Commodity> {
        self.commodities.iter().find(|c| c.symbol == symbol)
    }

    pub fn get_account(&self, index: usize) -> &Account {
        &self.accounts[index]
    }

    pub fn get_commodity(&self, index: CommodityIndex) -> &Commodity {
        &self.commodities[index]
    }

    pub fn get_posts(&self, indices: &Vec<PostIndex>) -> Vec<&Post> {
        indices.iter().map(|i| &self.posts[*i]).collect()
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
}