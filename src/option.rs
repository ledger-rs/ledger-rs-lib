use crate::Kind;

/**
 * option.cc
 *
 * Processes command arguments and options.
 */

/// Process arguments?
/// returns (commands, options)
pub fn process_arguments(args: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut options: Vec<String> = vec![];
    let mut commands: Vec<String> = vec![];

    // let mut remaining = vec![];

    // iterate through the list
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        if !arg.starts_with('-') {
            // otherwise return
            commands.push(arg.to_owned());
            continue;
        }

        // otherwise, it's an argument
        // if an item contains "-f", it is a real argument

        // long option
        if arg.starts_with("--") {
            // long argument
            if arg.len() == 2 {
                // it's a --, ending options processing
                todo!("handle this case?")
            } else if arg.len() == 1 {
                panic!("illegar option {}", arg)
            }

            // todo: check if there is '=' contained
            // else
            let option_name = &arg[2..];

            // TODO: find_option(option_name);

            // TODO: get argument value

            // TODO: process_option();

            todo!("complete")
        } else {
            // single-char option

            // iterate through all characters and identify options,
            for (i, c) in arg.char_indices() {
                if i == 0 {
                    // skipping the first ('-').
                    continue;
                }

                // TODO: check for a valid option and if it requires an argument?
                // find_option(c);
                // add option to the queue
                options.push(arg.to_owned());
            }

            // todo: for each option in option_queue (?)
            // are multiple arguments possible with "-"?

            // get the option argument
            if let Some(value) = iter.next() {
                // let mut whence = String::from("-");
                // whence.push(arg.chars().nth(0).unwrap());

                // TODO: check for validity, etc.
                // process_option(whence, Some(value.to_owned()));

                // for now, just add
                // options.push(whence);
                options.push(value.to_owned());
            } else {
                panic!("Missing option argument for {}", arg);
            }
        }
    }

    (commands, options)
}

fn find_option(letter: char) {
    let mut name = String::from(letter);
    name.push('_');

    lookup_session(crate::Kind::OPTION, &name);

    todo!()
}

/// find_option() from global.cc
fn lookup_option_global(option_name_char: char) {
    // adhiostv
    match option_name_char {
        's' => todo!("script"),
        't' => todo!("trace"),
        _ => todo!("other chars"),
    }

    todo!()
}

fn process_option(whence: String, value: Option<String>) {
    let mut args = vec![];

    // add the argument and the value to a collection
    args.push(whence);

    match value {
        Some(val) => args.push(val),
        None => (),
    }

    // TODO: check for validity
    // if wants_arg ...
    // there have to be 2 args.
}

/// Lookup options for session
fn lookup_session(kind: Kind, name: &str) {
    match kind {
        Kind::FUNCTION => todo!(),
        Kind::OPTION => todo!(),
        _ => todo!()
    }
    // TODO: 
    // lookup_option_session(option);
}

/// Searches for a short-version option. i.e. -f for file
fn lookup_option_session(option: char) {
    match option {
        'Q' => todo!(),
        'Z' => todo!(),
        'c' => todo!(),
        'd' => todo!(),
        'e' => todo!(),
        'f' => todo!("option file_"),
        'i' => todo!(),
        'l' => todo!(),
        'm' => todo!(),
        'n' => todo!(),
        'p' => todo!(),
        'r' => todo!(),
        's' => todo!(),
        't' => todo!(),
        'v' => todo!(),
        _ => todo!("return NULL"),
    }
}

/// Lookup options for reports
fn report_lookup() {
    // %ABCDEFGHIJLMOPRSTUVWXYabcdefghijlmnopqrstuvwy
    // t:
    // amount, tail, total, total_data, truncate, total_width, time_report

    todo!("go through the report options")
}

#[cfg(test)]
mod tests {
    use shell_words::split;

    use crate::option::process_arguments;

    #[test]
    fn test_process_arguments() {
        let args = split("accounts -f basic.ledger").unwrap();

        let (commands, options) = process_arguments(args);

        assert_eq!(1, commands.len());
        assert_eq!("accounts", commands[0]);

        // options
        assert_eq!(2, options.len());
        assert_eq!("-f", options[0]);
        assert_eq!("basic.ledger", options[1]);
    }
}
