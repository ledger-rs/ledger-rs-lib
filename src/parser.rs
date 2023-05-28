use core::panic;
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
    env,
    io::{BufRead, BufReader, Read},
    path::PathBuf,
    str::FromStr,
    todo,
};

use chrono::NaiveDate;

use crate::{
    account::Account,
    amount::Amount,
    commodity::Commodity,
    journal::{Journal, XactIndex},
    post::Post,
    scanner,
    xact::Xact,
};

// pub(crate) fn read<T: Read>(source: T) -> Journal {
//     let mut parser = Parser::new(source);
//     parser.parse();
//     parser.journal
// }

pub(crate) fn read_into_journal<T: Read>(source: T, journal: &mut Journal) {
    let mut parser = Parser::new(source, journal);

    parser.parse();
}

pub(crate) fn parse_date(date_str: &str) -> NaiveDate {
    // todo: support more date formats?

    NaiveDate::parse_from_str(date_str, "%Y-%m-%d").expect("date parsed")
}

struct Parser<'j, T: Read> {
    pub journal: &'j mut Journal,

    reader: BufReader<T>,
    buffer: String,
}

impl<'j, T: Read> Parser<'j, T> {
    pub fn new(source: T, journal: &'j mut Journal) -> Self {
        let reader = BufReader::new(source);
        // To avoid allocation, reuse the String variable.
        let buffer = String::new();

        Self {
            reader,
            buffer,
            journal,
        }
    }

