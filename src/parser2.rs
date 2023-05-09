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
use std::io::{BufRead, BufReader, Read};

use crate::{context::ParsingContext, journal::Journal};

pub(crate) fn read<T: Read>(source: T) -> Journal {
    // iterate over lines

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
                // end of file
                break;
            }
            Ok(_) => {
                // Remove the trailing newline characters
                let trimmed = &line.trim_end();

                read_next_directive(trimmed);
            }
        }

        // clear the buffer before reading the next line.
        line.clear();
    }

    context.journal
}

fn read_next_directive(line: &str) {
    if line.is_empty() {
        return;
    }

    // TODO: determine what the line is
    match line.chars().nth(0).unwrap() {
        // comments
        ';' | '#' | '*' | '|' => {
            // ignore
            return;
        }

        '-' => {
            // option_directive
        }

        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
            // Starts with date.
            let tokens = parse_xact_header(line);
            // TODO: create_xact(tokens);
            // todo: try to read Posts directly here.
            //reader
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
fn parse_xact_header(line: &str) -> [&str; 4] {
    if line.is_empty() {
        panic!("Invalid input for Xact record.")
    }

    let mut cursor: usize = 0;

    // Dates.
    // Date has to be at the beginning.

    let (date, offset) = parse_date(line);
    cursor += offset;

    // aux date
    let (aux_date, offset) = parse_aux_date(&line[cursor..]);
    cursor += offset;

    // Payee

    let (payee, offset) = parse_payee(&line[cursor..]);
    cursor += offset;

    // Note
    let note = parse_note(&line[cursor..]);

    [date, aux_date, payee, note]
}

/// Parse date from the input string.
///
/// returns the (date string, processed length)
fn parse_date(input: &str) -> (&str, usize) {
    let date: &str;
    let offset: usize;

    match input.find(|c| c == '=' || c == ' ') {
        Some(index) => {
            offset = index;
            date = &input[..offset];
        }
        None => {
            offset = input.len();
            date = &input;
            // return [date, "", "", ""];
        }
    };
    log::debug!("date: {:?}", date);
    (date, offset)
}

fn parse_aux_date(input: &str) -> (&str, usize) {
    let aux_date: &str;
    let mut cursor: usize = 0;

    match input.chars().next() {
        Some(' ') => {
            // no aux date
            aux_date = "";
        }
        Some('=') => {
            // have aux date. Skip '=', increase the offset by 1.

            cursor = match input[1..].find(' ') {
                Some(i) => 1 + i,
                None => input.len(),
            };
            aux_date = &input[1..cursor]
        }
        Some(_) => panic!("should not happen"),
        None => {
            // end of line.
            aux_date = "";
        }
    }
    log::debug!("aux_date: {:?}", aux_date);
    (aux_date, cursor)
}

/// Parse payee from the input string.
/// Returns (payee, processed length)
fn parse_payee(input: &str) -> (&str, usize) {
    let payee: &str;
    let cursor: usize;

    match input.find("  ;") {
        Some(index) => {
            cursor = index;
            payee = &input[..cursor].trim();
        }
        None => {
            // skip the ws
            let start = match next_non_ws(input) {
                Some(i) => i,
                None => 0,
            };

            payee = &input[start..].trim();
            cursor = input.len();
        }
    };
    (payee, cursor)
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

/// Parse tokens from a Post line.
fn parse_post(line: &str) -> [&str; 1] {
    todo!("complete")
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
mod parser_tests {
    use super::{parse_date, parse_xact_header};

    #[test]
    fn test_parsing_xact_header() {
        std::env::set_var("RUST_LOG", "trace");

        let input = "2023-05-01 Payee  ; Note";

        let mut iter = parse_xact_header(input).into_iter();
        // let [date, aux_date, payee, note] = iter.as_slice();

        assert_eq!("2023-05-01", iter.next().unwrap());
        assert_eq!("", iter.next().unwrap());
        assert_eq!("Payee", iter.next().unwrap());
        assert_eq!("Note", iter.next().unwrap());
    }

    #[test]
    fn test_parsing_xact_header_aux_dates() {
        let input = "2023-05-02=2023-05-01 Payee  ; Note";

        let mut iter = parse_xact_header(input).into_iter();

        assert_eq!("2023-05-02", iter.next().unwrap());
        assert_eq!("2023-05-01", iter.next().unwrap());
        assert_eq!("Payee", iter.next().unwrap());
        assert_eq!("Note", iter.next().unwrap());
    }

    #[test]
    fn test_parsing_xact_header_no_note() {
        let input = "2023-05-01 Payee";

        let mut iter = parse_xact_header(input).into_iter();

        assert_eq!("2023-05-01", iter.next().unwrap());
        assert_eq!("", iter.next().unwrap());
        assert_eq!("Payee", iter.next().unwrap());
        assert_eq!("", iter.next().unwrap());
    }

    #[test]
    fn test_parsing_xact_header_no_payee_w_note() {
        let input = "2023-05-01  ; Note";

        let mut iter = parse_xact_header(input).into_iter();

        assert_eq!("2023-05-01", iter.next().unwrap());
        assert_eq!("", iter.next().unwrap());
        assert_eq!("", iter.next().unwrap());
        assert_eq!("Note", iter.next().unwrap());
    }

    #[test]
    fn test_parsing_xact_header_date_only() {
        let input = "2023-05-01";

        let mut iter = parse_xact_header(input).into_iter();

        assert_eq!(input, iter.next().unwrap());
        assert_eq!("", iter.next().unwrap());
        assert_eq!("", iter.next().unwrap());
        assert_eq!("", iter.next().unwrap());
    }

    #[test]
    fn test_date_w_aux() {
        let input = "2023-05-01=2023";

        let (date, offset) = parse_date(input);

        assert_eq!("2023-05-01", date);
        assert_eq!(10, offset);
    }
}

#[cfg(test)]
mod lexer_tests {}
