/**
 * Ledger-rs library
 * 
 * Implements all the logic and provides an entry point to 3rd-party code.
 */

mod context;
mod journal;
mod parser;
mod xact;

/// entry point?
pub fn run(args: Vec<String>) -> Vec<String> {
    // here we should accept parameters:
    // - command / report
    // - input data (files/string)
    // - filters

    // stick to Ledger-compatible arguments?

    todo!("entry point")
}

/// Entry point for a report?
fn report() {
    todo!()
}

/// Parse input and return the model structure.
fn parse() {
    todo!()
}


#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn test_minimal() {
        // create a ledger command
        let command = "b -f tests/minimal.ledger";
        let args = shell_words::split(command).expect("arguments parsed");

        let actual = run(args);

        todo!("get output back")
    }
}