/*!
 * Parser with iterators
 *
 * Parses string tokens into model entities (Account, Transaction, Post, Amount...)
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
use core::panic;
use std::{
    env,
    io::{BufRead, BufReader, Read},
    path::PathBuf,
    str::FromStr,
    todo,
};

use anyhow::Error;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use crate::{
    amount::{Amount, Quantity},
    annotate::Annotation,
    journal::Journal,
    post::Post,
    scanner::{self, PostTokens},
    xact::Xact, parse_file,
};

pub const ISO_DATE_FORMAT: &str = "%Y-%m-%d";
pub const ISO_TIME_FORMAT: &str = "%H:%M:%S";

pub(crate) fn read_into_journal<T: Read>(source: T, journal: &mut Journal) {
    let mut parser = Parser::new(source, journal);

    parser.parse();
}

/// Parses ISO-formatted date string, like 2023-07-23
pub(crate) fn parse_date(date_str: &str) -> NaiveDate {
    // todo: support more date formats?

    NaiveDate::parse_from_str(date_str, ISO_DATE_FORMAT).expect("date parsed")
}

/// Create DateTime from date string only.
pub fn parse_datetime(iso_str: &str) -> Result<NaiveDateTime, anyhow::Error> {
    Ok(NaiveDateTime::new(
        NaiveDate::parse_from_str(iso_str, ISO_DATE_FORMAT)?,
        NaiveTime::MIN,
    ))
}

pub fn parse_amount(amount_str: &str, journal: &mut Journal) -> Option<Amount> {
    let (tokens, _) = scanner::scan_amount(amount_str);
    parse_amount_parts(tokens.quantity, tokens.symbol, journal)
}

/// Parse amount parts (quantity, commodity), i.e. "25", "AUD".
/// Returns Amount.
/// Panics if parsing fails.
pub fn parse_amount_parts(
    quantity: &str,
    commodity: &str,
    journal: &mut Journal,
) -> Option<Amount> {
    // Create Commodity, add to collection
    let commodity_ptr = journal.commodity_pool.find_or_create(commodity, None);

    if let Some(quantity) = Quantity::from_str(quantity) {
        Some(Amount::new(quantity, Some(commodity_ptr)))
    } else {
        None
    }
}

pub(crate) struct Parser<'j, T: Read> {
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

    /// Parse given input.
    /// Fill the Journal with parsed elements.
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
                let _ = self.xact_directive();
            }

            ' ' | '\t' => {
                todo!("complete")
            }

            // The rest
            _ => {
                // 4.7.2 command directives

                if self.general_directive() {
                    return Ok(());
                }

                let c = 'P';
                match c {
                        // ACDNPY
                        'P' => {
                            // a pricing xact
                            self.price_xact_directive();
                        }

                        c => {
                            log::warn!("not handled: {:?}", c);
                            todo!("handle other directives");
                        }
                    }
                    // TODO: todo!("the rest")
            }
        }

        Ok(())
    }

    /// textual.cc
    /// bool instance_t::general_directive(char *line)
    fn general_directive(&mut self) -> bool {
        // todo: skip if (*p == '@' || *p == '!')

        // split directive and argument
        let mut iter = self.buffer.split_whitespace();
        let Some(directive) = iter.next() else {
            panic!("no directive?")
        };
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
            'i' => match directive {
                "include" => {
                    let own_argument = argument.unwrap().to_owned();
                    self.include_directive(&own_argument);
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

    fn price_xact_directive(&mut self) {
        // pass on to the commodity pool
        self.journal
            .commodity_pool
            .parse_price_directive(&self.buffer);
    }

    fn create_xact(&mut self) -> *const Xact {
        let tokens = scanner::tokenize_xact_header(&self.buffer);
        let xact = Xact::create(tokens[0], tokens[1], tokens[2], tokens[3]);

        // Add xact to the journal
        let new_ref = self.journal.add_xact(xact);

        new_ref
    }

    fn xact_directive(&mut self) -> Result<(), Error> {
        let xact_ptr = self.create_xact() as *mut Xact;

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
                                    self.parse_trailing_note(xact_ptr);
                                }
                                _ => {
                                    parse_post(input, xact_ptr, &mut self.journal)?;
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
        crate::xact::finalize(xact_ptr, &mut self.journal);

        Ok(())
    }

    /// textual.cc
    /// void instance_t::include_directive(char *line)
    fn include_directive(&mut self, argument: &str) {
        let mut filename: PathBuf;

        // if (line[0] != '/' && line[0] != '\\' && line[0] != '~')
        if argument.starts_with('/') || argument.starts_with('\\') || argument.starts_with('~') {
            filename = PathBuf::from_str(argument).unwrap();
        } else {
            // relative path
            /*
            filename = PathBuf::from_str(argument).unwrap();
            // get the parent path?
            match filename.parent() {
                Some(parent) => {
                    // absolute path
                    filename = parent.to_path_buf();
                },
                None => {
                    // no parent path, use current directory
                    filename = env::current_dir().unwrap();
                },
            }
            */
            filename = env::current_dir().unwrap();
            filename.push(argument);
        }

        // TODO: resolve glob, i.e *.ledger

        // let mut file_found = false;
        let parent_path = filename.parent().unwrap();
        if parent_path.exists() {
            if filename.is_file() {
                // let base = filename.file_name();

                // read file.
                parse_file(filename.to_str().unwrap(), self.journal);
            }
        }

        // if !file_found {
        //     panic!("Include file not found");
        // }
    }

    /// Parses the trailing note from the buffer.
    /// xact_index = The index of the current transaction, being parsed.
    /// The note is added either to the transaction or the last post, based on it's position.
    ///
    fn parse_trailing_note(&mut self, xact_ptr: *mut Xact) {
        // This is a trailing note, and possibly a metadata info tag
        // It is added to the previous element (xact/post).

        let note = self.buffer.trim_start();
        // The note starts with the comment character `;`.
        let note = note[1..].trim();
        if note.is_empty() {
            return;
        }

        // let xact = self.journal.xacts.get_mut(xact_index).unwrap();
        let xact: &mut Xact;
        unsafe {
            xact = &mut *xact_ptr;
        }
        if xact.posts.is_empty() {
            // The first comment. Add to the xact.
            // let xact_mut = self.journal.xacts.get_mut(xact_index).unwrap();
            xact.add_note(note);
        } else {
            // Post comment. Add to the previous posting.
            // let last_post_index = xact.posts.last().unwrap();
            let last_post = xact.posts.last_mut().unwrap();
            // let post = self.journal.get_post_mut(*last_post_index);
            last_post.add_note(note);
        }
    }
}

