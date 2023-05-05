/**
 * Parser
 *
 * Parses textual input into the model structure.
 */
use std::io::{Read, BufReader, BufRead};

use chrono::NaiveDate;

use crate::{journal::Journal, context::ParsingContext, xact::Xact, post::Post, amount::Amount};

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
            Ok(0) => {
                // end of file?
                println!("End of file");
                break;
            }
            Ok(_) => {
                // Remove the trailing newline characters
                // let clean_line = strip_trailing_newline(&line);
                let clean_line = &line.trim_end();

                // use the read value
                let result = parse_line(&mut context, &clean_line);
                process_parsed_element(&mut context, result);

                // clear the buffer before reading the next line.
                line.clear();
            }
        }
    }

    context.journal
}

/// Parsing each individual line. The controller of the parsing logic.
fn parse_line<'a>(context: &mut ParsingContext, line: &str) -> LineParseResult {
    let len = line.len();
    if len == 0 {
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
            return parse_xact(line);
        }

        ' ' | '\t' => {
            if context.xact.is_some() {
                return parse_xact_content(context, line);
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

fn parse_date(line: &str) -> NaiveDate {
    // It should be enough to get the content until the first whitespace.
    let ws_index = line.find(' ').expect("date end");
    let date_str = &line[0..ws_index];

    // todo: support more date formats?
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").expect("date parsed");

    date
}

/// Parse a Posting.
/// line is the source line trimmed on both ends.
fn parse_post(context: &mut ParsingContext, line: &str) -> Post {
    let mut post = Post::new();

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

    let name = line[0..end].trim_end().to_string();
    // TODO: register account with the Journal, structure into a tree.
    post.account = name;

    // Parse the optional amount

    let next_char = line.chars().skip(end).next();
    if next.is_some() && next_char.is_some() && next_char != Some(';') && next_char != Some('=') {
        if next_char != Some('(') {
            let amount_slice = &line[next.unwrap()..];
            post.amount = Amount::parse(context, amount_slice);
        } else {
            post.amount = parse_amount_expr();
        }
    }

    // Parse the optional balance assignment

    // Parse the optional note

    // There should be nothing more to read

    // tags

    post
}

/// Finds the start of the next text element.
/// utils.h
/// inline char * next_element(char * buf, bool variable = false)
fn next_element(line: &str, start: usize, variable: bool) -> Option<usize> {
    let mut position: usize = 0;
    let mut spaces: u8 = 0;

    // iterate over the string
    for p in line.char_indices().skip(start) {
        let character = p.1;
        if !(character == ' ' || character == '\t') {
            continue;
        }

        // current character is space or tab.
        spaces += 1;

        if !variable || character == '\t' || spaces == 2 {
            position = p.0 + 1;
            return skip_ws(line, &position);
            // } else if character == '\t' {
            //     return skip_ws(line, &position + 1)
        }
    }

    None
}

fn parse_amount_expr() -> Amount {
    todo!("complete")
}

fn parse_xact(line: &str) -> LineParseResult {
    // let mut next_start: usize = 0;
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
    let mut payee = "<Unspecified payee>".to_owned();
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

fn parse_xact_content(context: &mut ParsingContext, source_line: &str) -> LineParseResult {
    let line = source_line.trim();

    // trailing note
    if line.starts_with(';') {
        todo!("trailing note")
    }
    // todo: assert, check, expr

    let post = parse_post(context, line);
    LineParseResult::Post(post)
}

/// handler for the parsed element
fn process_parsed_element(context: &mut ParsingContext, parse_result: LineParseResult) {
    match parse_result {
        LineParseResult::Comment => (),

        LineParseResult::Empty => {
            match context.xact.take() {
                Some(xact_val) => {
                    // An empty line is a separator between transactions.
                    // Append to Journal.
                    context.journal.add_xact(xact_val);

                    // Reset the current transaction variable. <= done by .take()
                    // context.xact = None;

                }
                // else just ignore.
                None => (),
            }
        }

        LineParseResult::Xact(xact) => {
            context.xact = Some(xact);
            // The transaction is finalized and added to Journal
            // after the posts are processed.
        }

        LineParseResult::Post(post) => {
            // todo: link xact to post.xact
            // add to xact.posts
            context.xact.as_mut().expect("xact ref").add_post(post);
        }
    }
}

/// Starts iterating through the string at the given location,
/// skips the whitespace and returns the location of the next element.
fn skip_ws(line: &str, start: &usize) -> Option<usize> {
    for p in line.char_indices().skip(*start) {
        let character = p.1;
        while character == ' ' || character == '\t' || character == '\n' {
            continue;
        }
        return Some(p.0);
    }

    return None;
}


#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::parse;

    #[test]
    fn test_minimal_parser() {
        let input = r#"; Minimal transaction
2023-04-10 Supermarket
    Expenses  20
    Assets

"#;
        let cursor = Cursor::new(input);

        let actual = parse(cursor);

        assert!(false, "add checks");
    }
}
