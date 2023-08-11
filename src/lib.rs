/*!
# Ledger-rs library

Ledger-cli functionality implemented in Rust

Early work-in-progress.

The basic functionality demo:

Given a `basic.ledger` text file, with the contents

```ledger
2023-04-21 Supermarket
    Expenses:Food  20 EUR
    Assets:Cash
```

you can use the library to parse the transactions from the file and provide a basic
report on account balances

```
    let actual = ledger_rs_lib::run_command("b -f tests/basic.ledger");

    assert!(!actual.is_empty());
    assert_eq!(5, actual.len());
    assert_eq!("Account  has balance 0 EUR", actual[0]);
    assert_eq!("Account Assets has balance -20 EUR", actual[1]);
    assert_eq!("Account Assets:Cash has balance -20 EUR", actual[2]);
    assert_eq!("Account Expenses has balance 20 EUR", actual[3]);
    assert_eq!("Account Expenses:Food has balance 20 EUR", actual[4]);
```
*/
use std::{fs::File, io::Cursor};

use journal::Journal;
use option::InputOptions;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub mod account;
mod annotate;
pub mod amount;
mod balance;
pub mod commodity;
mod directives;
pub mod history;
mod iterator;
pub mod journal;
mod journalreader;
mod option;
pub mod parser;
pub mod pool;
pub mod post;
pub mod report;
pub mod scanner;
pub mod utilities;
mod value;
pub mod xact;

/// An entry point for the CLIs.
/// The commands and arguments sent to the CLI are processed here. This is
/// so that 3rd-party clients can pass argv and get the same result.
/// The arguments should be compatible with Ledger, so that the functionality is comparable.
///
pub fn run(args: Vec<String>) -> Vec<String> {
    // separates commands from the options
    let (commands, options) = option::process_arguments(args);

    execute_command(commands, options)
}

/// A convenient entry point if you want to use a command string directly.
/// command: &str A Ledger-style command, i.e. "balance -f journal.ledger"
///
pub fn run_command(command: &str) -> Vec<String> {
    let args = shell_words::split(command).unwrap();
    run(args)
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
            let mut output = report::report_accounts(&journal);
            output.sort();
            output
        }
        'b' => {
            match verb.as_str() {
                "b" | "bal" | "balance" => {
                    // balance report
                    report::balance_report(&journal)
                }
                "budget" => {
                    // budget
                    todo!("budget!")
                }
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

/// Parses text containing Ledger-style journal.
/// text: &str  A Ledger-style journal. The same content that is normally
///             stored in text files
/// journal: &mut Journal  The result are stored in the given Journal instance.
pub fn parse_text(text: &str, journal: &mut Journal) {
    let source = Cursor::new(text);
    parser::read_into_journal(source, journal);
}

pub fn parser_experiment() {
    // read line from the Journal
    // determine the type
    // DirectiveType
    // scan the line
    // parse into a model instance
    // read additional lines, as needed. Ie for Xact/Posts.
    // return the directive with the entity, if created

    todo!("try the new approach")
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn wasm_test() -> String {
    "hello from wasm".to_owned()
}

#[cfg(test)]
mod lib_tests {
    use std::assert_eq;

    use crate::{amount::{Amount, Quantity}, option, run};

    #[test]
    fn test_minimal() {
        // create a ledger command
        let command = "b -f tests/minimal.ledger";
        let args = shell_words::split(command).unwrap();
        let expected = r#"Account  has balance 0
Account Assets has balance -20
Account Expenses has balance 20"#;

        // the test fails when checking the parent in fullname() for
        // Assets (the first child account).

        let actual = run(args).join("\n");

        // Assert
        assert!(!actual.is_empty());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_multiple_files() {
        // arrange
        let args =
            shell_words::split("accounts -f tests/minimal.ledger -f tests/basic.ledger").unwrap();
        let (_commands, input_options) = option::process_arguments(args);
        // let cdty = Commodity::new("EUR");

        // Act
        let journal = super::session_read_journal_files(&input_options);

        // Assert
        let xact0 = &journal.xacts[0];
        let xact1 = &journal.xacts[1];

        // xacts
        assert_eq!(2, journal.xacts.len());
        assert_eq!("Payee", xact0.payee);
        assert_eq!("Supermarket", xact1.payee);

        // posts
        // assert_eq!(4, journal.posts.len());
        assert_eq!(2, xact0.posts.len());
        assert_eq!(2, xact1.posts.len());
        assert_eq!(Some(Amount::new(20.into(), None)), xact0.posts[0].amount);
        // amount
        assert_eq!(xact1.posts[0].amount.unwrap().quantity, Quantity::from_str("20").unwrap());
        assert_eq!(xact1.posts[0].amount.unwrap().get_commodity().unwrap().symbol, "EUR");

        // accounts
        let mut accounts = journal.master.flatten_account_tree();
        // hack for the test, since the order of items in a hashmap is not guaranteed.
        accounts.sort_unstable_by_key(|acc| &acc.name);

        assert_eq!("", accounts[0].name);
        assert_eq!("Assets", accounts[1].name);
        assert_eq!("Cash", accounts[2].name);
        assert_eq!("Expenses", accounts[3].name);
        assert_eq!("Food", accounts[4].name);

        // commodities
        assert_eq!(1, journal.commodity_pool.commodities.len());
        assert_eq!(
            "EUR",
            journal.commodity_pool.find("EUR").unwrap().symbol
        );
    }
}
