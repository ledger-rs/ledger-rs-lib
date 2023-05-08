use crate::{journal::Journal, filters::calc_posts};

/**
 * Reports
 */

/// Balance report. Invoked with 'b' command.
/// Or accounts_report in ledger.
fn balance_report(journal: Journal) -> Vec<String> {
    // filters:
    // - amount
    // - total
    // - display amount
    // - display total
    // revalued total

    // iterate over posts

    // calc_posts::operator() is in filters.cc
    calc_posts();

    // accounts_flusher
    
    todo!()
}

fn accounts_flusher_operator() {
    // create accounts iterator
    // pass_down_accounts
    todo!()
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{journal::Journal, parser};

    use super::balance_report;

    fn create_journal() -> Journal {
        let src = r#";
2023-05-05 Payee
    Expenses  20 EUR
    Assets

"#;
        let source = Cursor::new(src);
        let journal = parser::parse(source);
        journal
    }

    //#[test]
    fn test_balance_report_one_xact() {
        let journal = create_journal();

        let actual = balance_report(journal);

        todo!("implement the report");
        assert!(false)
    }
}