/// Parses Post from the buffer, adds it to the Journal and links
/// to Xact, Account, etc.
fn parse_post(input: &str, xact_ptr: *const Xact, journal: &mut Journal) -> Result<(), Error> {
    let tokens = scanner::scan_post(input);

    // TODO: Make this more like Ledger now that we have pointers.

    // Create Account, add to collection
    let account_ptr = journal.register_account(tokens.account).unwrap();

    // create amount
    let amount_opt = parse_amount_parts(tokens.quantity, tokens.symbol, journal);

    // parse and add annotations.
    {
        let annotation = Annotation::parse(
            tokens.price_date,
            tokens.price_quantity,
            tokens.price_commodity,
            journal,
        )?;

        // TODO: if the cost price is total (not per unit)
        // details.price /= amount

        // store annotation
        journal
            .commodity_pool
            .find_or_create(tokens.symbol, Some(annotation));
    }

    // handle cost (2nd amount)
    let cost_option = parse_cost(&tokens, &amount_opt, journal);

    // note
    // TODO: parse note
    let note = None;

    // Create Post, link Xact, Account, Commodity
    let post_ref: &Post;
    {
        let post: Post;
        post = Post::new(account_ptr, xact_ptr, amount_opt, cost_option, note);

        // add Post to Xact.
        // let xact = journal.xacts.get_mut(xact_ptr).unwrap();
        let xact: &mut Xact;
        unsafe {
            xact = &mut *(xact_ptr.cast_mut());
        }
        // xact.post_indices.push(post_index);
        post_ref = xact.add_post(post);
    }

    // add Post to Account.posts
    {
        //let account = journal.accounts.get_mut(account_ptr).unwrap();
        let account = journal.get_account_mut(account_ptr);
        account.posts.push(post_ref);
    }

    Ok(())
}

