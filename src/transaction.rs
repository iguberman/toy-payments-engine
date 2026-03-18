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
pub enum TransactionState {
    Withdrawal(Transaction<Withdrawal>),
    Deposit(Transaction<Deposit>),
    Dispute(Transaction<Dispute>),
    Resolve(Transaction<Resolve>),
    Chargeback(Transaction<Chargeback>),
}

pub trait DisputeTransaction {
    fn dispute(&self, account: &mut Account) -> Result<Transaction<Dispute>>;
}

pub trait ResolveTransaction {
    fn resolve(&self, account: &mut Account) -> Result<Transaction<Resolve>>;
}

pub trait ChargebackTransaction {
    fn chargeback(&self, account: &mut Account) -> Result<Transaction<Chargeback>>;
}

impl DisputeTransaction for Transaction<Deposit> {
    fn dispute(&self, account: &mut Account) -> Result<Transaction<Dispute>> {
        account.hold(self.amount)?;
        Ok(Transaction {
            client_id: self.client_id,
            tx_id: self.tx_id,
            amount: self.amount,
            _state: PhantomData,
        })
    }
}

impl ResolveTransaction for Transaction<Dispute> {
    fn resolve(&self, account: &mut Account) -> Result<Transaction<Resolve>> {
        account.hold(self.amount)?;
        Ok(Transaction {
            client_id: self.client_id,
            tx_id: self.tx_id,
            amount: self.amount,
            _state: PhantomData,
        })
    }
}

impl ChargebackTransaction for Transaction<Dispute> {
    fn chargeback(&self, account: &mut Account) -> Result<Transaction<Chargeback>> {
        account.chargeback(self.amount)?;
        Ok(Transaction {
            client_id: self.client_id,
            tx_id: self.tx_id,
            amount: self.amount,
            _state: PhantomData,
        })
    }
}

pub struct Transaction<State> {
    pub client_id: u16,
    pub tx_id: u32,
    pub amount: f64,
    _state: PhantomData<State>,
}

// Common transaction methods will go here
impl<State> Transaction<State> {}

// Only withdrawal and deposit transactions can be newly created
impl Transaction<Withdrawal> {
    pub fn new_withdrawal(
        client_id: u16,
        tx_id: u32,
        amount: f64,
        account: &mut Account,
    ) -> Result<Transaction<Withdrawal>> {
        account.withdraw(amount)?;
        Ok(Self {
            client_id,
            tx_id,
            amount,
            _state: PhantomData,
        })
    }
}

impl Transaction<Deposit> {
    pub fn new_deposit(
        client_id: u16,
        tx_id: u32,
        amount: f64,
        account: &mut Account,
    ) -> Result<Transaction<Deposit>> {
        account.deposit(amount)?;
        Ok(Self {
            client_id,
            tx_id,
            amount,
            _state: PhantomData,
        })
    }
}
