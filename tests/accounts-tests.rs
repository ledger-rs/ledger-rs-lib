/*!
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
    let accounts = journal.master.flatten_account_tree();
    assert_eq!(5, accounts.len());
    let mut iterator = accounts.iter();
    assert_eq!("", iterator.next().unwrap().name);
    assert_eq!("Assets", iterator.next().unwrap().name);
    assert_eq!("Cash", iterator.next().unwrap().name);
    assert_eq!("Expenses", iterator.next().unwrap().name);
    assert_eq!("Food", iterator.next().unwrap().name);
}