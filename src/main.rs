use std::env;

use ledger_rs_lib;

/// Main entry point for the CLI.
fn main() {
    println!("Hello, Ledger-rs!");

    let args: Vec<String> = env::args().collect();
    println!("You requested {:?}", args);

    // let args = read_command_arguments(args);

    if !args.is_empty() {
        execute_command(args);
    } else {
        // repl.
        panic!("not implemented")
    }
}

// fn output_to_stdin(content: Vec<String>) {
//     content.try_for_each(|s| writeln!(stdout, "{}", s))
// }

fn execute_command(args: Vec<String>) {
    let output = ledger_rs_lib::run(args);
    // output_to_stdin(output);
    println!("{:?}", output);
}
