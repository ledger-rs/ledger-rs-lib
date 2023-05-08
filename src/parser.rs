/**
 * Parser
 *
 * Parses textual input into the model structure.
 */
use std::{io::{BufRead, BufReader, Read, self}, fs::File, path::Path};

use chrono::NaiveDate;

use crate::{
    account::Account,
    amount::Amount,
    context::ParsingContext,
    journal::Journal,
    post::Post,
    xact::{self, Xact},
};

enum LineParseResult {
    Comment,
    Empty,
    Xact(Xact),
    Post(Post),
}

/// parse textual input
pub fn parse<T: Read>(source: T) -> Journal {
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
            // Ok(0) => {
            //     // end of file?
            //     println!("End of file");
            //     break;
            // }
            Ok(count) => {
                log::debug!("Read {:?} characters", count);

                // Remove the trailing newline characters
                // let clean_line = strip_trailing_newline(&line);
                let trimmed = &line.trim_end();

                // Use the line from the file. Parse.
                let result = parse_line(&context, &trimmed);
                // Do something with the parsed element.
                process_parsed_element(&mut context, result);

                // clear the buffer before reading the next line.
                line.clear();
            }
        }
    }

    context.journal
}

/// The apparently efficient method from
/// https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Parsing each individual line. The controller of the parsing logic.
fn parse_line(context: &ParsingContext, line: &str) -> LineParseResult {
    if line.is_empty() {
        return LineParseResult::Empty;
    }

    let first_char = line.chars().nth(0).expect("first character");
    match first_char {
        // comments
        ';' | '#' | '*' | '|' => {
            // ignore
            return LineParseResult::Comment;
        }

        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
            // Starts with date.
            return parse_xact(line);
        }

        ' ' | '\t' => {
            if context.xact.is_some() {
                // We have an "open" transaction being parsed. Read the contents.
                return parse_xact_content(line);
            } else {
                panic!("Unexpected whitespace at beginning of line");
            }
        }

        _ => {
            // if !general_directive()
            // the rest
            todo!("the rest")
        }
    }
}

fn parse_date(date_str: &str) -> NaiveDate {
    // It should be enough to get the content until the first whitespace.
    // let ws_index = line.find(' ').expect("date end");
    // let date_str = &line[0..ws_index];

    // todo: support more date formats?

    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").expect("date parsed");

    date
}

/// Parse a Posting.
/// line is the source line trimmed on both ends.
fn parse_post(line: &str) -> Post {
    // todo: link to transaction
    // todo: position
    // pathname
    // position, line, sequence

    // todo: * and ! marks
    // state
    // virtual posts []
    // deferred posts <>

    let next = next_element(line, 0, true);

    let end = match next {
        Some(index) => index,
        None => line.len(),
    };

    let account_name = line[0..end].trim_end();
    // TODO: register account with the Journal, structure into a tree.
    //post.account = name;

    // Parse the optional amount

    let mut amount = None;
    let next_char = line.chars().skip(end).next();
    if next.is_some() && next_char.is_some() && next_char != Some(';') && next_char != Some('=') {
        if next_char != Some('(') {
            let amount_str = &line[next.unwrap()..];
            amount = Amount::parse(amount_str);
        } else {
            amount = parse_amount_expr();
        }
    }

    // Parse the optional balance assignment

    // Parse the optional note

    // There should be nothing more to read

    // tags

    let post = Post::new(account_name, amount);
    post
}

/// Finds the start of the next text element.
/// utils.h
/// inline char * next_element(char * buf, bool variable = false)
fn next_element(line: &str, start: usize, variable: bool) -> Option<usize> {
    let mut position: usize = 0;
    let mut spaces: u8 = 0;

    // iterate over the string
    for (i, character) in line.char_indices().skip(start) {
        if !(character == ' ' || character == '\t') {
            continue;
        }

        // current character is space or tab.
        spaces += 1;

        if !variable || character == '\t' || spaces == 2 {
            position = i + 1;
            return skip_ws(line, &position);
            // } else if character == '\t' {
            //     return skip_ws(line, &position + 1)
        }
    }

    None
}

