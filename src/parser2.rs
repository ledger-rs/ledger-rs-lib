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

pub(crate) fn parse<T: Read>(source: T) -> Journal {
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
            parse_xact_header(line);
        }

        ' ' | '\t' => {}

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
    // TODO: parse line:
    // TODO:   recognize tokens
    // TODO:   create iterator over tokens
    // TODO: iterate over line tokens
    // TODO: lexer - create model elements from tokens
    // TODO: store model elements in collections and link.
}

/// Parse Xact header record.
/// 2023-05-05=2023-05-01 Payee  ; Note
///
/// returns (date, aux_date, payee, note)
/// 
/// Ledger's documentation specifies the following format
/// ```
/// DATE[=EDATE] [*|!] [(CODE)] DESC
/// ```
/// but the DESC is not mandatory. <Unspecified Payee> is used in that case.
/// So, the Payee/Description is mandatory in the model but not in the input.
fn parse_xact_header(line: &str) -> (&str, Option<&str>, Option<&str>, Option<&str>) {
    if line.is_empty() {
        panic!("Invalid input for Xact record.")
    }

    let mut cursor: usize = 0;
    let date: &str;
    let aux_date: Option<&str>;
    let payee: Option<&str>;
    let note: Option<&str>;

    // Dates.
    // Date has to be at the beginning

    cursor = match line.find(|c| c == '=' || c == ' ') {
        Some(index) => index,
        None => line.len(),
    };
    date = &line[0..cursor];
    log::debug!("date: {:?}", date);

    // aux date
    match line.chars().nth(cursor) {
        Some(' ') => (), // no aux date
        Some('=') => todo!("have aux date"),
        Some(_) => panic!("should not happen"),
        None => (), // skip
    }

    // Payee

    cursor = match line.find("  ;") {
        Some(index) => index,
        None => line.len(),
    };
    // payee = &line[];
    payee = None;

    let note_str = match line.find("  ;") {
        Some(index) => todo!(),
        None => todo!(),
    };

    (date, aux_date, payee, note)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::account::Account;

    use super::parse_xact_header;

    #[test]
    fn test_minimal_parsing() {
        let input = r#"; Minimal transaction
        2023-04-10 Supermarket
            Expenses  20
            Assets
        "#;
        let cursor = Cursor::new(input);

        let journal = super::parse(cursor);

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

    #[test]
    fn test_parsing_xact_header() {
        let _ = env_logger::builder().write_style(env_logger::WriteStyle::Always).is_test(true).try_init();
        let input = "2023-05-01 Payee  ; Note";

        let (date, aux_date, payee, note) = parse_xact_header(input);

        todo!("complete")
    }

    #[test]
    fn test_parsing_xact_header_aux_dates() {
        let input = "2023-05-02=2023-05-01 Payee  ; Note";

        let (date, aux_date, payee, note) = parse_xact_header(input);

        todo!("complete")
    }

    #[test]
    fn test_parsing_xact_header_no_note() {
        let input = "2023-05-01 Payee";

        let (date, aux_date, payee, note) = parse_xact_header(input);

        todo!("complete")
    }

    #[test]
    fn test_parsing_xact_header_no_payee() {
        let input = "2023-05-01 Payee  ; Note";

        let (date, aux_date, payee, note) = parse_xact_header(input);

        todo!("complete")
    }

    #[test]
    fn test_parsing_xact_header_no_payee_note() {
        let input = "2023-05-01  ; Note";

        let (date, aux_date, payee, note) = parse_xact_header(input);

        todo!("complete")
    }

    #[test]
    fn test_parsing_xact_header_date_only() {
        let input = "2023-05-01";

        let (date, aux_date, payee, note) = parse_xact_header(input);

        assert_eq!(input, date);
        assert_eq!(None, aux_date);
        assert_eq!(None, payee);
        assert_eq!(None, note);
    }
}
