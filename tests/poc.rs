/*!
 * Proof-of-concept and ideas
 *
 * References between model entities
 * It seems that only using Rc<> would work for this purpose.
 */

use core::panic;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, PartialEq)]
struct Account {
    pub name: String,
    // posts: Vec<&'a Post<'a>>,
    pub parent: Option<Rc<RefCell<Account>>>,
    pub children: Vec<Rc<RefCell<Account>>>,

    // posts: Vec<Rc<
}

impl Account {
    pub fn new(name: &str) -> Self {
        // Self { parent: None, children: vec![] }
        Self {
            name: name.to_owned(),
            parent: None,
            children: vec![],
        }
    }
}

#[derive(Debug)]
struct Post {
    // account: Rc<Account>,
    // other post fields...
}

/// Using references with Rc<RefCell<>>
#[test]
fn test_ref_w_rc() {
    // arrange
    let root = Rc::new(RefCell::new(Account::new("master")));
    let mut accounts_map = HashMap::new();
    {
        root.borrow_mut().parent = None;
        accounts_map.insert("master", root.clone());

        // add assets to the map
        let assets = Rc::new(RefCell::new(Account::new("assets")));
        assets.borrow_mut().parent = Some(root.clone());
        accounts_map.insert("assets", assets.clone());
        // add assets to root's children
        root.borrow_mut().children.push(assets.clone());

        // add bank to the accounts map
        let bank = Rc::new(RefCell::new(Account::new("bank")));
        bank.borrow_mut().parent = Some(assets.clone());
        accounts_map.insert("bank", bank.clone());
        // add bank to assets' children
        assets.borrow_mut().children.push(bank.clone());
    }

    // act
    let bank = accounts_map.get("bank").unwrap();
    let Some(assets) = &bank.borrow().parent else {panic!("yo")};
    let Some(master) = &assets.borrow().parent else {panic!("yo")};

    // assert
    assert_eq!("master", master.borrow().name);
    assert_eq!(1, master.borrow().children.len());

    //let child = master.borrow().children.get(0).unwrap().borrow();
}
