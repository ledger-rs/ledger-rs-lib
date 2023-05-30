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
    assert_eq!(4, journal.accounts.len());
    assert_eq!("Expenses", journal.accounts.iter().nth(0).unwrap().name);
}