fn parse_cost(
    tokens: &PostTokens,
    amount: &Option<Amount>,
    journal: &mut Journal,
) -> Option<Amount> {
    if tokens.cost_quantity.is_empty() || amount.is_none() {
        return None;
    }

    // parse cost (per-unit vs total)
    let cost_result = parse_amount_parts(tokens.cost_quantity, tokens.cost_symbol, journal);
    if cost_result.is_none() {
        return None;
    }
    let mut cost = cost_result.unwrap();

    if tokens.is_per_unit {
        // per-unit cost
        let mut cost_val = cost;
        cost_val *= amount.unwrap();
        cost = cost_val;
    }
    // Total cost is already the end-value.

    Some(cost)
}

#[cfg(test)]
mod tests {
    use std::{io::Cursor, todo};

    use super::Parser;
    use crate::journal::Journal;

    /// Enable this test again when the functionality is complete
    #[test]
    fn test_general_directive() {
        let source = Cursor::new("include tests/basic.ledger");
        let mut journal = Journal::new();

        let mut parser = Parser::new(source, &mut journal);

        parser.parse();
        //parser.general_directive();

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
        assert_eq!(3, journal.master.flatten_account_tree().len());
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

        let post1 = &xact.posts[0];
        assert_eq!("Expenses", journal.get_account(post1.account).name);
        assert_eq!("20", post1.amount.as_ref().unwrap().quantity.to_string());
        assert_eq!(None, post1.amount.as_ref().unwrap().get_commodity());

        let post2 = &xact.posts[1];
        assert_eq!("Assets", journal.get_account(post2.account).name);
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

    use crate::{
        journal::Journal,
        parser::{self, read_into_journal},
    };

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
        let post1 = &xact.posts[0];
        assert_eq!("Expenses", journal.get_account(post1.account).name);
        assert_eq!("20", post1.amount.as_ref().unwrap().quantity.to_string());
        assert_eq!(None, post1.amount.as_ref().unwrap().get_commodity());

        // let post_2 = xact.posts.iter().nth(1).unwrap();
        let post2 = &xact.posts[1];
        assert_eq!("Assets", journal.get_account(post2.account).name);
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
            let posts = &xact.posts;
            assert_eq!(2, posts.len());

            let acc1 = journal.get_account(posts[0].account);
            assert_eq!("Expenses", acc1.name);
            let acc2 = journal.get_account(posts[1].account);
            assert_eq!("Assets", acc2.name);
        } else {
            assert!(false);
        }
    }

    #[test_log::test]
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
        let posts = &xact.posts;
        assert_eq!(2, posts.len());

        // post 1
        let p1 = &posts[0];
        let account = journal.get_account(p1.account);
        assert_eq!("Investment", account.name);
        let parent = journal.get_account(account.parent);
        assert_eq!("Assets", parent.name);
        // amount
        let Some(a1) = &p1.amount else { panic!() };
        assert_eq!("20", a1.quantity.to_string());
        let comm1 = a1.get_commodity().unwrap();
        assert_eq!("VEUR", comm1.symbol);
        let Some(ref cost1) = p1.cost else { panic!() };
        // cost
        assert_eq!(200, cost1.quantity.into());
        assert_eq!("EUR", cost1.get_commodity().unwrap().symbol);

        // post 2
        let p2 = &posts[1];
        assert_eq!("Assets", journal.get_account(p2.account).name);
        // amount
        let Some(a2) = &p2.amount else { panic!() };
        assert_eq!("-200", a2.quantity.to_string());
        let comm2 = a2.get_commodity().unwrap();
        assert_eq!("EUR", comm2.symbol);

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
        let xact = &journal.xacts[0];
        assert_eq!(1, journal.xacts.len());
        assert_eq!(4, xact.posts.len());
        assert_eq!(7, journal.master.flatten_account_tree().len());
        assert_eq!(2, journal.commodity_pool.commodities.len());
    }
}

