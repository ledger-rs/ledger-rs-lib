use rust_decimal::Decimal;
use rust_decimal_macros::dec;

/**
 * External reports tests
 */

fn split_args(command: &str) -> Vec<String> {
    shell_words::split(command).unwrap()
}

#[test]
fn test_balance_minimal() {
    let args: Vec<String> = shell_words::split("b -f tests/minimal.ledger").unwrap();
    
    let actual = ledger_rs_lib::run(args);

    // TODO: compare to expected output.
    assert!(!actual.is_empty());
    assert_eq!("Account Assets has balance -20", actual[0]);
    assert_eq!("Account Expenses has balance 20", actual[1])
}

#[test]
fn test_balance_basic() {
    let args: Vec<String> = shell_words::split("b -f tests/basic.ledger").unwrap();
    
    let actual = ledger_rs_lib::run(args);

    // TODO: compare to expected output.
    assert!(!actual.is_empty());
    assert_eq!("Account Assets:Cash has balance -20 EUR", actual[0]);
    assert_eq!("Account Expenses:Food has balance 20 EUR", actual[1])
}

#[test]
fn test_accounts() {
    let args: Vec<String> = shell_words::split("accounts -f tests/minimal.ledger").unwrap();

    let actual = ledger_rs_lib::run(args);

    assert!(!actual.is_empty());
    let expected = vec!["Expenses", "Assets"];
    assert_eq!(expected, actual);
}

#[test]
fn test_account_filter() {
    let args: Vec<String> = split_args("accounts Asset -f tests/minimal.ledger");

    let actual = ledger_rs_lib::run(args);

    assert!(!actual.is_empty());
    let expected = vec!["Assets"];
    assert_eq!(expected, actual);
}

/// Test Balance report, without any parameters.
/// Just two accounts.
#[test]
fn test_balance_plain() {
    let args = split_args("b -f tests/basic.ledger");
    let expected = r#"Account Balances
   -20 Assets
    20 Expenses
"#;

    let actual = ledger_rs_lib::run(args);

    todo!("assert")
    // assert_eq!(expected, actual);
}

/// Display account balances with multiple currencies.
#[test]
fn test_balance_multiple_currencies() {
    let args = split_args("b -f tests/multiple_currencies.ledger");
    let actual = ledger_rs_lib::run(args);

    assert!(false);
    // assert_eq!("Account Assets:Cash has balance -20 ");
}