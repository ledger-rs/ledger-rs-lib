/*!
 * Scanner scans the input text and returns tokens (groups of characters) back for parsing.
 * Scans/tokenizes the journal files.
 * There are scanner functions for every element of the journal.
 */

pub(crate) struct PostTokens<'a> {
    pub account: &'a str,
    pub quantity: &'a str,
    pub symbol: &'a str,
    pub cost_quantity: &'a str,
    pub cost_symbol: &'a str,
    pub is_per_unit: bool,
}

/// Structure for the tokens from scanning the Amount part of the Posting.
struct AmountTokens<'a> {
    pub quantity: &'a str,
    pub symbol: &'a str,
    /// Any remaining content
    pub remainder: &'a str
}

struct CostTokens<'a> {
    pub quantity: &'a str,
    pub symbol: &'a str,
    pub is_per_unit: bool,
    pub remainder: &'a str,
}

impl<'a> CostTokens<'a> {
    pub fn new() -> Self {
        Self {
            quantity: "",
            symbol: "",
            is_per_unit: false,
            remainder: "",
        }
    }
}

/// Parse Xact header record.
/// 2023-05-05=2023-05-01 Payee  ; Note
///
/// returns [date, aux_date, payee, note]
///
/// Check for .is_empty() after receiving the result and handle appropriately.
///
/// Ledger's documentation specifies the following format
///
/// DATE[=EDATE] [*|!] [(CODE)] DESC
///
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
/// returns (account, quantity, symbol, cost_q, cost_s, is_per_unit)
/// 
/// Reference methods:
/// - amount_t::parse
/// 
pub(crate) fn scan_post(input: &str) -> PostTokens {
    // clear the initial whitespace.
    let input = input.trim_start();

    // todo: state = * cleared, ! pending

    if input.is_empty() || input.chars().nth(0) == Some(';') {
        panic!("Posting has no account")
    }

    // todo: virtual, deferred account [] () <>

    // two spaces is a separator betweer the account and amount.
    // Eventually, also support the tab as a separator:
    // something like |p| p == "  " || p  == '\t'

    let Some(sep_index) = input.find("  ") else {
        return PostTokens {
            account: input.trim_end(),
            quantity: "",
            symbol: "",
            cost_quantity: "",
            cost_symbol: "",
            is_per_unit: false,
        }
    };

    // there's more content

    let account = &input[..sep_index];
    let amount_tokens = scan_amount(&input[sep_index + 2..]);
    let cost_tokens = match input.is_empty() {
        true => CostTokens::new(),
        false => scan_cost(input),
    };

    // TODO: handle post comment
    // scan_xyz(input)

    return PostTokens {
        account,
        quantity: amount_tokens.quantity,
        symbol: amount_tokens.symbol,
        cost_quantity: cost_tokens.quantity,
        cost_symbol: cost_tokens.symbol,
        is_per_unit: cost_tokens.is_per_unit,
    };
}

