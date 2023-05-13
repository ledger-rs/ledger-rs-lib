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
        if self.buffer.is_empty() {
            return Ok(());
        }

        // TODO: determine what the line is
        match self.buffer.chars().nth(0).unwrap() {
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

    fn xact_directive(&mut self) {
        let tokens = scanner::tokenize_xact_header(&self.buffer);
        let xact = Xact::create(tokens[0], tokens[1], tokens[2], tokens[3]);
        // Add xact to the journal
        let xact_index = self.journal.add_xact(xact);

        // TODO: read the Xact contents (Posts, Comments, etc.)
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

                                    // Create Account, add to collection
                                    let account = Account::parse(tokens[0]);
                                    let account_index = self.journal.add_account(account);

                                    // TODO: Create Commodity, add to collection
                                    // tokens[1], tokens[2]
                                    // commodity_pool.find(symbol)
                                    // pool.create(symbol)
                                    let commodity = Commodity::parse(tokens[2]);
                                    let commodity_index = match commodity {
                                        Some(c) => Some(self.journal.add_commodity(c)),
                                        None => None,
                                    };

                                    // create amount
                                    let amount = Amount::parse2(tokens[1], commodity_index);

                                    // TODO: handle cost (2nd amount)

                                    // TODO: Create Post, link Xact, Account, Commodity
                                    let post = Post::create_indexed(
                                        account_index,
                                        xact_index,
                                        amount,
                                    );
                                    let post_index = self.journal.add_post(post);

                                    let xact = self.journal.xacts.get_mut(xact_index).unwrap();
                                    xact.posts.push(post_index);
                                }
                            }
                        }
                        _ => {
                            panic!("should not happen")
                        }
                    }
                }
            }

            // log::debug!("read: {:?}, {:?}", x, &line);
            self.buffer.clear(); // empty the buffer before reading
        }
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
        assert_eq!(Account::new("Expenses"), post1.account_temp);
        assert_eq!("20", post1.amount.as_ref().unwrap().quantity.to_string());
        assert_eq!(None, post1.amount.as_ref().unwrap().commodity);

        // let post_2 = xact.posts.iter().nth(1).unwrap();
        let post2 = &journal.posts[xact.posts[1]];
        assert_eq!(Account::new("Assets"), post2.account_temp);
    }
}

#[cfg(test)]
mod parser_tests {}