    pub fn parse(&mut self) {
        loop {
            match self.reader.read_line(&mut self.buffer) {
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

                    match self.read_next_directive() {
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
            self.buffer.clear();
        }
    }

    fn read_next_directive(&mut self) -> Result<(), String> {
        // if self.buffer.is_empty() {
        //     return Ok(());
        // }
        // let length = self.buffer.len();
        // log::debug!("line length: {:?}", length);
        if self.buffer == "\r\n" || self.buffer == "\n" {
            return Ok(());
        }

        // determine what the line is
        match self.buffer.chars().peekable().peek().unwrap() {
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
                self.xact_directive();
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

                    c => {
                        log::warn!("not handled: {:?}", c);
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

    /// textual.cc
    /// bool instance_t::general_directive(char *line)
    fn general_directive(&self) -> bool {
        // todo: skip if (*p == '@' || *p == '!')

        // split directive and argument
        let mut iter = self.buffer.split_whitespace();
        let Some(directive) = iter.next() else { panic!("no directive?") };
        let argument = iter.next();

        // todo: check arguments for directives that require one
        // match directive {
        //     "comment" | "end" | "python" | "test" | "year" | "Y" => {
        //         //
        //         // Year can also be specified with one letter?
        //         ()
        //     }
        //     _ => {
        //         panic!("The directive {:?} requires an argument!", directive);
        //     }
        // }

        match directive.chars().peekable().peek().unwrap() {
            'a' => {
                todo!("a");
            }

            // bcde
            'i' => match self.buffer.as_str() {
                "include" => {
                    self.include_directive(argument.unwrap());
                    return true;
                }
                "import" => {
                    todo!("import directive")
                }
                _ => (),
            },

            // ptvy
            _ => {
                todo!("handle")
            }
        }

        // lookup(DIRECTIVE, self.buffer)

        false
    }

    fn xact_directive(&mut self) {
        let tokens = scanner::tokenize_xact_header(&self.buffer);
        let xact = Xact::create(tokens[0], tokens[1], tokens[2], tokens[3]);
        // Add xact to the journal
        let xact_index = self.journal.add_xact(xact);

        // Read the Xact contents (Posts, Comments, etc.)
        // Read until separator (empty line).
        loop {
            self.buffer.clear(); // empty the buffer before reading
            match self.reader.read_line(&mut self.buffer) {
                Err(e) => {
                    println!("Error: {:?}", e);
                    break;
                }
                Ok(0) => {
                    // end of file
                    log::debug!("0-length buffer");
                    break;
                }
                Ok(_) => {
                    if self.buffer.is_empty() {
                        // panic!("Unexpected whitespace at the beginning of line!")
                        todo!("Check what happens here")
                    }

                    // parse line
                    match self.buffer.chars().peekable().peek() {
                        Some(' ') => {
                            // valid line, starts with space.
                            let input = self.buffer.trim_start();

                            // if the line is blank after trimming, exit (end the transaction).
                            if input.is_empty() {
                                break;
                            }

                            // Process the Xact content line. Could be a Comment or a Post.
                            match input.chars().peekable().peek() {
                                Some(';') => {
                                    todo!("trailing note")
                                }
                                _ => {
                                    parse_post(input, xact_index, &mut self.journal);
                                }
                            }
                        }
                        Some('\r') | Some('\n') => {
                            // empty line "\r\n". Exit.
                            break;
                        }
                        _ => {
                            panic!("should not happen")
                        }
                    }
                }
            }

            // empty the buffer before exiting.
            self.buffer.clear();
        }

        // "finalize" transaction
        crate::xact::finalize(xact_index, &mut self.journal);
    }

    fn include_directive(&self, argument: &str) {
        let mut filename: PathBuf;

        // if (line[0] != '/' && line[0] != '\\' && line[0] != '~')
        if argument.starts_with('/') || argument.starts_with('\\') || argument.starts_with('~') {
            filename = PathBuf::from_str(argument).unwrap();
        } else {
            // relative path
            // TODO: get the parent path?
            // dir = parent_path()
            // if (parent_path.empty())
            // else, use current directory
            filename = env::current_dir().unwrap();

            filename.set_file_name(argument);
        }

        let mut file_found = false;
        let parent_path = filename.parent().unwrap();
        if parent_path.exists() {
            if filename.is_file() {
                // let base = filename.file_name();

                // TODO: read file
                todo!("read file")
            }
        }

        if !file_found {
            panic!("Include file not found");
        }
    }
}

/// Parses Post from the buffer, adds it to the Journal and links
/// to Xact, Account, etc.
fn parse_post(input: &str, xact_index: XactIndex, journal: &mut Journal) {
    let tokens = scanner::scan_post(input);

    let account_index;
    {
        // Create Account, add to collection
        let account = Account::parse(tokens[0]);
        account_index = journal.add_account(account);
    }

    let commodity_index;
    {
        // Create Commodity, add to collection
        let symbol = tokens[2];

        // commodity_pool.find(symbol)
        let commodity = journal.commodity_pool.find(symbol);
        // let commodity = Commodity::parse(symbol);
        commodity_index = match commodity {
            Some(c) => Some(journal.commodity_pool.create(symbol)),
            None => None,
        };
    }

    // create amount
    let amount = Amount::parse(tokens[1], commodity_index);

    // TODO: handle cost (2nd amount)
    let price_commodity_index;
    {
        let symbol = tokens[4];

        // let commodity = Commodity::parse(symbol);
        let commodity = journal.commodity_pool.find(symbol);

        todo!("check this case");

        // price_commodity_index = match commodity {
        //     Some(c) => Some(journal.commodity_pool.add_commodity(c)),
        //     None => None,
        // }
    }
    let cost = Amount::parse(tokens[3], price_commodity_index);

    let post_index;
    {
        // Create Post, link Xact, Account, Commodity
        let post = Post::new(account_index, xact_index, amount, cost);
        post_index = journal.add_post(post);
    }

    // add Post to Account.posts
    {
        let account = journal.accounts.get_mut(account_index).unwrap();
        account.post_indices.push(post_index);
    }

    {
        // add Post to Xact.
        let xact = journal.xacts.get_mut(xact_index).unwrap();
        xact.posts.push(post_index);
    }
}

#[cfg(test)]
mod tests {
    use std::{io::Cursor, todo};

    use crate::journal::Journal;

    use super::Parser;

    #[test]
    fn test_general_directive() {
        let source = Cursor::new("include some-file.ledger");
        let mut journal = Journal::new();

        let parser = Parser::new(source, &mut journal);

        parser.general_directive();

        todo!("assert")
    }

    /// A transaction record, after which comes a line with spaces only.
    /// This should be parseable.
    #[test]
    fn test_xact_with_space_after() {
        let src = r#";
2023-05-05 Payee
    Expenses  25 EUR
    Assets
            
"#;
        let source = Cursor::new(src);
        let mut journal = Journal::new();
        let mut parser = Parser::new(source, &mut journal);

        // Act
        parser.parse();

        // Assert
        assert_eq!(2, journal.accounts.len());
    }
}

#[cfg(test)]
mod full_tests {
    use std::io::Cursor;

    use crate::{journal::Journal, parser::read_into_journal};

    #[test]
    fn test_minimal_parsing() {
        let input = r#"; Minimal transaction
2023-04-10 Supermarket
    Expenses  20
    Assets
"#;
        let cursor = Cursor::new(input);
        let mut journal = Journal::new();

        // Act
        super::read_into_journal(cursor, &mut journal);

        // Assert
        assert_eq!(1, journal.xacts.len());

        let xact = journal.xacts.first().unwrap();
        assert_eq!("Supermarket", xact.payee);
        assert_eq!(2, xact.posts.len());

        let post1 = &journal.posts[xact.posts[0]];
        assert_eq!("Expenses", journal.get_account(post1.account_index).name);
        assert_eq!("20", post1.amount.as_ref().unwrap().quantity.to_string());
        assert_eq!(None, post1.amount.as_ref().unwrap().commodity_index);

        // let post_2 = xact.posts.iter().nth(1).unwrap();
        let post2 = &journal.posts[xact.posts[1]];
        assert_eq!("Assets", journal.get_account(post2.account_index).name);
    }

    #[test]
    fn test_multiple_currencies_one_xact() {
        let input = r#";
2023-05-05 Payee
    Assets:Cash EUR  -25 EUR
    Assets:Cash USD   30 USD
"#;
        let cursor = Cursor::new(input);
        let mut journal = Journal::new();

        // Act
        read_into_journal(cursor, &mut journal);

        // Assert
        assert_eq!(2, journal.commodity_pool.commodities.len());
    }
}

#[cfg(test)]
mod parser_tests {
    use std::{assert_eq, io::Cursor};

    use crate::{journal::Journal, parser::{self, read_into_journal}};

    #[test]
    fn test_minimal_parser() {
        let input = r#"; Minimal transaction
2023-04-10 Supermarket
    Expenses  20
    Assets
"#;
        let cursor = Cursor::new(input);
        let mut journal = Journal::new();

        // Act
        parser::read_into_journal(cursor, &mut journal);

        // Assert

        assert_eq!(1, journal.xacts.len());

        let xact = journal.xacts.first().unwrap();
        assert_eq!("Supermarket", xact.payee);
        assert_eq!(2, xact.posts.len());

        // let exp_account = journal.get
        let post1 = &journal.posts[xact.posts[0]];
        assert_eq!("Expenses", journal.get_account(post1.account_index).name);
        assert_eq!("20", post1.amount.as_ref().unwrap().quantity.to_string());
        assert_eq!(None, post1.amount.as_ref().unwrap().commodity_index);

        // let post_2 = xact.posts.iter().nth(1).unwrap();
        let post2 = &journal.posts[xact.posts[1]];
        assert_eq!("Assets", journal.get_account(post2.account_index).name);
    }

    #[test]
    fn test_parse_standard_xact() {
        let input = r#"; Standard transaction
2023-04-10 Supermarket
    Expenses  20 EUR
    Assets
"#;
        let cursor = Cursor::new(input);
        let mut journal = Journal::new();

        super::read_into_journal(cursor, &mut journal);

        // Assert
        // Xact
        assert_eq!(1, journal.xacts.len());

        if let Some(xact) = journal.xacts.first() {
            // assert!(xact.is_some());

            assert_eq!("Supermarket", xact.payee);

            // Posts
            let posts = journal.get_posts(&xact.posts);
            assert_eq!(2, posts.len());

            let acc1 = journal.get_account(posts[0].account_index);
            assert_eq!("Expenses", acc1.name);
            let acc2 = journal.get_account(posts[1].account_index);
            assert_eq!("Assets", acc2.name);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_parse_trade_xact() {
        // Arrange
        let input = r#"; Standard transaction
2023-04-10 Supermarket
    Assets:Investment  20 VEUR @ 10 EUR
    Assets
"#;
        let cursor = Cursor::new(input);
        let mut journal = Journal::new();

        // Act
        read_into_journal(cursor, &mut journal);

        // Assert
        let xact = journal.xacts.first().unwrap();
        assert_eq!("Supermarket", xact.payee);

        let posts = journal.get_posts(&xact.posts);
        assert_eq!(2, posts.len());

        // post 1
        let p1 = posts[0];
        assert_eq!(
            "Assets:Investment",
            journal.get_account(p1.account_index).name
        );
        // amount
        let Some(a1) = &p1.amount else {panic!()};
        assert_eq!("20", a1.quantity.to_string());
        let comm1 = journal.commodity_pool.commodity_history.get_commodity(a1.commodity_index.unwrap());
        assert_eq!("VEUR", comm1.symbol);
        let Some(ref cost1) = p1.cost else { panic!()};
        // cost
        assert_eq!("10", cost1.quantity.to_string());
        assert_eq!(
            "EUR",
            journal.commodity_pool.commodity_history.get_commodity(cost1.commodity_index.unwrap()).symbol
        );

        // post 2
        let p2 = posts[1];
        assert_eq!("Assets", journal.get_account(p2.account_index).name);
        // amount
        let Some(a2) = &p2.amount else {panic!()};
        assert_eq!("-20", a2.quantity.to_string());
        let comm2 = journal.commodity_pool.commodity_history.get_commodity(a2.commodity_index.unwrap());
        assert_eq!("VEUR", comm2.symbol);

        assert!(p2.cost.is_none());
    }

    #[test]
    fn test_parsing_account_tree() {
        let input = r#"
2023-05-23 Payee
  Assets:Eur   -20 EUR
  Assets:USD    30 USD
  Trading:Eur   20 EUR
  Trading:Usd  -30 USD
"#;

        let cursor = Cursor::new(input);
        let mut journal = Journal::new();

        // Act
        read_into_journal(cursor, &mut journal);

        // Assert
        assert_eq!(1, journal.xacts.len());
        assert_eq!(4, journal.posts.len());
        assert_eq!(4, journal.accounts.len());
        assert_eq!(2, journal.commodity_pool.commodities.len());
    }
}

#[cfg(test)]
mod amount_parsing_tests {
    use rust_decimal_macros::dec;

    use crate::{journal::Journal, parser::parse_post, xact::Xact, pool::CommodityIndex};

    use super::Amount;

    fn setup() -> Journal {
        let mut journal = Journal::new();
        let xact = Xact::create("2023-05-02", "", "Supermarket", "");
        let xact_index = journal.add_xact(xact);

        journal
    }

    #[test]
    fn test_positive_no_commodity() {
        let expected = Amount {
            quantity: dec!(20),
            commodity_index: None,
        };
        let actual = Amount::parse("20", None).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_negative_no_commodity() {
        let actual = Amount::parse("-20", None).unwrap();
        let expected = Amount {
            quantity: dec!(-20),
            commodity_index: None,
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_pos_w_commodity_separated() {
        let expected = Amount {
            quantity: dec!(20),
            commodity_index: Some(CommodityIndex::new(0)),
        };
        let mut journal = setup();

        // Act

        parse_post("  Assets  20 EUR", 0, &mut journal);
        let post = journal.posts.first().unwrap();
        let Some(amount) = &post.amount else { todo!() }; // else None;

        // assert!(actual.is_some());
        assert_eq!(expected, *amount);

        // commodity
        let c = journal.commodity_pool.commodity_history.get_commodity(amount.commodity_index.unwrap());
        assert_eq!("EUR", c.symbol);
    }

    #[test]
    fn test_neg_commodity_separated() {
        let expected = Amount {
            quantity: dec!(-20),
            commodity_index: Some(CommodityIndex::new(5)),
        };
        let mut journal = setup();

        // Act
        parse_post("  Assets  -20 EUR", 0, &mut journal);

        // Assert
        let post = journal.posts.first().unwrap();
        let Some(a) = &post.amount else { panic!() };
        let q = a.quantity;
        assert_eq!(dec!(-20), q);

        let commodity = journal.commodity_pool.commodity_history.get_commodity(a.commodity_index.unwrap());
        assert_eq!("EUR", commodity.symbol);
    }

    #[test]
    fn test_full_w_commodity_separated() {
        // Arrange
        let mut journal = setup();

        // Act
        parse_post("  Assets  -20000.00 EUR", 0, &mut journal);
        let post = journal.posts.first().unwrap();
        let Some(ref amount) = post.amount else { panic!()};

        // Assert
        assert_eq!("-20000.00", amount.quantity.to_string());
        assert_eq!(
            "EUR",
            journal
                .commodity_pool.commodity_history.get_commodity(amount.commodity_index.unwrap())
                .symbol
        );
    }

    #[test]
    fn test_full_commodity_first() {
        // Arrange
        let mut journal = setup();

        // Act
        parse_post("  Assets  A$-20000.00", 0, &mut journal);
        let post = journal.posts.first().unwrap();
        let Some(ref amount) = post.amount else { panic!()};

        // Assert
        assert_eq!("-20000.00", amount.quantity.to_string());
        assert_eq!(
            "A$",
            journal
                .commodity_pool.commodity_history
                .get_commodity(amount.commodity_index.unwrap())
                .symbol
        );
    }

    #[test]
    fn test_quantity_separators() {
        let input = "-1000000.00";
        let expected = dec!(-1_000_000);

        let amount = Amount::parse(input, None);
        assert!(amount.is_some());

        let actual = amount.unwrap().quantity;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_addition() {
        //let c1 = Commodity::new("EUR");
        let left = Amount::new(dec!(10), None);
        // let c2 = Commodity::new("EUR");
        let right = Amount::new(dec!(15), None);

        let actual = left + right;

        assert_eq!(dec!(25), actual.quantity);
        // assert!(actual.commodity.is_some());
        // assert_eq!("EUR", actual.commodity.unwrap().symbol);
    }

    #[test]
    fn test_add_assign() {
        // let c1 = Commodity::new("EUR");
        let mut actual = Amount::new(dec!(21), None);
        // let c2 = Commodity::new("EUR");
        let other = Amount::new(dec!(13), None);

        // actual += addition;
        actual.add(&other);

        assert_eq!(dec!(34), actual.quantity);
    }

    #[test]
    fn test_null_amount() {
        let input = " ";
        let actual = Amount::parse(input, None);

        assert_eq!(None, actual);
    }

    #[test]
    fn test_copy_from_no_commodity() {
        let other = Amount::new(dec!(10), None);
        let actual = Amount::copy_from(&other);

        assert_eq!(dec!(10), actual.quantity);
        // assert_eq!(None, actual.commodity);
    }
}
