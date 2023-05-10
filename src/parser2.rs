/**
 * Parser with iterators
 *
 * The main idea here is to minimize memory allocations.
 * The parsing is done in functions, not objects.
 * Each parser will provide an iterator over the tokens it recognizes, i.e. Xact parser
 * will iterate over the Xact header items: date, payee, note.
 * Post parser provides an iterator over Account, Amount. Amount parser provides
 * sign, quantity, symbol, price.
 * Iterator returns None if a token is not present.
 *
 * Tokens are then handled by lexer, which creates instances of Structs and populates
 * the collections in the Journal.
 * It also creates links among the models. This functionality is from finalize() function.
 */
use std::{
    error::Error,
    io::{BufRead, BufReader, Read},
};

use crate::{context::ParsingContext, journal::Journal, xact::Xact};

pub(crate) fn read<T: Read>(source: T) -> Journal {
    // iterate over lines

    let mut reader = BufReader::new(source);
    // let mut context = ParsingContext::new();
    // To avoid allocation, reuse the String variable.
    let mut line = String::new();

    loop {
        match reader.read_line(&mut line) {
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
            Ok(0) => {
                // end of file
                break;
            }
            Ok(_) => {
                // Remove the trailing newline characters
                // let trimmed = &line.trim_end();

                match read_next_directive(&mut line, &mut reader) {
                    Ok(_) => (), // continue
                    Err(err) => {
                        log::error!("Error: {:?}", err);
                        println!("Error: {:?}", err);
                        break;
                    }
                };
            }
        }

        // clear the buffer before reading the next line.
        line.clear();
    }

    todo!("return journal")
}

fn read_next_directive<T: Read>(
    line: &mut String,
    reader: &mut BufReader<T>,
) -> Result<(), String> {
    if line.is_empty() {
        return Ok(());
    }

    // TODO: determine what the line is
    match line.chars().nth(0).unwrap() {
        // comments
        ';' | '#' | '*' | '|' => {
            // ignore
            return Ok(());
        }

        '-' => {
            // option_directive
        }

        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
            // Starts with date/number.
            // TODO: move all this into a function
            xact_directive(reader, line);
        }

        ' ' | '\t' => {
            todo!("complete")
        }

        // The rest
        c => {
            // 4.7.2 command directives

            // if !general_directive()
            match c {
                'P' => {
                    // price
                }

                _ => {
                    todo!("handle other directives");
                }
            }
            todo!("the rest")
        }
    }

    // TODO: lexer - create model elements from tokens
    // TODO: store model elements in collections and link.

    Ok(())
}

fn xact_directive<T: Read>(reader: &mut BufReader<T>, line: &mut String) {
    let tokens = tokenize_xact_header(line);
    let xact = Xact::create(tokens[0], tokens[1], tokens[2], tokens[3]);

    // TODO: read the Xact contents (Posts, Comments, etc.)
    // TODO: read until separator (empty line)
    loop {
        line.clear(); // empty the buffer before reading
        match reader.read_line(line) {
            Err(e) => {
                println!("Error: {:?}", e);
                break;
            }
            Ok(0) => {
                // end of file
                break;
            }
            Ok(_) => {
                if line.is_empty() {
                    todo!("finalize the transaction")
                }

                // parse
                match line.chars().peekable().peek() {
                    Some(' ') => {
                        // valid line
                        tokenize_xact_content(line);
                        todo!("process")
                    }
                    _ => {
                        panic!("should not happen")
                    }
                }
            }
        }

        // log::debug!("read: {:?}, {:?}", x, &line);
        line.clear(); // empty the buffer before reading
    }
    todo!("put everything into the Journal")
}

/// Parse Xact header record.
/// 2023-05-05=2023-05-01 Payee  ; Note
///
/// returns [date, aux_date, payee, note]
///
/// Check for .is_empty() after receiving the result and handle appropriately.
///
/// Ledger's documentation specifies the following format
/// ```
/// DATE[=EDATE] [*|!] [(CODE)] DESC
/// ```
/// but the DESC is not mandatory. <Unspecified Payee> is used in that case.
/// So, the Payee/Description is mandatory in the model but not in the input.
fn tokenize_xact_header(input: &str) -> [&str; 4] {
    if input.is_empty() {
        panic!("Invalid input for Xact record.")
    }

    // Dates.
    // Date has to be at the beginning.

    let (date, input) = tokenize_date(input);

    // aux date
    let (aux_date, input) = tokenize_aux_date(input);

    // Payee

    let (payee, input) = parse_payee(input);

    // Note
    let note = parse_note(input);

    [date, aux_date, payee, note]
}

