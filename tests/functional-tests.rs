use ledger_rs_lib::journal::Journal;

/**
 * Functional tests
 * 
 */

#[test]
fn test_price_reading() {
    // Arrange
    let mut j = Journal::new();
    let text = r#"
P 2022-03-03 13:00:00 EUR 1.12 USD
"#;
    // Act
    ledger_rs_lib::parse_text(text, &mut j);

    // Assert
    todo!("check that the price was parsed")
}