/**
 * Parser
 *
 * Parses textual input into the model structure.
 */
use std::io::Read;

enum LineParseResult {
    Comment,
    Empty,
    // Xact(Xact),
    // Post(Post),
}

/// parse textual input
pub fn parse<T: Read>(source: T) {
    todo!()
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::parse;

    #[test]
    fn test_minimal_parser() {
        let input = r#"; Minimal transaction
2023-04-10 Supermarket
    Expenses  20
    Assets

"#;
        let cursor = Cursor::new(input);

        let actual = parse(cursor);

        assert!(false, "add checks");
    }
}
