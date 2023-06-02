/*!
 * External parser tests
 */

 use chrono::NaiveDate;
 use ledger_rs_lib::{journal::Journal, amount::{Decimal, Amount}, parse_file, pool::CommodityIndex};
 
#[test]
fn smoke_test_parsing() {
    let file_path = "tests/minimal.ledger";
    let mut journal = Journal::new();
    
    ledger_rs_lib::parse_file(file_path, &mut journal);

    assert_eq!(1, journal.xacts.len());
    assert_eq!(2, journal.posts.len());
}

/// Testing reading the blank lines. Seems to be an issue on Windows?
#[test]
fn test_parsing_two_xact() {
    let file_path = "tests/two_xact.ledger";
    let mut journal = Journal::new();
    
    ledger_rs_lib::parse_file(file_path, &mut journal);

    assert_eq!(2, journal.xacts.len());
    assert_eq!(4, journal.posts.len());
}

#[test]
fn detailed_basic_test() {
    let file_path = "tests/basic.ledger";
    let mut journal = Journal::new();

    // Act
    ledger_rs_lib::parse_file(file_path, &mut journal);

    // Assert
    assert_eq!(1, journal.xacts.len());
    let xact = journal.xacts.first().unwrap();
    assert_eq!(NaiveDate::parse_from_str("2023-04-21", "%Y-%m-%d").unwrap(), xact.date.unwrap());
    assert_eq!("Supermarket", xact.payee);
    // Posts
    assert_eq!(2, journal.posts.len());
    let post1 = journal.posts.get(xact.posts[0]).unwrap();
    assert_eq!("Food", journal.get_account(post1.account_index).name);
    let amount1 = &post1.amount.as_ref().unwrap();
    assert_eq!(Decimal::from(20), amount1.quantity);
    let symbol = &journal.commodity_pool.get_commodity(*amount1.commodity_index.as_ref().unwrap()).symbol;
    assert_eq!("EUR", symbol);

    let post2 = journal.posts.get(xact.posts[1]).unwrap();
    assert_eq!("Cash", journal.get_account(post2.account_index).name);
    let amount2 = &post2.amount.as_ref().unwrap();
    assert_eq!(Decimal::from(-20), amount2.quantity);
    let symbol = &journal.commodity_pool.get_commodity(*amount2.commodity_index.as_ref().unwrap()).symbol;
    assert_eq!("EUR", symbol);
}

/// TODO: include when the feature is implemented
//#[test]
fn test_include() {
    // let args = split("accounts -f tests/include.ledger").unwrap();
    let input = "include tests/minimal.ledger";
    let mut journal = Journal::new();

    ledger_rs_lib::parse_text(input, &mut journal);

    assert_eq!(1, journal.xacts.len());
    todo!("complete the feature")
}

#[test]
fn test_parsing_multiple_currencies() {
    // Arrange
    let file_path = "tests/multiple_currencies.ledger";
    let mut journal = Journal::new();

    // Act
    ledger_rs_lib::parse_file(file_path, &mut journal);

    // Assert
    assert!(!journal.xacts.is_empty());
    assert!(!journal.posts.is_empty());
}

#[test]
fn test_parsing_account_tree() {
    // Arrange
    let file_path = "tests/basic.ledger";
    let mut journal = Journal::new();

    // Act
    ledger_rs_lib::parse_file(file_path, &mut journal);

    // Assert
    assert!(!journal.xacts.is_empty());
    assert_eq!(5, journal.accounts.len());
}

#[test]
fn test_parsing_lots_per_unit() {
    let mut journal = Journal::new();

    parse_file("tests/trade-buy-sell.ledger", &mut journal);

    // Assert

    // xacts
    assert!(!journal.xacts.is_empty());
    assert_eq!(2, journal.xacts.len());

    // posts
    assert_eq!(4, journal.posts.len());
    let cur_index: CommodityIndex = 1.into();
    let expected_cost = Amount::new(25.into(), Some(cur_index));
    assert_eq!(expected_cost, journal.posts[2].cost.unwrap());
    let cur = journal.get_commodity(cur_index);
    assert_eq!("EUR", cur.symbol);
}

#[test]
fn test_parsing_lots_full_price() {
    let mut journal = Journal::new();

    parse_file("tests/trade-buy-sell-full-price.ledger", &mut journal);

    // Assert

    // xacts
    assert!(!journal.xacts.is_empty());
    assert_eq!(2, journal.xacts.len());

    // posts
    assert_eq!(4, journal.posts.len());
    let expected_cost = Amount::new(25.into(), Some(1.into()));
    assert_eq!(expected_cost, journal.posts[2].cost.unwrap());
}