/// Scans the first Amount from the input
/// returns:
/// (Quantity, Symbol, remainder)
///
fn scan_amount(input: &str) -> AmountTokens {
    let input = input.trim_start();

    // Check the next character
    let c = *input.chars().peekable().peek().expect("A valid character");

    if c.is_digit(10) || c == '-' || c == '.' || c == ',' {
        // scan_amount_number_first(input)
        let (quantity, input) = scan_quantity(input);
        let (symbol, input) = scan_symbol(input);
        AmountTokens {
            quantity,
            symbol,
            remainder: input,
        }
    } else {
        // scan_amount_symbol_first(input)
        let (symbol, input) = scan_symbol(input);
        let (quantity, input) = scan_quantity(input);
        AmountTokens {
            quantity,
            symbol,
            remainder: input,
        }
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
/// [quantity, symbol, remainder, is_per_unit]
fn scan_cost(input: &str) -> CostTokens {
    // @ or () or @@
    if input.chars().peekable().peek() != Some(&'@') {
        return CostTokens {
            quantity: "",
            symbol: "",
            is_per_unit: false,
            remainder: "",
        };
    }

    // We have a price.
    // () is a virtual cost. Ignore for now.

    let (first_char, is_per_unit) = if input.chars().nth(1) != Some('@') {
        // per-unit cost
        (2, true)
    } else {
        // total cost
        (3, false)
    };
    let input = &input[first_char..].trim_start();
    let amount_tokens = scan_amount(input);

    CostTokens {
        quantity: amount_tokens.quantity,
        symbol: amount_tokens.symbol,
        is_per_unit,
        remainder: input,
    }
}

/// Scans the Price directive
///
/// i.e.  
/// P 2022-03-03 13:00:00 EUR 1.12 USD
///
/// returns [date, time, commodity, quantity, price_commodity]
pub(crate) fn scan_price_directive(input: &str) -> [&str; 5] {
    // Skip the starting P and whitespace.
    let input = input[1..].trim_start();

    // date
    let (date, input) = scan_price_element(input);

    // time
    let input = input.trim_start();
    let (time, input) = match input.chars().peekable().peek().unwrap().is_digit(10) {
        // time
        true => scan_price_element(input),
        // no time
        false => ("", input),
    };

    // commodity
    let input = input.trim_start();
    let (commodity, input) = scan_price_element(input);

    // price, quantity
    let input = input.trim_start();
    let (quantity, input) = scan_price_element(input);

    // price, commodity
    let input = input.trim_start();
    let (price_commodity, _input) = scan_price_element(input);

    [date, time, commodity, quantity, price_commodity]
}

fn find_next_separator(input: &str) -> Option<usize> {
    input.find(|c| c == ' ' || c == '\t')
}

fn scan_price_element(input: &str) -> (&str, &str) {
    let Some(separator_index) = find_next_separator(input)
        else {
            return (input, "")
        };

    // date, rest
    (&input[..separator_index], &input[separator_index..])
}

/// identifies the next element, the content until the next separator.
fn next_element(input: &str) -> Option<&str> {
    // assuming the element starts at the beginning, no whitespace.
    if let Some(next_sep) = find_next_separator(input) {
        Some(&input[..next_sep])
    } else {
        None
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
    use super::{scan_post, scan_symbol};
    use crate::scanner::scan_amount;

    #[test]
    fn test_tokenize_post_full() {
        let input = "  Assets  20 VEUR @ 25.6 EUR";

        // Act
        let tokens = scan_post(input);

        // Assert
        assert_eq!("Assets", tokens.account);
        assert_eq!("20", tokens.quantity);
        assert_eq!("VEUR", tokens.symbol);
        assert_eq!("25.6", tokens.cost_quantity);
        assert_eq!("EUR", tokens.cost_symbol);
    }

    #[test]
    fn test_tokenize_post_w_amount() {
        let input = "Assets  20 EUR";

        // Act
        let tokens = scan_post(input);

        // Assert
        assert_eq!("Assets", tokens.account);
        assert_eq!("20", tokens.quantity);
        assert_eq!("EUR", tokens.symbol);
        assert_eq!("", tokens.cost_quantity);
        assert_eq!("", tokens.cost_symbol);
        assert_eq!(false, tokens.is_per_unit);
    }

    #[test]
    fn test_tokenize_post_quantity_only() {
        let input = "Assets  20";

        // Act
        let tokens = scan_post(input);

        // Assert
        assert_eq!("Assets", tokens.account);
        assert_eq!("20", tokens.quantity);
    }

    #[test]
    fn test_tokenize_post_account() {
        let input = "  Assets";

        // Act
        let tokens = scan_post(input);

        // Assert
        assert_eq!("Assets", tokens.account);
        assert_eq!("", tokens.quantity);
    }

    #[test]
    fn test_tokenize_amount() {
        let input = "  Assets  25 EUR";

        let tokens = scan_post(input);

        assert_eq!("25", tokens.quantity);
        assert_eq!("EUR", tokens.symbol);
        assert_eq!("", tokens.cost_quantity);
        assert_eq!("", tokens.cost_symbol);
    }

    #[test]
    fn test_tokenize_neg_amount() {
        let input = "  Expenses  -25 EUR";

        let actual = scan_post(input);

        assert_eq!("-25", actual.quantity);
        assert_eq!("EUR", actual.symbol);
    }

    #[test]
    fn test_tokenize_amount_dec_sep() {
        let input = "  Expenses  25.0 EUR";

        let actual = scan_post(input);

        assert_eq!("25.0", actual.quantity);
        assert_eq!("EUR", actual.symbol);
    }

    #[test]
    fn test_tokenize_amount_th_sep() {
        let input = "  Expenses  25,00 EUR";

        let actual = scan_post(input);

        assert_eq!("25,00", actual.quantity);
        assert_eq!("EUR", actual.symbol);
    }

    #[test]
    fn test_tokenize_amount_all_sep() {
        let input = "  Expenses  25,0.01 EUR";

        let actual = scan_post(input);

        assert_eq!("25,0.01", actual.quantity);
        assert_eq!("EUR", actual.symbol);
    }

    #[test]
    fn test_tokenize_amount_symbol_first() {
        let input = "  Expenses  €25";

        let actual = scan_post(input);

        assert_eq!("25", actual.quantity);
        assert_eq!("€", actual.symbol);
    }

    #[test]
    fn test_scan_amount_number_first_ws() {
        let input = "  Expenses  25,0.01 EUR";

        let actual = scan_post(input);

        assert_eq!("Expenses", actual.account);
        assert_eq!("25,0.01", actual.quantity);
        assert_eq!("EUR", actual.symbol);
        assert_eq!("", actual.cost_quantity);
        assert_eq!("", actual.cost_symbol);
    }

    #[test]
    fn test_scan_amount_number_first() {
        let input = "  Expenses  25,0.01EUR";

        let tokens = scan_post(input);

        assert_eq!("Expenses", tokens.account);
        assert_eq!("25,0.01", tokens.quantity);
        assert_eq!("EUR", tokens.symbol);
        assert_eq!("", tokens.cost_quantity);
        assert_eq!("", tokens.cost_symbol);
    }

    #[test]
    fn test_scan_amount_symbol_first_ws() {
        let input = "EUR 25,0.01";

        let tokens = scan_amount(input);

        assert_eq!("25,0.01", tokens.quantity);
        assert_eq!("EUR", tokens.symbol);
    }

    #[test]
    fn test_scan_amount_symbol_first() {
        let input = "EUR25,0.01";

        let tokens = scan_amount(input);

        assert_eq!("25,0.01", tokens.quantity);
        assert_eq!("EUR", tokens.symbol);
    }

    #[test]
    fn test_scan_amount_symbol_first_neg() {
        let input = "EUR-25,0.01";

        let tokens = scan_amount(input);

        assert_eq!("-25,0.01", tokens.quantity);
        assert_eq!("EUR", tokens.symbol);
        // assert_eq!("", actual[2]);
        // assert_eq!("", actual[3]);
    }

    #[test]
    fn test_scan_quantity_full() {
        let input = "5 VECP @ 13.68 EUR";

        let tokens = scan_amount(input);

        assert_eq!("5", tokens.quantity);
        assert_eq!("VECP", tokens.symbol);
        assert_eq!("@ 13.68 EUR", tokens.remainder);
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

        let tokens = scan_post(input);

        // Check that the cost has been scanned
        assert_eq!("Account", tokens.account);
        assert_eq!("5", tokens.quantity);
        assert_eq!("VAS", tokens.symbol);
        assert_eq!("13.21", tokens.cost_quantity);
        assert_eq!("AUD", tokens.cost_symbol);
        assert_eq!(true, tokens.is_per_unit);
    }

    #[test]
    fn test_scanning_total_cost() {
        let input = "  Account  5 VAS @@ 10 AUD";

        let tokens = scan_post(input);

        // Check that the cost has been scanned
        assert_eq!("Account", tokens.account);
        assert_eq!("5", tokens.quantity);
        assert_eq!("VAS", tokens.symbol);
        assert_eq!("10", tokens.cost_quantity);
        assert_eq!("AUD", tokens.cost_symbol);
    }

    // TODO: #[test]
    fn test_scan_sale_lot() {
        let input = "    Assets:Stocks  -10 VEUR {20 EUR} [2023-04-01] @ 25 EUR";

        let tokens = scan_post(input);

        // Assert
        assert_eq!("Assets:Stocks", tokens.account);
        assert_eq!("-10", tokens.quantity);
        assert_eq!("25", tokens.cost_quantity);
        assert_eq!("EUR", tokens.cost_symbol);
    }
}

#[cfg(test)]
mod scanner_tests_amount {
    use super::scan_cost;

    #[test]
    fn test_scanning_costs() {
        let input = "@ 25.86 EUR";

        let tokens = scan_cost(input);

        assert_eq!("25.86", tokens.quantity);
        assert_eq!("EUR", tokens.symbol);
        assert_eq!(true, tokens.is_per_unit);
        assert_eq!("", tokens.remainder);
    }

    #[test]
    fn test_scanning_cost_full() {
        let input = "@@ 25.86 EUR";

        let tokens = scan_cost(input);

        assert_eq!("25.86", tokens.quantity);
        assert_eq!("EUR", tokens.symbol);
        assert_eq!(false, tokens.is_per_unit);
        assert_eq!("", tokens.remainder);
    }
}

#[cfg(test)]
mod scanner_tests_price_directive {
    use super::scan_price_directive;

    #[test]
    fn test_scan_price_directive() {
        let line = "P 2022-03-03 13:00:00 EUR 1.12 USD";

        let actual = scan_price_directive(line);

        assert_eq!("2022-03-03", actual[0]);
        assert_eq!("13:00:00", actual[1]);
        assert_eq!("EUR", actual[2]);
        assert_eq!("1.12", actual[3]);
        assert_eq!("USD", actual[4]);
    }
}