/// Parse date from the input string.
///
/// returns the (date string, remaining string)
fn tokenize_date(input: &str) -> (&str, &str) {
    // let date: &str;
    // let offset: usize;

    match input.find(|c| c == '=' || c == ' ') {
        Some(index) => {
            // offset = index;
            //date = &input[..index];
            return (&input[..index], &input[index..]);
        }
        None => {
            // offset = input.len();
            // date = &input;
            // return [date, "", "", ""];
            return (&input, "");
        }
    };
    // log::debug!("date: {:?}", date);
    // (date, offset)
}

/// Parse auxillary date.
/// Returns the (date_str, remains).
fn tokenize_aux_date(input: &str) -> (&str, &str) {
    let aux_date: &str;
    // let mut cursor: usize = 0;
    // skip ws
    // let input = input.trim_start();

    match input.chars().peekable().peek() {
        Some('=') => {
            // have aux date.
            // skip '=' sign
            let input = input.trim_start_matches('=');

            // find the next separator
            match input.find(' ') {
                Some(i) => return (&input[..i], &input[i..]),
                None => return (input, ""),
            };
        }
        _ => {
            // end of line, or character other than '='
            return ("", input);
        }
    }
}

/// Parse payee from the input string.
/// Returns (payee, processed length)
fn parse_payee(input: &str) -> (&str, &str) {
    match input.find("  ;") {
        Some(index) => (&input[..index].trim(), &input[index..]),
        None => (input.trim(), ""),
    }
}

fn parse_note(input: &str) -> &str {
    match input.is_empty() {
        true => "",
        false => &input[3..].trim(),
    }
    // log::debug!("note: {:?}", note);
}

/// Create Xact from tokens.
/// Lexer function.
fn create_xact(tokens: [&str; 4]) {
    todo!("create xact from tokens")
}

/// Process the Xact content line. Could be a Comment or a Post.
fn tokenize_xact_content(input: &str) -> [&str; 2] {
    let input = input.trim_start();

    match input.chars().peekable().peek() {
        Some(';') => todo!("trailing note"),
        _ => tokenize_post(input)
    }
}

/// Parse tokens from a Post line.
///   ACCOUNT  AMOUNT  [; NOTE]
/// 
/// input: &str  trimmed Post content
/// returns [account, amount]
fn tokenize_post(input: &str) -> [&str; 2] {
    // two spaces is a separator betweer the account and amount.
    // Eventually, also support the tab as a separator:
    // |p| p == "  " || p  == '\t'
    match input.find("  ") {
        Some(i) => return [&input[..i], &input[i+2..]],
        None => [input, ""],
    }
}

/// Find the index of the next non-ws character.
fn next_non_ws(input: &str) -> Option<usize> {
    input.find(|c| c != ' ' && c != '\t')
}

#[cfg(test)]
mod full_tests {
    use crate::account::Account;
    use std::io::Cursor;

    #[test]
    fn test_minimal_parsing() {
        let input = r#"; Minimal transaction
2023-04-10 Supermarket
    Expenses  20
    Assets
"#;
        let cursor = Cursor::new(input);

        let journal = super::read(cursor);

        assert_eq!(1, journal.xacts.len());

        let xact = journal.xacts.first().unwrap();
        assert_eq!("Supermarket", xact.payee);
        assert_eq!(2, xact.posts.len());

        // let post_1 = xact.posts.iter().nth(0).unwrap();
        let post1 = &journal.posts[xact.posts[0]];
        assert_eq!(Account::new("Expenses"), post1.account);
        assert_eq!("20", post1.amount.as_ref().unwrap().quantity.to_string());
        assert_eq!(None, post1.amount.as_ref().unwrap().commodity);

        // let post_2 = xact.posts.iter().nth(1).unwrap();
        let post2 = &journal.posts[xact.posts[1]];
        assert_eq!(Account::new("Assets"), post2.account);
    }
}

#[cfg(test)]
mod lexer_tests_xact {
    use super::{tokenize_date, tokenize_xact_header};

