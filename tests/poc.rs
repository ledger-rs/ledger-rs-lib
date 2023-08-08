/*!
 * Proof-of-concept and ideas
 *
 * References between model entities
 * It seems that only using Rc<> would work for this purpose.
 */

use core::panic;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use ledger_rs_lib::commodity::Commodity;

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

    pub fn get_children(&self) -> &Vec<Rc<RefCell<Account>>> {
        &self.children
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

    let master = accounts_map.get("master").unwrap();
    let binding = master.borrow();
    let binding = binding.get_children().get(0).unwrap().borrow();
    let binding = binding.get_children().get(0).unwrap().borrow();
    let grandchild_name = binding.name.as_str();
    assert_eq!("bank", grandchild_name);

    // but if we copy the end value, then we don't need to break it down.
    let name = master
        .borrow()
        .children
        .get(0)
        .unwrap()
        .borrow()
        .name
        .to_owned();
    assert_eq!("assets", name);

    let name = master
        .borrow()
        .children
        .get(0)
        .unwrap()
        .borrow()
        .children
        .get(0)
        .unwrap()
        .borrow()
        .name
        .to_owned();
    assert_eq!("bank", name);
}

/// Pointer gymnastics. Pass pointers around and convert to references
/// when needed.
/// In a structure where the data is only populated and never deleted,
/// this should be safe.
#[test]
fn test_pointer_passing() {
    // alchemy?
    // arrange
    const CURRENCY: &str = "EUR";
    let mut container = HashMap::new();
    let eur = Commodity::new(CURRENCY);
    let eur_ptr = &eur as *const Commodity;
    let eur_mut_ptr = eur_ptr as *mut Commodity;
    
    // act
    container.insert(eur.symbol.to_owned(), eur);

    // assert
    let expected_ref: &Commodity;
    let expected_mut_ref: &mut Commodity;
    unsafe {
        expected_ref = &*eur_ptr;
        expected_mut_ref = &mut*eur_mut_ptr;
    }
    assert_eq!(CURRENCY, expected_ref.symbol);
    assert_eq!(CURRENCY, expected_mut_ref.symbol);
}