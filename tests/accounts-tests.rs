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
    assert_eq!("", accounts.iter().nth(0).unwrap().name);
    assert_eq!("Expenses", accounts.iter().nth(1).unwrap().name);
    assert_eq!("Food", accounts.iter().nth(2).unwrap().name);
    assert_eq!("Assets", accounts.iter().nth(3).unwrap().name);
    assert_eq!("Cash", accounts.iter().nth(4).unwrap().name);
}