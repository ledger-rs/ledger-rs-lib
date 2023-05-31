/**
 * Test operations with accounts
 */

use ledger_rs_lib::journal::Journal;

/// Read a transaction and parse account tree
#[test]
fn test_creating_account_tree() {
    // Arrange
    let file_path = "tests/basic.ledger";
    let mut journal = Journal::new();

    // Act
    ledger_rs_lib::parse_file(file_path, &mut journal);

    // Assert
    assert_eq!(5, journal.accounts.len());
    assert_eq!("", journal.accounts.iter().nth(0).unwrap().name);
    assert_eq!("Expenses", journal.accounts.iter().nth(1).unwrap().name);
    assert_eq!("Food", journal.accounts.iter().nth(2).unwrap().name);
    assert_eq!("Assets", journal.accounts.iter().nth(3).unwrap().name);
    assert_eq!("Cash", journal.accounts.iter().nth(4).unwrap().name);
}