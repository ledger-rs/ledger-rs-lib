use std::cell::RefCell;

/**
 * External tests
 */

// todo: #[test]
fn minimal_test() {
    let command = "b -f tests/minimal.ledger";
    let args: Vec<String> = shell_words::split(command).unwrap();
    
    let actual = ledger_rs_prototype::run(args);

    // todo: compare to expected output.
    assert!(false)
}
