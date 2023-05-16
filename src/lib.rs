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
mod filters;
pub mod journal;
pub mod option;
mod parser;
mod pool;
mod post;
mod report;
mod scanner;
mod utils;
mod xact;

/// entry point.
/// The commands and arguments sent to the CLI are recognized and processed here. This is
/// so that 3rd-party clients can pass argv and get the same result.
/// The arguments should be compatible with Ledger, so that the functionality is comparable.
///
pub fn run(args: Vec<String>) -> Vec<String> {
    // separates commands from the options
    // read_command_arguments(args)
    let (commands, options) = option::process_arguments(args);

    execute_command(commands, options)
}

pub enum Kind {
    UNKNOWN,
    FUNCTION,
    OPTION,
    PRECOMMAND,
    COMMAND,
    DIRECTIVE,
    FORMAT,
}

fn get_filename_argument(args: &Vec<String>) -> Option<&str> {
    // Find the position of the -f arg
    let Some(index) = args.iter().position(|a| a == &"-f")
    else {
        return None;
    };

    // now take the filename
    let filename = match args.iter().nth(index + 1) {
        Some(file) => Some(file.as_str()),
        None => None,
    };

    filename
}

/// Entry point for a report?
fn report(journal: &Journal) -> Vec<String> {
    // TODO: identify which report

    // iterate over Journal
    // apply filters, etc.

    // get the output

    // TODO: replace this temporary report
    let mut output = report::report_accounts(journal).collect::<Vec<String>>();
    output.sort();
    output
}

/// global::execute_command equivalent
fn execute_command(commands: Vec<String>, options: Vec<String>) -> Vec<String> {
    let verb = commands.iter().nth(0).unwrap();

    // todo: look for pre-command
    // look_for_precommand(verb);

    // if !precommand
    //   if !at_repl
    let journal = session_read_journal_files(&options);

    // todo: lookup(COMMAND, verb)

    let command_args = &commands[1..];

    // for now just use a pre-defined report
    report(&journal)
}

fn look_for_precommand(verb: &str) {
    todo!()
}

fn session_read_journal_files(options: &Vec<String>) -> Journal {
    // Minimalistic approach:
    // get the file input
    let file_path = match get_filename_argument(&options) {
        Some(filename) => filename,
        None => panic!("No filename passed as argument"),
    };

    // parse the journal file(s)
    parse_file(file_path)
}

/// Parse input and return the model structure.
pub fn parse_file(file_path: &str) -> Journal {
    let file = File::open(file_path).expect("file opened");
    parser::read(file)
}

pub fn parse_text(text: &str) -> Journal {
    let source = Cursor::new(text);
    parser::read(source)
}

#[cfg(test)]
mod lib_tests {
    use crate::run;

    #[test]
    fn test_minimal() {
        // create a ledger command
        let command = "b -f tests/minimal.ledger";
        let args = shell_words::split(command).expect("arguments parsed");
        let expected = r#""#;

        let actual = run(args);

        todo!("get output back")
    }
}

#[cfg(test)]
mod arg_tests {
    use crate::get_filename_argument;

    #[test]
    fn test_get_file_arg() {
        let command = "b -f tests/minimal.ledger";
        let args = shell_words::split(command).expect("arguments parsed");
        let expected = Some("tests/minimal.ledger");

        let actual = get_filename_argument(&args);

        assert_eq!(expected, actual);
    }
}
