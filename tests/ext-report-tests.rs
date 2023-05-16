/**
 * External reports tests
 */

fn split_args(command: &str) -> Vec<String> {
    shell_words::split(command).unwrap()
}

// todo: #[test]
fn minimal_test_b() {
    let command = "b -f tests/minimal.ledger";
    let args: Vec<String> = shell_words::split(command).unwrap();
    
    let actual = ledger_rs_lib::run(args);

    // todo: compare to expected output.
    assert!(false)
}

#[test]
fn test_accounts() {
    let command = "accounts -f tests/minimal.ledger";
    let args: Vec<String> = shell_words::split(command).unwrap();

    let actual = ledger_rs_lib::run(args);

    assert!(!actual.is_empty());
    let expected = vec!["Expenses", "Assets"];
    assert_eq!(expected, actual);
}

#[test]
fn test_balance_plain() {
    let args = split_args("b -f tests/basic.ledger");

    let actual = ledger_rs_lib::run(args);

    todo!("assert the output")
}