    #[test]
    fn test_parsing_xact_header() {
        std::env::set_var("RUST_LOG", "trace");

        let input = "2023-05-01 Payee  ; Note";

        let mut iter = tokenize_xact_header(input).into_iter();
        // let [date, aux_date, payee, note] = iter.as_slice();

        assert_eq!("2023-05-01", iter.next().unwrap());
        assert_eq!("", iter.next().unwrap());
        assert_eq!("Payee", iter.next().unwrap());
        assert_eq!("Note", iter.next().unwrap());
    }

    #[test]
    fn test_parsing_xact_header_aux_dates() {
        let input = "2023-05-02=2023-05-01 Payee  ; Note";

        let mut iter = tokenize_xact_header(input).into_iter();

        assert_eq!("2023-05-02", iter.next().unwrap());
        assert_eq!("2023-05-01", iter.next().unwrap());
        assert_eq!("Payee", iter.next().unwrap());
        assert_eq!("Note", iter.next().unwrap());
    }

    #[test]
    fn test_parsing_xact_header_no_note() {
        let input = "2023-05-01 Payee";

        let mut iter = tokenize_xact_header(input).into_iter();

        assert_eq!("2023-05-01", iter.next().unwrap());
        assert_eq!("", iter.next().unwrap());
        assert_eq!("Payee", iter.next().unwrap());
        assert_eq!("", iter.next().unwrap());
    }

    #[test]
    fn test_parsing_xact_header_no_payee_w_note() {
        let input = "2023-05-01  ; Note";

        let mut iter = tokenize_xact_header(input).into_iter();

        assert_eq!("2023-05-01", iter.next().unwrap());
        assert_eq!("", iter.next().unwrap());
        assert_eq!("", iter.next().unwrap());
        assert_eq!("Note", iter.next().unwrap());
    }

    #[test]
    fn test_parsing_xact_header_date_only() {
        let input = "2023-05-01";

        let mut iter = tokenize_xact_header(input).into_iter();

        assert_eq!(input, iter.next().unwrap());
        assert_eq!("", iter.next().unwrap());
        assert_eq!("", iter.next().unwrap());
        assert_eq!("", iter.next().unwrap());
    }

    #[test]
    fn test_date_w_aux() {
        let input = "2023-05-01=2023";

        let (date, remains) = tokenize_date(input);

        assert_eq!("2023-05-01", date);
        assert_eq!("=2023", remains);
    }

    // test built-in ws removal with .trim()
    #[test]
    fn test_ws_skip() {
        // see if trim removes tabs
        let input = "\t \t Text \t";
        let actual = input.trim();

        assert_eq!("Text", actual);

        // This confirms that .trim() and variants can be used for skipping whitespace.
    }
}

#[cfg(test)]
mod lexer_tests_post {
    use super::tokenize_xact_content;
    use super::tokenize_post;

    #[test]
    fn test_tokenize_post_full() {
        let input = "  Assets  20 VEUR @ 25.6 EUR";

        // Act
        let tokens = tokenize_xact_content(input);

        // Assert        
        let mut iterator = tokens.into_iter();

        assert_eq!("Assets", iterator.next().unwrap());
        assert_eq!("20 VEUR @ 25.6 EUR", iterator.next().unwrap());
    }

    #[test]
    fn test_tokenize_post_w_amount() {
        let input = "  Assets  20 EUR";

        // Act
        let tokens = tokenize_xact_content(input);

        // Assert        
        let mut iterator = tokens.into_iter();

        assert_eq!("Assets", iterator.next().unwrap());
        assert_eq!("20 EUR", iterator.next().unwrap());
    }

    #[test]
    fn tokenize_xact_post_quantity_only() {
        let input = "  Assets  20";

        // Act
        let tokens = tokenize_xact_content(input);

        // Assert        
        let mut iterator = tokens.into_iter();

        assert_eq!("Assets", iterator.next().unwrap());
        assert_eq!("20", iterator.next().unwrap());
    }

    #[test]
    fn tokenize_post_quantity_only() {
        let input = "Assets  20";

        // Act
        let tokens = tokenize_post(input);

        // Assert        
        let mut iterator = tokens.into_iter();

        assert_eq!("Assets", iterator.next().unwrap());
        assert_eq!("20", iterator.next().unwrap());
    }

    #[test]
    fn test_tokenize_post_account() {
        let input = "Assets";

        // Act
        let tokens = tokenize_post(input);

        // Assert        
        let mut iterator = tokens.into_iter();

        assert_eq!("Assets", iterator.next().unwrap());
        assert_eq!("", iterator.next().unwrap());
    }
}

#[cfg(test)]
mod parser_tests {}
