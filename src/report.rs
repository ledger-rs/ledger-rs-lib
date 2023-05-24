use rust_decimal::Decimal;

use crate::{account::Account, journal::Journal};

/**
 * Reports
 */

/// Accounts report. Command: `accounts`.
///
/// void report_t::posts_report(post_handler_ptr handler)
/// in output.cc
/// report_accounts
pub fn report_accounts(journal: &Journal) -> impl Iterator<Item = String> + '_ {
    journal
        .accounts
        .iter()
        .map(|account| account.name.to_string())
}

fn report_commodities() {
    todo!()
}

fn report_payees() {
    todo!()
}

/// Balance report. Invoked with 'b' command.
/// Or accounts_report in ledger.
pub fn balance_report(mut journal: Journal) -> Vec<String> {
    // filters:
    // - amount
    // - total
    // - display amount
    // - display total
    // revalued total

    let mut output = vec![];

    // sort accounts
    journal.accounts.sort_by(|a, b| a.name.cmp(&b.name));
    // iterate over accounts
    for (i, acc) in journal.accounts.iter().enumerate() {
        // calculate balances

        // TODO: separate balance per currency
        let balance: Decimal = journal
            .posts
            .iter()
            .map(|post| post.amount.as_ref().unwrap().quantity)
            .sum::<Decimal>();

        let line = format!("Account {} has balance {}", &acc.name, balance);
        output.push(line);
    }

    // iterate over posts

    // calc_posts::operator() is in filters.cc

    // accounts_flusher

    output
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
        let mut journal = Journal::new();
        parser::read_into_journal(source, &mut journal);
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