#[cfg(test)]
mod posting_parsing_tests {
    use std::io::Cursor;

    use super::Parser;
    use crate::{amount::Quantity, journal::Journal, parse_file, parser::parse_datetime};

    #[test]
    fn test_parsing_buy_lot() {
        let file_path = "tests/lot.ledger";
        let mut j = Journal::new();

        // Act
        parse_file(file_path, &mut j);

        // Assert
        assert_eq!(1, j.xacts.len());
        assert_eq!(4, j.master.flatten_account_tree().len());
        assert_eq!(2, j.commodity_pool.commodities.len());
        // price
        assert_eq!(2, j.commodity_pool.commodity_history.node_count());
        assert_eq!(1, j.commodity_pool.commodity_history.edge_count());
        // Check price: 10 VEUR @ 12.75 EUR
        let price = j
            .commodity_pool
            .commodity_history
            .edge_weight(0.into())
            .unwrap();
        let expected_date = parse_datetime("2023-05-01").unwrap();
        // let existing_key = price.keys().nth(0).unwrap();
        assert!(price.contains_key(&expected_date));
        let value = price.get(&expected_date).unwrap();
        assert_eq!(Quantity::from(12.75), *value);
    }

    #[test]
    fn test_buy_lot_cost() {
        let file_path = "tests/lot.ledger";
        let mut j = Journal::new();

        // Act
        parse_file(file_path, &mut j);

        // Assert the price of "10 VEUR @ 12.75 EUR" must to be 127.50 EUR
        let xact = j.xacts.get(0).unwrap();
        let post = &xact.posts[0];
        let cost = post.cost.unwrap();
        assert_eq!(cost.quantity, 127.5.into());

        let eur = j.commodity_pool.find("EUR").unwrap();
        assert_eq!(cost.commodity, eur);
    }

    #[test]
    fn test_parsing_trailing_xact_comment() {
        let input = r#"2023-03-02 Payee
    ; this is xact comment
    Expenses  20 EUR
    Assets
"#;
        let journal = &mut Journal::new();
        let mut parser = Parser::new(Cursor::new(input), journal);

        parser.parse();

        // Assert
        assert!(journal.xacts[0].note.is_some());
        assert_eq!(
            Some("this is xact comment".to_string()),
            journal.xacts[0].note
        );
    }

    #[test]
    fn test_parsing_trailing_post_comment() {
        let input = r#"2023-03-02 Payee
    Expenses  20 EUR
    ; this is post comment
    Assets
"#;
        let journal = &mut Journal::new();
        let mut parser = Parser::new(Cursor::new(input), journal);

        parser.parse();

        // Assert
        let xact = &journal.xacts[0];
        assert!(xact.posts[0].note.is_some());
        assert!(xact.posts[1].note.is_none());
        assert_eq!(Some("this is post comment".to_string()), xact.posts[0].note);
    }
}

#[cfg(test)]
mod amount_parsing_tests {
    use super::Amount;
    use crate::{amount::Quantity, journal::Journal, parser::parse_post, xact::Xact};

    fn setup() -> Journal {
        let mut journal = Journal::new();
        let xact = Xact::create("2023-05-02", "", "Supermarket", "");
        journal.add_xact(xact);

        journal
    }

