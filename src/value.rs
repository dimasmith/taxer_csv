//! Reusable validated value objects.
//!
//! Records are composed of those object to prevent accepting invalid values.

use std::borrow::Cow;
use std::cmp::Ordering;
use serde::Serialize;
use thiserror::Error;

const MAX_AMOUNT: f64 = 10000000.;

#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
pub struct Amount(f64);

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TaxCode(String);

#[derive(Debug, Clone, Error)]
#[error("invalid amount {invalid_amount}")]
pub struct AmountError {
    pub invalid_amount: f64,
}

#[derive(Debug, Clone, Error)]
#[error("valid tax code must be 8 or 10 digits {invalid_code}")]
pub struct TaxCodeError {
    pub invalid_code: String,
}

impl Amount {
    pub fn new(raw: f64) -> Result<Self, AmountError> {
        let acceptable_amounts = 0.01..MAX_AMOUNT;
        if !acceptable_amounts.contains(&raw) {
            return Err(AmountError {invalid_amount: raw});
        }
        Ok(Amount(raw))
    }

    pub fn raw(&self) -> f64 {
        self.0
    }
}

impl Eq for Amount {}
impl PartialOrd for Amount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Amount {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.0.partial_cmp(&other.0) {
            Some(ordering) => ordering,
            None => unreachable!(),
        }
    }
}

impl TryFrom<f64> for Amount {
    type Error = AmountError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Amount::new(value)
    }
}

impl TaxCode {
    pub fn new(raw: Cow<String>) -> Result<Self, TaxCodeError> {
        if raw.len() != 8 && raw.len() != 10 {
            return Err(TaxCodeError { invalid_code: raw.to_string() });
        }
        Ok(TaxCode(raw.to_string()))
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl AsRef<str> for TaxCode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}