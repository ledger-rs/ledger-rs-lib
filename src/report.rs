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
pub fn balance_report(journal: Journal) -> Vec<String> {
    let balances = get_account_balances(journal);

    // Format output
    format_balance_report(balances)
}

fn get_account_balances(journal: Journal) -> Vec<(String, Decimal)> {
    let mut balances = vec![];

    // calculate balances
    for (i, acc) in journal.accounts.iter().enumerate() {
        // TODO: separate balance per currency

        let balance: Decimal = journal
            .posts
            .iter()
            .filter(|post| post.account_index == i)
            .map(|post| post.amount.as_ref().unwrap().quantity)
            .sum();

        balances.push((acc.name.to_owned(), balance));
    }
    balances
}

fn format_balance_report(mut balances: Vec<(String, Decimal)>) -> Vec<String> {
    // sort accounts
    balances.sort_by(|(acc1, bal1), (acc2, bal2)| acc1.cmp(&acc2));

    let mut output = vec![];
    for (account, balance) in balances {
        let line = format!("Account {} has balance {}", account, balance.to_string());
        output.push(line);
    }
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
    Expenses  25 EUR
    Assets

"#;
        let source = Cursor::new(src);
        let mut journal = Journal::new();
        parser::read_into_journal(source, &mut journal);
        journal
    }

    #[test]
    fn test_balance_report_one_xact() {
        let journal = create_journal();

        let actual = balance_report(journal);

        assert!(!actual.is_empty());
        assert_eq!("Account Assets has balance -25 EUR", actual[0]);
        assert_eq!("Account Expenses has balance 25 EUR", actual[1]);
    }
}