    #[test]
    fn test_positive_no_commodity() {
        let expected = Amount::new(20.into(), None);
        let actual = Amount::new(20.into(), None);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_negative_no_commodity() {
        let actual = Amount::new((-20).into(), None);
        let expected = Amount::new((-20).into(), None);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_pos_w_commodity_separated() {
        const SYMBOL: &str = "EUR";
        let mut journal = setup();
        let eur_ptr = journal.commodity_pool.create(SYMBOL, None);
        let xact_ptr = &journal.xacts[0] as *const Xact;
        let expected = Amount::new(20.into(), Some(eur_ptr));

        // Act

        let _ = parse_post("  Assets  20 EUR", xact_ptr, &mut journal);

        // Assert
        let xact = &journal.xacts[0];
        let post = xact.posts.first().unwrap();
        let amount = &post.amount.unwrap();

        assert_eq!(&expected.commodity, &amount.commodity);
        assert_eq!(expected, *amount);

        // commodity
        let c = amount.get_commodity().unwrap();
        assert_eq!("EUR", c.symbol);
    }

    #[test]
    fn test_neg_commodity_separated() {
        const SYMBOL: &str = "EUR";
        let mut journal = setup();
        let eur_ptr = journal.commodity_pool.create(SYMBOL, None);
        let expected = Amount::new((-20).into(), Some(eur_ptr));
        let xact_ptr = &journal.xacts[0] as *const Xact;

        // Act
        let _ = parse_post("  Assets  -20 EUR", xact_ptr, &mut journal);

        // Assert
        let xact = &journal.xacts[0];
        let post = xact.posts.first().unwrap();
        let Some(amt) = &post.amount else { panic!() };
        assert_eq!(&expected, amt);

        let commodity = amt.get_commodity().unwrap();
        assert_eq!("EUR", commodity.symbol);
    }

    #[test]
    fn test_full_w_commodity_separated() {
        // Arrange
        let mut journal = setup();
        let xact_ptr = &journal.xacts[0] as *const Xact;

        // Act
        let _ = parse_post("  Assets  -20000.00 EUR", xact_ptr, &mut journal);
        let xact = &journal.xacts[0];
        let post = xact.posts.first().unwrap();
        let Some(ref amount) = post.amount else {
            panic!()
        };

        // Assert
        assert_eq!("-20000.00", amount.quantity.to_string());
        assert_eq!("EUR", amount.get_commodity().unwrap().symbol);
    }

    #[test]
    fn test_full_commodity_first() {
        // Arrange
        let mut journal = setup();
        let xact_ptr = &journal.xacts[0] as *const Xact;

        // Act
        let _ = parse_post("  Assets  A$-20000.00", xact_ptr, &mut journal);
        let xact = &journal.xacts[0];
        let post = xact.posts.first().unwrap();
        let Some(ref amount) = post.amount else {
            panic!()
        };

        // Assert
        assert_eq!("-20000.00", amount.quantity.to_string());
        assert_eq!("A$", amount.get_commodity().unwrap().symbol);
    }

    #[test]
    fn test_quantity_separators() {
        let input = "-1000000.00";
        let expected = Quantity::from(-1_000_000);

        let amount = Amount::new(Quantity::from_str(input).unwrap(), None);

        let actual = amount.quantity;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_addition() {
        //let c1 = Commodity::new("EUR");
        let left = Amount::new(10.into(), None);
        // let c2 = Commodity::new("EUR");
        let right = Amount::new(15.into(), None);

        let actual = left + right;

        assert_eq!(Quantity::from(25), actual.quantity);
        // assert!(actual.commodity.is_some());
        // assert_eq!("EUR", actual.commodity.unwrap().symbol);
    }

    #[test]
    fn test_add_assign() {
        // let c1 = Commodity::new("EUR");
        let mut actual = Amount::new(21.into(), None);
        // let c2 = Commodity::new("EUR");
        let other = Amount::new(13.into(), None);

        // actual += addition;
        actual.add(&other);

        assert_eq!(Quantity::from(34), actual.quantity);
    }

    #[test]
    fn test_copy_from_no_commodity() {
        let other = Amount::new(10.into(), None);
        let actual = Amount::copy_from(&other);

        assert_eq!(Quantity::from(10), actual.quantity);
        // assert_eq!(None, actual.commodity);
    }
}
