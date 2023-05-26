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
/// The possible syntax for an amount is:
///   [-]NUM[ ]SYM [@ AMOUNT]
///   SYM[ ][-]NUM [@ AMOUNT]
///
/// input: &str  Post content
/// returns [account, amount]
pub(crate) fn scan_post(input: &str) -> [&str; 5] {
    let input = input.trim_start();

    // two spaces is a separator betweer the account and amount.
    // Eventually, also support the tab as a separator:
    // something like |p| p == "  " || p  == '\t'
    match input.find("  ") {
        Some(i) => {
            let account = &input[..i];
            let (quantity, symbol, input) = scan_amount(&input[i + 2..]);
            let (cost_q, cost_s, input) = match input.is_empty() {
                true => ("", "", ""),
                false => scan_cost(input),
            };

            // TODO: handle post comment
            // scan_xyz(input)

            return [account, quantity, symbol, cost_q, cost_s];
        }
        None => [input, "", "", "", ""],
    }
}

/// Scans the first Amount from the input and returns:
/// (Quantity, Symbol, remainder)
///
fn scan_amount(input: &str) -> (&str, &str, &str) {
    let input = input.trim_start();
    
    // Check the next character
    let c = *input.chars().peekable().peek().expect("A valid character");

    if c.is_digit(10) || c == '-' || c == '.' || c == ',' {
        // scan_amount_number_first(input)
        let (quantity, input) = scan_quantity(input);
        let (symbol, input) = scan_symbol(input);
        (quantity, symbol, input)
    } else {
        // scan_amount_symbol_first(input)
        let (symbol, input) = scan_symbol(input);
        let (quantity, input) = scan_quantity(input);
        (quantity, symbol, input)
    }
}

/// Reads the quantity string.
/// Returns [quantity, remainder]
fn scan_quantity(input: &str) -> (&str, &str) {
    for (i, c) in input.char_indices() {
        // stop if an invalid number character encountered.
        if c.is_digit(10) || c == '-' || c == '.' || c == ',' {
            // continue
        } else {
            return (&input[..i], &input[i..].trim_start());
        }
    }
    // else, return the full input.
    (input, "")
}

/// Scans the symbol in the input string.
/// Returns (symbol, remainder)
fn scan_symbol(input: &str) -> (&str, &str) {
    let input = input.trim_start();

    // TODO: check for valid double quotes

    for (i, c) in input.char_indices() {
        // Return when a separator or a number is found.
        if c.is_whitespace() || c == '@' || c.is_digit(10) || c == '-' {
            return (&input[..i], &input[i..].trim_start());
        }
    }
    // else return the whole input.
    (input, "")
}

