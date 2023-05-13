/**
 * Ledger-rs library
 * 
 * Implements all the logic and provides an entry point to 3rd-party code.
 */
use std::{fs::File, io::Cursor};

use journal::Journal;

mod account;
mod amount;
mod commodity;
mod context;
mod filters;
pub mod journal;
mod parser;
mod parser2;
mod pool;
mod post;
mod report;
mod scanner;
mod utils;
mod xact;

/// entry point?
pub fn run(args: Vec<String>) -> Vec<String> {
    // here we should accept parameters:
    // - command / report
    // - input data (files/string)
    // - filters

    // stick to Ledger-compatible arguments?

    // Minimalistic approach:
    // get the file input
    let file_path = match get_filename_argument(&args) {
        Some(filename) => filename,
        None => panic!("No filename passed as argument"),
    };
    // parse the file
    let journal = parse(file_path);

    // TODO: which report?
    // for now just use the balance report
    let output = report(journal);

    output
}

fn get_filename_argument(args: &Vec<String>) -> Option<&str> {
    if !args.contains(&"-f".to_string()) {
        return None;
    }

    // Find the position of the -f arg
    let index = args.iter().position(|a| a == &"-f").expect("the position of -f arg");
    // now take the filename
    let filename = match args.iter().nth(index + 1) {
        Some(file) => Some(file.as_str()),
        None => None,
    };
    
    filename
}

/// Entry point for a report?
fn report(journal: Journal) -> Vec<String> {
    // identify which report

    // iterate over Journal
    // apply filters, etc.

    // get the output

    vec![]
}

/// Parse input and return the model structure.
pub fn parse_file(file_path: &str) -> Journal {
    let file = File::open(file_path).expect("file opened");
    parser::parse(file)
}

pub fn parse(text: &str) -> Journal {
    let source = Cursor::new(text);
    parser::parse(source)
}


#[cfg(test)]
mod tests {
    use crate::{run, get_filename_argument};

    #[test]
    fn test_minimal() {
        // create a ledger command
        let command = "b -f tests/minimal.ledger";
        let args = shell_words::split(command).expect("arguments parsed");

        let actual = run(args);

        todo!("get output back")
    }

    #[test]
    fn test_get_file_arg() {
        let command = "b -f tests/minimal.ledger";
        let args = shell_words::split(command).expect("arguments parsed");
        let expected = Some("tests/minimal.ledger");

        let actual = get_filename_argument(&args);

        assert_eq!(expected, actual);
    }
}