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