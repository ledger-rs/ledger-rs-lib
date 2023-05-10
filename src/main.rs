use std::env;

use ledger_rs_lib::run;

/// Main entry point for the CLI.
fn main() {
    println!("Hello, Ledger-rs!");

    let args: Vec<String> = env::args().collect();
    println!("You requested {:?}", args);
    
    let actual = ledger_rs_lib::run(args);

    println!("{:#?}", actual);
}
