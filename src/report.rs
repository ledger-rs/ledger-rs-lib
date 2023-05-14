use crate::{journal::Journal, filters::calc_posts};

/**
 * Reports
 */

/// Accounts report. Command: `accounts`.
/// 
/// void report_t::posts_report(post_handler_ptr handler)
/// in output.cc
/// report_accounts
pub fn report_accounts(journal: &Journal) -> impl Iterator<Item = String> + '_ {
    journal.accounts.iter().map(|account| account.name.to_string())
}

fn report_commodities() {
    todo!()
}

fn report_payees() {
    todo!()
}

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
        let journal = parser::read(source);
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