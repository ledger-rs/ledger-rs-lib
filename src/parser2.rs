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

use crate::{
    account::Account, amount::Amount, commodity::Commodity, journal::Journal, post::Post, scanner,
    xact::Xact,
};

pub(crate) fn read<T: Read>(source: T) -> Journal {
    let mut parser = Parser::new(source);
    parser.parse();

    parser.journal
}

struct Parser<T: Read> {
    pub journal: Journal,

    reader: BufReader<T>,
    buffer: String,
}

impl<T: Read> Parser<T> {
    pub fn new(source: T) -> Self {
        let reader = BufReader::new(source);
        // To avoid allocation, reuse the String variable.
        let buffer = String::new();

        Self {
            reader,
            buffer,
            journal: Journal::new(),
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
        if self.buffer == "\r\n" {
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

                    // parse
                    match self.buffer.chars().peekable().peek() {
                        Some(' ') => {
                            // valid line
                            let input = self.buffer.trim_start();
                            // Process the Xact content line. Could be a Comment or a Post.
                            match input.chars().peekable().peek() {
                                Some(';') => {
                                    todo!("trailing note")
                                }
                                _ => {
                                    let tokens = scanner::scan_post(input);

                                    let account_index;
                                    {
                                        // Create Account, add to collection
                                        let account = Account::parse(tokens[0]);
                                        account_index = self.journal.add_account(account);
                                    }

                                    let commodity_index;
                                    {
                                        // Create Commodity, add to collection
                                        // commodity_pool.find(symbol)
                                        // pool.create(symbol)
                                        let commodity = Commodity::parse(tokens[2]);
                                        commodity_index = match commodity {
                                            Some(c) => Some(self.journal.add_commodity(c)),
                                            None => None,
                                        };
                                    }

                                    // create amount
                                    let amount = Amount::parse2(tokens[1], commodity_index);

                                    // TODO: handle cost (2nd amount)
                                    let price_commodity_index;
                                    {
                                        let commodity = Commodity::parse(tokens[4]);
                                        price_commodity_index = match commodity {
                                            Some(c) => Some(self.journal.add_commodity(c)),
                                            None => None,
                                        }
                                    }
                                    let cost = Amount::parse2(tokens[3], price_commodity_index);

                                    let post_index;
                                    {
                                        // Create Post, link Xact, Account, Commodity
                                        let post =
                                            Post::new(account_index, xact_index, amount);
                                        post_index = self.journal.add_post(post);
                                    }

                                    // add Post to Account.posts
                                    {
                                        let account =
                                            self.journal.accounts.get_mut(account_index).unwrap();
                                        account.post_indices.push(post_index);
                                    }

                                    {
                                        // add Post to Xact.
                                        let xact = self.journal.xacts.get_mut(xact_index).unwrap();
                                        xact.posts.push(post_index);
                                    }
                                }
                            }
                        }
                        Some('\r') => {
                            // empty line "\r\n". Exit.
                            break;
                        }
                        _ => {
                            // log::warn!("we have {:?}", c);
                            panic!("should not happen")
                        }
                    }
                }
            }

            // "finalize" transaction
            crate::xact::finalize_indexed(xact_index, &mut self.journal);

            // empty the buffer before exiting.
            self.buffer.clear();
        }
    }
}

#[cfg(test)]
mod full_tests {
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

        let post1 = &journal.posts[xact.posts[0]];
        assert_eq!("Expenses", journal.get_account(post1.account_index).name);
        assert_eq!("20", post1.amount.as_ref().unwrap().quantity.to_string());
        assert_eq!(None, post1.amount.as_ref().unwrap().commodity_index);

        // let post_2 = xact.posts.iter().nth(1).unwrap();
        let post2 = &journal.posts[xact.posts[1]];
        assert_eq!("Assets", journal.get_account(post2.account_index).name);
    }
}

#[cfg(test)]
mod parser_tests {
    use std::io::Cursor;

    #[test]
    fn parse_standard_xact() {
        let input = r#"; Standard transaction
2023-04-10 Supermarket
    Expenses  20 EUR
    Assets
"#;
        let cursor = Cursor::new(input);

        let journal = super::read(cursor);

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
        } else {
            assert!(false);
        }
    }
}
