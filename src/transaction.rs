use crate::account::Account;
use anyhow::Result;
use std::marker::PhantomData;

pub struct Withdrawal;
pub struct Deposit;
pub struct Dispute;
pub struct Resolve;
pub struct Chargeback;

// NOTE: that we only ever explicitly validate states: Deposit and Dispute,
// But it is nice to keep the enum values explicit for clarity and any future changes
// hence allow unused here
#[allow(unused)]
// For keeping different transaction types in a collection
pub enum TransactionType {
    Withdrawal(TransactionState<Withdrawal>),
    Deposit(TransactionState<Deposit>),
    Dispute(TransactionState<Dispute>),
    Resolve(TransactionState<Resolve>),
    Chargeback(TransactionState<Chargeback>),
}

pub struct TransactionState<State> {
    _state: PhantomData<State>,
}

impl TransactionState<Deposit> {
    pub fn new_deposit() -> Self {
        Self {
            _state: PhantomData,
        }
    }

    pub fn dispute(&self, amount: f64, account: &mut Account) -> Result<TransactionState<Dispute>> {
        account.hold(amount)?;
        Ok(TransactionState {
            _state: PhantomData,
        })
    }
}

impl TransactionState<Dispute> {
    pub fn resolve(&self, amount: f64, account: &mut Account) -> Result<TransactionState<Resolve>> {
        account.resolve(amount)?;
        Ok(TransactionState {
            _state: PhantomData,
        })
    }

    pub fn chargeback(
        &self,
        amount: f64,
        account: &mut Account,
    ) -> Result<TransactionState<Chargeback>> {
        account.chargeback(amount)?;
        Ok(TransactionState {
            _state: PhantomData,
        })
    }
}

impl TransactionState<Withdrawal> {
    pub fn new_withdrawal() -> Self {
        Self {
            _state: PhantomData,
        }
    }
}

pub struct Transaction {
    pub client_id: u16,
    pub amount: f64,
    pub state: TransactionType,
}

// Only withdrawal and deposit transactions can be newly created
impl Transaction {
    pub fn new_withdrawal(
        client_id: u16,
        amount: f64,
        account: &mut Account,
    ) -> Result<Transaction> {
        account.withdraw(amount)?;
        Ok(Self {
            client_id,
            amount,
            state: TransactionType::Withdrawal(TransactionState::new_withdrawal()),
        })
    }
    pub fn new_deposit(client_id: u16, amount: f64, account: &mut Account) -> Result<Transaction> {
        account.deposit(amount)?;
        Ok(Self {
            client_id,
            amount,
            state: TransactionType::Deposit(TransactionState::new_deposit()),
        })
    }
}
