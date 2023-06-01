/*!
 * External reports tests
 */

#[test]
fn test_balance_minimal() {
    // Act
    let actual = ledger_rs_lib::run_command("b -f tests/minimal.ledger");

    // Assert
    assert!(!actual.is_empty());
    assert_eq!(3, actual.len());
    assert_eq!("Account  has balance ", actual[0]);
    assert_eq!("Account Assets has balance -20", actual[1]);
    assert_eq!("Account Expenses has balance 20", actual[2])
}

#[test]
fn test_balance_basic() {
    let actual = ledger_rs_lib::run_command("b -f tests/basic.ledger");

    assert!(!actual.is_empty());
    assert_eq!(5, actual.len());
    assert_eq!("Account  has balance ", actual[0]);
    assert_eq!("Account Assets has balance ", actual[1]);
    assert_eq!("Account Assets:Cash has balance -20 EUR", actual[2]);
    assert_eq!("Account Expenses has balance ", actual[3]);
    assert_eq!("Account Expenses:Food has balance 20 EUR", actual[4]);
}

#[test]
fn test_accounts() {
    // Act
    let actual = ledger_rs_lib::run_command("accounts -f tests/minimal.ledger");

    assert!(!actual.is_empty());
    let expected = vec!["", "Assets", "Expenses"];
    assert_eq!(expected, actual);
}

/// TODO: enable test when the functionality is implemented
//#[test]
fn test_account_filter() {
    // Act
    let actual = ledger_rs_lib::run_command("accounts Asset -f tests/minimal.ledger");

    assert!(!actual.is_empty());
    // Only Assets should be returned.
    let expected = vec!["Assets"];
    assert_eq!(expected, actual);
}

/// Test Balance report, without any parameters.
/// Just two accounts.
#[test]
fn test_balance_plain() {
    let expected = r#"Account Balances
   -20 Assets
    20 Expenses
"#;

    let actual = ledger_rs_lib::run_command("b -f tests/basic.ledger");

    assert!(!actual.is_empty());
    assert_eq!(5, actual.len());
    assert_eq!("Account  has balance ", actual[0]);
    assert_eq!("Account Assets has balance ", actual[1]);
    assert_eq!("Account Assets:Cash has balance -20 EUR", actual[2]);
    assert_eq!("Account Expenses has balance ", actual[3]);
    assert_eq!("Account Expenses:Food has balance 20 EUR", actual[4]);
}

/// TODO: Enable when implemented
/// Display account balances with multiple currencies.
// #[test]
fn test_balance_multiple_currencies() {
    let actual = ledger_rs_lib::run_command("b -f tests/multiple_currencies.ledger");

    assert!(false);
    // assert_eq!("Account Assets:Cash has balance -20 ");
}