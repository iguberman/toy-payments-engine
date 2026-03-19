use crate::account::Account;
use crate::transaction::{ChargebackTransaction, DisputeTransaction, ResolveTransaction};
use crate::transaction::{Transaction, TransactionState};
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::io::Write;

pub struct TxProcessor {
    transactions: HashMap<u32, TransactionState>,
    clients: HashMap<u16, Account>,
}

impl TxProcessor {
    pub fn new() -> Self {
        let clients = HashMap::new();
        let transactions = HashMap::new();
        Self {
            transactions,
            clients,
        }
    }

    /// Deposit: create new transaction (and maybe new account if this is the first deposit for this client)
    pub fn deposit(&mut self, client_id: u16, tx_id: u32, amount: f64) -> Result<Account> {
        if self.transactions.contains_key(&tx_id) {
            return Err(anyhow!("Transaction already exists!"));
        }
        #[allow(clippy::unwrap_or_default)]
        let account = self.clients.entry(client_id).or_insert(Account::default());
        let transaction_state =
            TransactionState::Deposit(Transaction::new_deposit(client_id, amount, account)?);
        self.transactions.insert(tx_id, transaction_state);
        Ok(account.clone())
    }

    /// Withdrawal: create new transaction, but fail if account doesn't exist or has insufficient available funds
    pub fn withdrawal(&mut self, client_id: u16, tx_id: u32, amount: f64) -> Result<Account> {
        if self.transactions.contains_key(&tx_id) {
            return Err(anyhow!("Transaction already exists!"));
        }
        // Withdrawal will fail if account doesn't yet exist, due to insufficient funds
        // since Zero amounts are weeded out at ingestion step,
        // It is safer to not create new account and avoid wasting resources in this case
        let account = self
            .clients
            .get_mut(&client_id)
            .ok_or_else(|| anyhow!("Can not withdraw from non-existing account!"))?;
        let transaction_state = TransactionState::Withdrawal(Transaction::new_withdrawal(
            client_id, amount, account,
        )?);
        self.transactions.insert(tx_id, transaction_state);
        Ok(account.clone())
    }

    /// Dispute existing "deposit" transaction if corresponding account is not locked
    pub fn dispute(&mut self, client_id: u16, tx_id: u32) -> Result<Account> {
        let transaction_state = self
            .transactions
            .get_mut(&tx_id)
            .ok_or_else(|| anyhow!("Transaction not found"))?;
        match &transaction_state {
            TransactionState::Deposit(deposit_transaction) => {
                if deposit_transaction.client_id != client_id {
                    Err(anyhow!("Client_id mismatch!"))
                } else {
                    let account = self
                        .clients
                        .get_mut(&client_id)
                        .ok_or(anyhow!("Client account not found: {}", client_id))?;
                    match deposit_transaction.dispute(account) {
                        Ok(disputed_transaction) => {
                            *transaction_state = TransactionState::Dispute(disputed_transaction);
                            Ok(account.clone())
                        }
                        Err(e) => Err(e),
                    }
                }
            }
            _ => Err(anyhow!(
                "Transaction is not a deposit and can not be disputed"
            )),
        }
    }

    /// Resolve existing "dispute" transaction if corresponding account is not locked
    pub fn resolve(&mut self, client_id: u16, tx_id: u32) -> Result<Account> {
        let transaction_state = self
            .transactions
            .get_mut(&tx_id)
            .ok_or_else(|| anyhow!("Transaction not found"))?;
        match &transaction_state {
            TransactionState::Dispute(disputed_transaction) => {
                if disputed_transaction.client_id != client_id {
                    Err(anyhow!("Client_id mismatch!"))
                } else {
                    let account = self
                        .clients
                        .get_mut(&client_id)
                        .ok_or(anyhow!("Client account not found: {}", client_id))?;
                    match disputed_transaction.resolve(account) {
                        Ok(resolved_transaction) => {
                            *transaction_state = TransactionState::Resolve(resolved_transaction);
                            Ok(account.clone())
                        }
                        Err(e) => Err(e),
                    }
                }
            }
            _ => Err(anyhow!(
                "Transaction is not disputed and can not be resolved"
            )),
        }
    }

    /// Chargeback existing "dispute" transaction if corresponding account is not locked
    /// This will lock the account for good
    pub fn chargeback(&mut self, client_id: u16, tx_id: u32) -> Result<Account> {
        let transaction_state = self
            .transactions
            .get_mut(&tx_id)
            .ok_or_else(|| anyhow!("Transaction not found"))?;
        match &transaction_state {
            TransactionState::Dispute(disputed_transaction) => {
                if disputed_transaction.client_id != client_id {
                    Err(anyhow!("Client_id mismatch!"))
                } else {
                    let account = self
                        .clients
                        .get_mut(&client_id)
                        .ok_or(anyhow!("Client account not found: {}", client_id))?;
                    match disputed_transaction.chargeback(account) {
                        Ok(chargeback_transaction) => {
                            *transaction_state =
                                TransactionState::Chargeback(chargeback_transaction);
                            Ok(account.clone())
                        }
                        Err(e) => Err(e),
                    }
                }
            }
            _ => Err(anyhow!(
                "Transaction is not disputed and can not be charged back"
            )),
        }
    }

    pub fn output_accounts<W: Write>(&self, mut writer: W) {
        writeln!(writer, "client,available,held,total,locked").unwrap();
        for (client_id, account) in self.clients.iter() {
            writeln!(writer, "{},{}", client_id, account).unwrap();
        }
    }
}
