use chrono::NaiveDate;
use rust_decimal_macros::dec;

/**
 * External parser tests
 */

#[test]
fn test_parsing() {
    let file_path = "tests/minimal.ledger";
    let journal = ledger_rs_prototype::parse(file_path);

    assert_eq!(1, journal.xacts.len());
    assert_eq!(2, journal.posts.len());
}

/// Testing reading the blank lines. Seems to be an issue on Windows?
#[test]
fn test_parsing_two_xact() {
    let file_path = "tests/two_xact.ledger";
    let journal = ledger_rs_prototype::parse(file_path);

    assert_eq!(2, journal.xacts.len());
    assert_eq!(4, journal.posts.len());
}

#[test]
fn detailed_basic_test() {
    let file_path = "tests/basic.ledger";
    
    // Act
    let journal = ledger_rs_prototype::parse(file_path);

    // Assert
    assert_eq!(1, journal.xacts.len());
    let xact = journal.xacts.first().unwrap();
    assert_eq!(NaiveDate::parse_from_str("2023-04-21", "%Y-%m-%d").unwrap(), xact.date.unwrap());
    assert_eq!("Supermarket", xact.payee);
    // Posts
    assert_eq!(2, journal.posts.len());
    let post1 = journal.posts.get(xact.posts[0]).unwrap();
    assert_eq!("Expenses:Food", post1.account_temp.name);
    let amount1 = &post1.amount;
    assert_eq!(dec!(20), amount1.quantity);
    assert_eq!("EUR", amount1.commodity.as_ref().unwrap().symbol);

    let post2 = journal.posts.get(xact.posts[1]).unwrap();
    assert_eq!("Assets:Cash", post2.account_temp.name);
    let amount2 = &post2.amount;
    assert_eq!(dec!(-20), amount2.quantity);
    assert_eq!("EUR", amount2.commodity.as_ref().unwrap().symbol);
}