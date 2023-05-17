/**
 * External reports tests
 */

fn split_args(command: &str) -> Vec<String> {
    shell_words::split(command).unwrap()
}

#[test]
fn minimal_balance_test_b() {
    let args: Vec<String> = shell_words::split("b -f tests/minimal.ledger").unwrap();
    
    let actual = ledger_rs_lib::run(args);

    // todo: compare to expected output.
    assert!(false)
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