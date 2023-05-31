/**
 * Ledger-rs library
 *
 * Implements all the logic and provides an entry point to 3rd-party code.
 */
use std::{fs::File, io::Cursor};

use journal::Journal;
use option::InputOptions;

mod account;
pub mod amount;
mod balance;
mod commodity;
mod history;
pub mod journal;
pub mod option;
mod parser;
mod pool;
mod post;
mod report;
mod scanner;
mod xact;

/// Entry point.
/// The commands and arguments sent to the CLI are recognized and processed here. This is
/// so that 3rd-party clients can pass argv and get the same result.
/// The arguments should be compatible with Ledger, so that the functionality is comparable.
///
pub fn run(args: Vec<String>) -> Vec<String> {
    // separates commands from the options
    let (commands, options) = option::process_arguments(args);

    execute_command(commands, options)
}

/// global::execute_command equivalent
fn execute_command(commands: Vec<String>, input_options: InputOptions) -> Vec<String> {
    let verb = commands.iter().nth(0).unwrap();

    // todo: look for pre-command
    // look_for_precommand(verb);

    // if !precommand
    //   if !at_repl
    let journal = session_read_journal_files(&input_options);

    // todo: lookup(COMMAND, verb)

    let command_args = &commands[1..];

    // execute command
    match verb.chars().next().unwrap() {
        'a' => {
            // accounts?
            // TODO: replace this temporary report
            let mut output = report::report_accounts(&journal).collect::<Vec<String>>();
            output.sort();
            output
        }
        'b' => {
            match verb.as_str() {
                "b" | "bal" | "balance" => {
                    // balance report
                    report::balance_report(journal)
                },
                "budget" => {
                    // budget
                    todo!("budget!")
                },
                _ => {
                    todo!("?")
                }
            }
        }
        _ => todo!("handle"),
    }
}

fn look_for_precommand(verb: &str) {
    todo!()
}

fn session_read_journal_files(options: &InputOptions) -> Journal {
    // Minimalistic approach:
    // get the file input

    // multiple filenames
    let mut journal = Journal::new();
    for filename in &options.filenames {
        // parse the journal file(s)
        parse_file(filename, &mut journal);
    }

    journal
}

/// Parse input and return the model structure.
pub fn parse_file(file_path: &str, journal: &mut Journal) {
    let file = File::open(file_path).expect("file opened");
    parser::read_into_journal(file, journal);
}

pub fn parse_text(text: &str, journal: &mut Journal) {
    let source = Cursor::new(text);
    parser::read_into_journal(source, journal);
}

#[cfg(test)]
mod lib_tests {
    use std::assert_eq;

    use crate::{amount::Amount, option, run, pool::CommodityIndex};

    #[test]
    fn test_minimal() {
        // create a ledger command
        let command = "b -f tests/minimal.ledger";
        let args = shell_words::split(command).unwrap();
        let expected = r#"Account  has balance 
Account Assets has balance -20
Account Expenses has balance 20"#;

        let actual = run(args).join("\n");

        // Assert
        assert!(!actual.is_empty());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_multiple_files() {
        let args =
            shell_words::split("accounts -f tests/minimal.ledger -f tests/basic.ledger").unwrap();
        let (_commands, input_options) = option::process_arguments(args);

        let journal = super::session_read_journal_files(&input_options);

        // Assert

        // xacts
        assert_eq!(2, journal.xacts.len());
        assert_eq!("Payee", journal.xacts[0].payee);
        assert_eq!("Supermarket", journal.xacts[1].payee);

        // posts
        assert_eq!(4, journal.posts.len());
        assert_eq!(Some(Amount::new(20.into(), None)), journal.posts[0].amount);
        assert_eq!(
            Some(Amount::new(20.into(), Some(CommodityIndex::new(0)))),
            journal.posts[2].amount
        );

        // accounts
        assert_eq!("", journal.accounts[0].name);
        assert_eq!("Expenses", journal.accounts[1].name);
        assert_eq!("Assets", journal.accounts[2].name);
        assert_eq!("Food", journal.accounts[3].name);
        assert_eq!("Cash", journal.accounts[4].name);

        // commodities
        assert_eq!(1, journal.commodity_pool.commodities.len());
        assert_eq!("EUR", journal.commodity_pool.find_commodity("EUR").unwrap().symbol);
    }
}