fn parse_amount_expr() -> Option<Amount> {
    todo!("complete")
}

/// Parse transaction header
/// 2023-05-05 Payee  ; Note
fn parse_xact(line: &str) -> LineParseResult {
    // the first space is between the Date and the Payee.
    let separator_index = line
        .find(' ')
        .expect("separator between the date and the payee");
    let date_str = &line[..separator_index];
    let payee_str = &line[separator_index + 1..];

    let note_separator_index = line.find("  ;");
    if note_separator_index.is_some() {
        // let note_str = &line
    }

    // now translate the strings into values
    let date = parse_date(date_str);
    let payee = payee_str;
    // TODO: parse note
    let note = None;

    let xact = Xact::new(Some(date), payee, note);
    LineParseResult::Xact(xact)
}

fn parse_xact_old(line: &str) -> LineParseResult {
    let next = next_element(line, 0, false);
    let mut next_index = match next {
        Some(index) => index,
        None => 0,
    };

    // Parse the date
    let date = parse_date(line);

    if line.contains('=') {
        // TODO Parse the aux date
    }

    // TODO Parse the optional cleared flag: *
    // if next.is_some() {}

    // Parse the optional code: (TEXT)
    // if next.is_some() && next == '(' {}

    // Parse the description text
    let mut payee = "<Unspecified payee>";
    if next.is_some() && next_index < line.len() {
        let mut pos = next_index;
        let mut spaces: usize = 0;
        let mut tabs: usize = 0;
        // iterate further
        for character in line.chars().skip(next_index) {
            if character == ' ' {
                spaces += 1;
            } else if character == '\t' {
                tabs += 1;
            } else if character == ';' && (tabs > 0 || spaces > 1) {
                todo!("complete")
            } else {
                spaces = 0;
                tabs = 0;
            }
            pos += 1;
        }
        // TODO: validate payee
        // xact->payee = context.journal->validate_payee(next);
        payee = line[next_index..].into();
        // next = p;
        next_index = pos;
    }

    // Parse the xact note
    let note: Option<String> = None;
    if next.is_some()
        && next_index < line.len()
        && line.chars().skip(next_index).next() == Some(';')
    {
        todo!("append note")
    }

    // Parse all of the posts associated with this xact
    // ^ Parsed separately.

    // Tags

    let xact = Xact::new(Some(date), payee, note);
    LineParseResult::Xact(xact)
}

fn parse_xact_content(source_line: &str) -> LineParseResult {
    let line = source_line.trim();

    // trailing note
    if line.starts_with(';') {
        todo!("trailing note")
    }
    // todo: assert, check, expr

    let post = parse_post(line);
    LineParseResult::Post(post)
}

/// handler for the parsed element
fn process_parsed_element(context: &mut ParsingContext, parse_result: LineParseResult) {
    match parse_result {
        LineParseResult::Comment => (),

        LineParseResult::Empty => {
            match context.xact.take() {
                Some(xact) => {
                    // An empty line is a separator between transactions.

                    // Append Transaction to Journal.
                    let posts = context.posts.take().unwrap();
                    xact::finalize(xact, posts, &mut context.journal);

                    // Reset the current transaction variable.
                    context.xact = None;
                }
                // else just ignore.
                None => (),
            }
        }

        LineParseResult::Xact(xact) => {
            // Store in the context while being parsed.
            context.xact = Some(xact);

            // The transaction is finalized and added to Journal
            // after all the posts from the Xact are processed.
        }

        LineParseResult::Post(post) => {
            context.add_post(post);
        }
    }
}

/// Starts iterating through the string at the given location,
/// skips the whitespace and returns the location of the next element.
fn skip_ws(line: &str, start: &usize) -> Option<usize> {
    for (i, c) in line.char_indices().skip(*start) {
        while c == ' ' || c == '\t' || c == '\n' {
            continue;
        }
        return Some(i);
    }
    return None;
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::account::Account;

    use super::parse;

    #[test]
    fn test_minimal_parser() {
        let input = r#"; Minimal transaction
2023-04-10 Supermarket
    Expenses  20
    Assets

"#;
        let cursor = Cursor::new(input);

        let journal = parse(cursor);

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
