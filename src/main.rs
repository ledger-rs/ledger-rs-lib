use std::{env, io::stdout};

/// Main entry point for the CLI.
fn main() {
    println!("Hello, Ledger-rs!");

    let args: Vec<String> = env::args().collect();
    println!("You requested {:?}", args);
    
    let output = ledger_rs_lib::run(args);
    // output_to_stdin(output);
    println!("{:?}", output);
}

// fn output_to_stdin(content: Vec<String>) {
//     content.try_for_each(|s| writeln!(stdout, "{}", s))
// }
