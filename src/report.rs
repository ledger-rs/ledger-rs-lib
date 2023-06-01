/*!
 * Reports module containing the report definitions
 */

use crate::{balance::Balance, journal::Journal, account::Account};

/// Accounts report. Command: `accounts`.
///
/// void report_t::posts_report(post_handler_ptr handler)
/// in output.cc
/// report_accounts
pub fn report_accounts(journal: &Journal) -> Vec<String> {
    journal
        .accounts
        .iter()
        .map(|account| account.name.to_string())
        .collect()
}

fn report_commodities() {
    todo!()
}

fn report_payees() {
    todo!()
}

/// Balance report. Invoked with 'b' command.
/// Or accounts_report in ledger.
/// Vec<String>
pub fn balance_report(journal: &Journal) -> Vec<String> {
    // let balances = get_account_balances(&journal);
    // Now that the account totals are implemented, simply walk the master account.
    // Format output
    // format_balance_report(balances, &journal)

    let master = journal.get_master_account();
    get_children_lines(master, journal)
}

/// Quick test of the account traversal for assembling the totals.
fn get_children_lines<'a>(account: &'a Account, journal: &'a Journal) -> Vec<String> {
    let mut result = vec![];

    // account.total(journal).amounts.iter().map(|amt| amt.quantity)

    let mut balance_line = String::new();
    let total = account.total(journal);
    for amount in total.amounts.iter() {
        balance_line += amount.quantity.to_string().as_str();
        if amount.commodity_index.is_some() {
            if let Some(i) = amount.commodity_index {
                let c = journal.get_commodity(i);
                
                balance_line += " ";
                balance_line += c.symbol.as_str();
            }
        }
    }
    result.push(format!("Account {} has balance {}", account.name, balance_line));

    // children amounts
    for (_, index) in &account.accounts {
        let acct = journal.get_account(*index);
        result.extend(get_children_lines(acct, journal));
    }

    // account.accounts.values().map(|index| format!("Account") journal.get_account(*index).name)
    // .into_iter()

    result
}

/// To be deprecated, unless significantly faster than the account traversing.
/// Calculates account balances.
/// returns (account_name, balance)
///
fn get_account_balances(journal: &Journal) -> Vec<(String, Balance)> {
    let mut balances = vec![];

    // calculate balances
    for (i, acc) in journal.accounts.iter().enumerate() {
        // get posts for this account.
        let filtered_posts = journal.posts.iter().filter(|post| post.account_index == i);

        // TODO: separate balance per currency

        let mut balance: Balance = Balance::new();
        for post in filtered_posts {
            balance.add(&post.amount.as_ref().unwrap());
        }

        balances.push((acc.fullname(journal), balance));
    }
    balances
}

/// To be deprecated.
fn format_balance_report(mut balances: Vec<(String, Balance)>, journal: &Journal) -> Vec<String> {
    // sort accounts
    balances.sort_by(|(acc1, _bal1), (acc2, _bal2)| acc1.cmp(&acc2));

    let mut output = vec![];
    for (account, balance) in balances {
        let mut bal_text: String = String::new();
        for amount in &balance.amounts {
            //
            let symbol = match amount.commodity_index {
                Some(i) => journal.commodity_pool.commodity_history.get_commodity(i).symbol.as_str(),
                None => "",
            };

            if !bal_text.is_empty() {
                bal_text += ", ";
            }
            
            bal_text += amount.quantity.to_string().as_str();

            if !symbol.is_empty() {
                bal_text += " ";
                bal_text += symbol;
            }
        }
        let line = format!("Account {} has balance {}", account, bal_text);
        output.push(line);
    }
    output
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::balance_report;
    use crate::{journal::Journal, parser};

    #[test]
    fn test_balance_report_one_xact() {
        let src = r#";
2023-05-05 Payee
    Expenses  25 EUR
    Assets

"#;
        let mut journal = Journal::new();
        parser::read_into_journal(Cursor::new(src), &mut journal);

        let actual: Vec<String> = balance_report(&journal);

        assert!(!actual.is_empty());
        assert_eq!(3, actual.len());
        assert_eq!("Account Assets has balance -25", actual[0]);
        assert_eq!("Account Expenses has balance 25", actual[1]);
        assert_eq!("Account  has balance 0", actual[2]);
        // assert_eq!("Account  has balance ", actual[0]);
        // assert_eq!("Account Assets has balance -25 EUR", actual[1]);
        // assert_eq!("Account Expenses has balance 25 EUR", actual[2]);
    }

    #[test]
    fn test_bal_report_two_commodities() {
        let src = r#";
2023-05-05 Payee
    Expenses  25 EUR
    Assets

2023-05-05 Payee 2
    Expenses  13 BAM
    Assets
"#;
        let source = Cursor::new(src);
        let mut journal = Journal::new();
        parser::read_into_journal(source, &mut journal);

        // Act
        let actual: Vec<String> = balance_report(&journal);

        // Assert
        assert!(!actual.is_empty());
        assert_eq!(3, actual.len());
        assert_eq!("Account  has balance 0 EUR0 BAM", actual[0]);
        assert_eq!("Account Assets has balance -25 EUR-13 BAM", actual[1]);
        assert_eq!("Account Expenses has balance 25 EUR13 BAM", actual[2]);
    }

    #[test]
    fn test_bal_multiple_commodities_in_the_same_xact() {
        let src = r#";
2023-05-05 Payee
    Assets:Cash EUR  -25 EUR
    Assets:Cash USD   30 USD
"#;
        let source = Cursor::new(src);
        let mut journal = Journal::new();
        parser::read_into_journal(source, &mut journal);

        // Act
        let actual: Vec<String> = balance_report(&journal);

        // Assert
        assert!(!actual.is_empty());
        assert_eq!(4, actual.len());
        assert_eq!("Account  has balance -25EUR30USD", actual[0]);
        assert_eq!("Account Assets has balance 30USD-25EUR", actual[1]);
        assert_eq!("Account Assets:Cash USD has balance 30 USD", actual[2]);
        assert_eq!("Account Assets:Cash EUR has balance -25 EUR", actual[3]);
    }
}
