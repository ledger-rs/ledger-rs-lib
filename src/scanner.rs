/**
 * Scanner scans the input text and returns tokens (groups of characters) back for parsing.
 */

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
pub(crate) fn tokenize_xact_header(input: &str) -> [&str; 4] {
    if input.is_empty() {
        panic!("Invalid input for Xact record.")
    }

    // Dates.
    // Date has to be at the beginning.

    let (date, input) = tokenize_date(input);

    // aux date
    let (aux_date, input) = tokenize_aux_date(input);

    // Payee

    let (payee, input) = tokenize_payee(input);

    // Note
    let note = tokenize_note(input);

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

fn tokenize_note(input: &str) -> &str {
    match input.is_empty() {
        true => "",
        false => &input[3..].trim(),
    }
    // log::debug!("note: {:?}", note);
}

/// Parse payee from the input string.
/// Returns (payee, processed length)
fn tokenize_payee(input: &str) -> (&str, &str) {
    match input.find("  ;") {
        Some(index) => (&input[..index].trim(), &input[index..]),
        None => (input.trim(), ""),
    }
}

/// Parse tokens from a Post line.
///   ACCOUNT  AMOUNT  [; NOTE]
///
/// input: &str  trimmed Post content
/// returns [account, amount]
pub(crate) fn tokenize_post(input: &str) -> [&str; 2] {
    // two spaces is a separator betweer the account and amount.
    // Eventually, also support the tab as a separator:
    // |p| p == "  " || p  == '\t'
    match input.find("  ") {
        Some(i) => return [&input[..i], &input[i + 2..]],
        None => [input, ""],
    }
}

#[cfg(test)]
mod scanner_tests_xact {
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
mod scanner_tests_post {
    use super::tokenize_post;

    #[test]
    fn test_tokenize_post_full() {
        let input = "Assets  20 VEUR @ 25.6 EUR";

        // Act
        let tokens = tokenize_post(input);

        // Assert
        let mut iterator = tokens.into_iter();

        assert_eq!("Assets", iterator.next().unwrap());
        assert_eq!("20 VEUR @ 25.6 EUR", iterator.next().unwrap());
    }

    #[test]
    fn test_tokenize_post_w_amount() {
        let input = "Assets  20 EUR";

        // Act
        let tokens = tokenize_post(input);

        // Assert
        let mut iterator = tokens.into_iter();

        assert_eq!("Assets", iterator.next().unwrap());
        assert_eq!("20 EUR", iterator.next().unwrap());
    }

    #[test]
    fn test_tokenize_post_quantity_only() {
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
