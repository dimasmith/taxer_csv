//! Reusable validated value objects.
//!
//! Records are composed of those object to prevent accepting invalid values.

use serde::Serialize;
use std::cmp::Ordering;
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
            return Err(AmountError {
                invalid_amount: raw,
            });
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
    pub fn new(raw: impl Into<String>) -> Result<Self, TaxCodeError> {
        let raw = raw.into();
        if raw.len() != 8 && raw.len() != 10 {
            return Err(TaxCodeError { invalid_code: raw });
        }
        if raw.chars().any(|c| !c.is_ascii_digit()) {
            return Err(TaxCodeError { invalid_code: raw });
        }
        Ok(TaxCode(raw))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accept_valid_tax_code() {
        assert!(TaxCode::new("12345678").is_ok());
        assert!(TaxCode::new("1234567890").is_ok());
    }

    #[test]
    fn reject_tax_code_of_invalid_length() {
        assert!(TaxCode::new("123").is_err());
        assert!(TaxCode::new("").is_err());
        assert!(TaxCode::new("12345678901").is_err());
        assert!(TaxCode::new("123456789").is_err());
        assert!(TaxCode::new("1234567").is_err());
    }

    #[test]
    fn reject_tax_code_of_non_digits() {
        assert!(TaxCode::new("1234567a").is_err());
        assert!(TaxCode::new("1234 567").is_err());
        assert!(TaxCode::new("1234-56789").is_err());
        assert!(TaxCode::new("-123456789").is_err());
        assert!(TaxCode::new("x123456789").is_err());
    }

    #[test]
    fn accept_valid_amount() {
        assert!(Amount::new(0.01).is_ok());
        assert!(Amount::new(1.0).is_ok());
        assert!(Amount::new(9999999.99).is_ok());
        assert!(Amount::new(MAX_AMOUNT - 0.01).is_ok());
    }

    #[test]
    fn reject_amounts_out_of_bounds() {
        // Less than minimum
        assert!(Amount::new(-1.0).is_err());
        assert!(Amount::new(-100.0).is_err());
        // Equal to minimum bound (exclusive)
        assert!(Amount::new(0.0).is_err());
        // Equal to or greater than max bound
        assert!(Amount::new(MAX_AMOUNT).is_err());
        assert!(Amount::new(MAX_AMOUNT + 1.0).is_err());
        assert!(Amount::new(1e12).is_err());
    }

    #[test]
    fn reject_amounts_special_cases() {
        assert!(Amount::new(f64::NAN).is_err());
        assert!(Amount::new(f64::INFINITY).is_err());
        assert!(Amount::new(f64::NEG_INFINITY).is_err());
    }
}
