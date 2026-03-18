use crate::account::Account;
use crate::tx_processor::TxProcessor;
use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::fmt;
use std::fmt::Display;

const DEPOSIT: &str = "deposit";
const WITHDRAWAL: &str = "withdrawal";
const DISPUTE: &str = "dispute";
const RESOLVE: &str = "resolve";
const CHARGEBACK: &str = "chargeback";

#[derive(Debug, Deserialize)]

pub struct RawTxRecord {
    #[serde(rename = "type")]
    tx_type: String,
    pub client: u16,
    tx: u32,
    amount: Option<f64>,
}

impl Display for RawTxRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(amount) = self.amount {
            write!(
                f,
                "{}, {}, {}, {amount}",
                self.tx_type, self.client, self.tx
            )
        } else {
            write!(f, "{}, {}, {}", self.tx_type, self.client, self.tx,)
        }
    }
}

pub fn process_record(tx_record: &RawTxRecord, tx_processor: &mut TxProcessor) -> Result<Account> {
    let tx_type_str = tx_record.tx_type.as_str().to_lowercase();

    // Following ATM rules when withdrawal or deposit is not specified or equal to zero
    if let Some(amount) = tx_record.amount {
        if amount <= 0.0000 {
            return Err(anyhow!(
                "Invalid record: Amount must be greater than 0.0000"
            ));
        }
    }

    match tx_type_str.as_str() {
        DEPOSIT => tx_processor.deposit(
            tx_record.client,
            tx_record.tx,
            tx_record
                .amount
                .ok_or_else(|| anyhow!("Invalid record: Missing deposit amount"))?,
        ),
        WITHDRAWAL => tx_processor.withdrawal(
            tx_record.client,
            tx_record.tx,
            tx_record
                .amount
                .ok_or_else(|| anyhow!("Invalid record: Missing withdrawal amount"))?,
        ),
        DISPUTE => tx_processor.dispute(tx_record.client, tx_record.tx),
        RESOLVE => tx_processor.resolve(tx_record.client, tx_record.tx),
        CHARGEBACK => tx_processor.chargeback(tx_record.client, tx_record.tx),
        _ => Err(anyhow!("Unsupported operation {tx_type_str}")),
    }
}
