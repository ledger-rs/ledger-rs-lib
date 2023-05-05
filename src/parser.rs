/**
 * Parser
 *
 * Parses textual input into the model structure.
 */
use std::io::{Read, BufReader, BufRead};

use crate::{journal::Journal, context::ParsingContext};

enum LineParseResult {
    Comment,
    Empty,
    // Xact(Xact),
    // Post(Post),
}

/// parse textual input
pub fn parse<T: Read>(source: T) -> Journal {
    let mut reader = BufReader::new(source);
    let mut context = ParsingContext::new();
    // To avoid allocation, reuse the String variable.
    let mut line = String::new();

    loop {
        match reader.read_line(&mut line) {
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
            Ok(0) => {
                // end of file?
                println!("End of file");
                break;
            }
            Ok(_) => {
                // Remove the trailing newline characters
                // let clean_line = strip_trailing_newline(&line);
                let clean_line = &line.trim_end();

                // use the read value
                // TODO: let result = parse_line(&mut context, &clean_line);
                // TODO: process_parsed_element(&mut context, result);

                // clear the buffer before reading the next line.
                line.clear();
            }
        }
    }

    context.journal
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