/// Scans the cost
///
/// @ AMOUNT or @@ AMOUNT
///
/// The first is per-unit cost and the second is the total cost.
/// Returns
/// [quantity, symbol, remainder]
fn scan_cost(input: &str) -> (&str, &str, &str) {
    // @ or () or @@
    if input.chars().peekable().peek() != Some(&'@') {
        return ("", "", "");
    }

    // We have a price.
    // () is a virtual cost. Ignore for now.

    if input.chars().nth(1) != Some('@') {
        // per-unit cost
        let input = &input[2..].trim_start();
        let (quantity, symbol, input) = scan_amount(input);
        (quantity, symbol, input)
    } else {
        todo!("total cost")
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
    use crate::scanner::scan_amount;

    use super::{scan_post, scan_symbol};

    #[test]
    fn test_tokenize_post_full() {
        let input = "  Assets  20 VEUR @ 25.6 EUR";

        // Act
        let tokens = scan_post(input);

        // Assert
        let mut iterator = tokens.into_iter();

        assert_eq!("Assets", iterator.next().unwrap());
        assert_eq!("20", iterator.next().unwrap());
        assert_eq!("VEUR", iterator.next().unwrap());
        assert_eq!("25.6", iterator.next().unwrap());
        assert_eq!("EUR", iterator.next().unwrap());
    }

    #[test]
    fn test_tokenize_post_w_amount() {
        let input = "Assets  20 EUR";

        // Act
        let tokens = scan_post(input);

        // Assert
        let mut iterator = tokens.into_iter();

        assert_eq!("Assets", iterator.next().unwrap());
        assert_eq!("20", iterator.next().unwrap());
        assert_eq!("EUR", iterator.next().unwrap());
        assert_eq!("", iterator.next().unwrap());
        assert_eq!("", iterator.next().unwrap());
        assert_eq!(None, iterator.next());
    }

    #[test]
    fn test_tokenize_post_quantity_only() {
        let input = "Assets  20";

        // Act
        let tokens = scan_post(input);

        // Assert
        let mut iterator = tokens.into_iter();

        assert_eq!("Assets", iterator.next().unwrap());
        assert_eq!("20", iterator.next().unwrap());
    }

    #[test]
    fn test_tokenize_post_account() {
        let input = "  Assets";

        // Act
        let tokens = scan_post(input);

        // Assert
        let mut iterator = tokens.into_iter();

        assert_eq!("Assets", iterator.next().unwrap());
        assert_eq!("", iterator.next().unwrap());
    }

    #[test]
    fn test_tokenize_amount() {
        let input = "  Assets  25 EUR";

        let actual = scan_post(input);

        assert_eq!("25", actual[1]);
        assert_eq!("EUR", actual[2]);
        assert_eq!("", actual[3]);
        assert_eq!("", actual[4]);
    }

    #[test]
    fn test_tokenize_neg_amount() {
        let input = "  Expenses  -25 EUR";

        let actual = scan_post(input);

        assert_eq!("-25", actual[1]);
        assert_eq!("EUR", actual[2]);
    }

    #[test]
    fn test_tokenize_amount_dec_sep() {
        let input = "  Expenses  25.0 EUR";

        let actual = scan_post(input);

        assert_eq!("25.0", actual[1]);
        assert_eq!("EUR", actual[2]);
    }

    #[test]
    fn test_tokenize_amount_th_sep() {
        let input = "  Expenses  25,00 EUR";

        let actual = scan_post(input);

        assert_eq!("25,00", actual[1]);
        assert_eq!("EUR", actual[2]);
    }

    #[test]
    fn test_tokenize_amount_all_sep() {
        let input = "  Expenses  25,0.01 EUR";

        let actual = scan_post(input);

        assert_eq!("25,0.01", actual[1]);
        assert_eq!("EUR", actual[2]);
    }

    #[test]
    fn test_tokenize_amount_symbol_first() {
        let input = "  Expenses  €25";

        let actual = scan_post(input);

        assert_eq!("25", actual[1]);
        assert_eq!("€", actual[2]);
    }

    #[test]
    fn test_scan_amount_number_first_ws() {
        let input = "  Expenses  25,0.01 EUR";
        let actual = scan_post(input);

        assert_eq!("Expenses", actual[0]);
        assert_eq!("25,0.01", actual[1]);
        assert_eq!("EUR", actual[2]);
        assert_eq!("", actual[3]);
        assert_eq!("", actual[4]);
    }

    #[test]
    fn test_scan_amount_number_first() {
        let input = "  Expenses  25,0.01EUR";

        let actual = scan_post(input);

        assert_eq!("Expenses", actual[0]);
        assert_eq!("25,0.01", actual[1]);
        assert_eq!("EUR", actual[2]);
        assert_eq!("", actual[3]);
        assert_eq!("", actual[4]);
    }

    #[test]
    fn test_scan_amount_symbol_first_ws() {
        let input = "EUR 25,0.01";

        let (quantity, symbol, rest) = scan_amount(input);

        assert_eq!("25,0.01", quantity);
        assert_eq!("EUR", symbol);
    }

    #[test]
    fn test_scan_amount_symbol_first() {
        let input = "EUR25,0.01";

        let (quantity, symbol, rest) = scan_amount(input);

        assert_eq!("25,0.01", quantity);
        assert_eq!("EUR", symbol);
    }

    #[test]
    fn test_scan_amount_symbol_first_neg() {
        let input = "EUR-25,0.01";

        let (quantity, symbol, rest) = scan_amount(input);

        assert_eq!("-25,0.01", quantity);
        assert_eq!("EUR", symbol);
        // assert_eq!("", actual[2]);
        // assert_eq!("", actual[3]);
    }

    #[test]
    fn test_scan_quantity_full() {
        let input = "5 VECP @ 13.68 EUR";

        let (quantity, symbol, remainder) = scan_amount(input);

        assert_eq!("5", quantity);
        assert_eq!("VECP", symbol);
        assert_eq!("@ 13.68 EUR", remainder);
    }

    #[test]
    fn test_scan_symbol_quotes() {
        let input = " \"VECP\" @ 13.68 EUR";

        let (actual, remainder) = scan_symbol(input);

        assert_eq!("\"VECP\"", actual);
        assert_eq!("@ 13.68 EUR", remainder);
    }

    #[test]
    fn test_scan_symbol() {
        let input = " VECP @ 13.68 EUR";

        let (actual, remainder) = scan_symbol(input);

        assert_eq!("VECP", actual);
        assert_eq!("@ 13.68 EUR", remainder);
    }

    #[test]
    fn test_scan_symbol_only() {
        let input = " VECP ";

        let (actual, remainder) = scan_symbol(input);

        assert_eq!("VECP", actual);
        assert_eq!("", remainder);
    }

    #[test]
    fn test_scanning_cost() {
        let input = "  Account  5 VAS @ 13.21 AUD";

        let actual = scan_post(input);

        // Check that the cost has been scanned
        assert_eq!("Account", actual[0]);
        assert_eq!("5", actual[1]);
        assert_eq!("VAS", actual[2]);
        assert_eq!("13.21", actual[3]);
        assert_eq!("AUD", actual[4]);
    }
}

#[cfg(test)]
mod amount_tests {
    use super::scan_cost;

    #[test]
    fn test_scanning_costs() {
        let input = "@ 25.86 EUR";

        let (quantity, symbol, remainder) = scan_cost(input);

        assert_eq!("25.86", quantity);
        assert_eq!("EUR", symbol);
    }
}
