/*!
 * Tests for the library functionality useful for 3rd-party software.
 */

 use ledger_rs_lib::{journal::Journal, amount::Quantity};

/// Verify the validity of a new transaction.
#[test]
fn test_xact_verification() {
    let src = r#"2023-05-23 Supermarket
    Expenses:Food  20 EUR
    Assets:Cash
"#;
    let mut journal = Journal::new();

    // Act
    ledger_rs_lib::parse_text(src, &mut journal);

    // Assert
    assert_eq!(1, journal.xacts.len());
    let xact = &journal.xacts[0];
    assert_eq!(Quantity::from(-20), xact.posts[1].amount.as_ref().unwrap().quantity);
}