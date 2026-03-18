use anyhow::Result;
use std::fmt;
use std::fmt::Display;

/// Utility to round amount to 4 decimal places
#[inline]
pub fn round_to_4(v: f64) -> f64 {
    (v * 10_000.0).round() / 10_000.0
}

#[derive(Debug, Clone)]
pub struct Account {
    available: f64,
    held: f64,
    locked: bool,
}

impl Default for Account {
    fn default() -> Self {
        Self {
            available: 0.0,
            held: 0.0,
            locked: false,
        }
    }
}

impl Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{},{},{},{}",
            self.available(),
            self.held(),
            self.total(),
            self.is_locked()
        )
    }
}

impl Account {
    pub fn total(&self) -> f64 {
        round_to_4(self.available() + self.held())
    }

    pub fn withdraw(&mut self, amount: f64) -> Result<()> {
        if self.locked {
            return Err(anyhow::anyhow!("Account is locked"));
        }

        if self.available < amount {
            return Err(anyhow::anyhow!("Insufficient funds to withdraw!"));
        }
        self.available -= round_to_4(amount);
        Ok(())
    }

    pub fn deposit(&mut self, amount: f64) -> Result<()> {
        if self.locked {
            return Err(anyhow::anyhow!("Account is locked"));
        }
        self.available += round_to_4(amount);
        Ok(())
    }

    pub fn hold(&mut self, amount: f64) -> Result<()> {
        if self.locked {
            return Err(anyhow::anyhow!("Account is locked"));
        }
        self.available -= amount;
        self.held += amount;
        Ok(())
    }

    pub fn resolve(&mut self, amount: f64) -> Result<()> {
        if self.locked {
            return Err(anyhow::anyhow!("Account is locked!"));
        }

        self.held -= amount;
        self.available += amount;
        Ok(())
    }

    pub fn chargeback(&mut self, amount: f64) -> Result<()> {
        if self.locked {
            return Err(anyhow::anyhow!("Account is already locked!"));
        }

        self.held -= amount;
        self.available -= amount;
        self.locked = true;
        Ok(())
    }

    pub fn available(&self) -> f64 {
        round_to_4(self.available)
    }

    pub fn held(&self) -> f64 {
        round_to_4(self.held)
    }

    pub fn lock(&mut self) {
        if self.locked {
            println!("Account is already locked!");
        } else {
            self.locked = true;
        }
    }

    pub fn is_locked(&self) -> bool {
        self.locked
    }

    pub fn unlock(&mut self) {
        if !self.locked {
            println!("Account is already unlocked!");
        } else {
            self.locked = false;
        }
    }
}
