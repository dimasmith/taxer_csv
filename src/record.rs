//! Data records to serialize to Taxer format.

use crate::value::{Amount, TaxCode};
use chrono::NaiveDateTime;
use serde::{Serialize, Serializer};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InvalidRecord {
    #[error("missing tax code")]
    MissingTaxCode,
    #[error("missing date")]
    MissingDate,
    #[error("missing amount")]
    MissingAmount,
    #[error("invalid amount: amount must be a positive number, but was {0}")]
    InvalidAmount(f64),
    #[error("invalid tax code: the tax code must contain 8 or 10 digits, but was {0}")]
    InvalidTaxCode(String),
}

/// Taxer record with all supported fields.
#[derive(Debug, Clone, Serialize)]
pub struct TaxerRecord {
    pub tax_code: TaxCode,
    #[serde(serialize_with = "serialize_date")]
    pub date: NaiveDateTime,
    pub amount: Amount,
    pub comment: String,
    pub operation: String,
    pub income_type: String,
    pub account_name: String,
    pub currency_code: String,
}

impl TaxerRecord {
    /// Create record with required data. Other fields will be empty.
    pub fn new(
        tax_code: TaxCode,
        date: NaiveDateTime,
        amount: Amount,
        comment: impl Into<String>,
    ) -> Self {
        TaxerRecord {
            tax_code,
            date,
            amount,
            comment: comment.into(),
            account_name: String::default(),
            currency_code: "UAH".to_string(),
            operation: String::default(),
            income_type: String::default(),
        }
    }

    /// Returns a builder for TaxerRecord.
    pub fn builder() -> TaxerRecordBuilder {
        TaxerRecordBuilder::default()
    }
}

/// Builder for TaxerRecord.
#[derive(Debug, Default)]
pub struct TaxerRecordBuilder {
    tax_code: Option<TaxCode>,
    date: Option<NaiveDateTime>,
    amount: Option<Amount>,
    comment: Option<String>,
    operation: Option<String>,
    income_type: Option<String>,
    account_name: Option<String>,
    currency_code: Option<String>,
}

impl TaxerRecordBuilder {
    pub fn tax_code(mut self, tax_code: TaxCode) -> Self {
        self.tax_code = Some(tax_code);
        self
    }

    pub fn tax_code_raw(
        mut self,
        tax_code: impl Into<String>,
    ) -> Result<TaxerRecordBuilder, InvalidRecord> {
        let raw_code = tax_code.into();
        self.tax_code = Some(
            TaxCode::new(raw_code).map_err(|e| InvalidRecord::InvalidTaxCode(e.invalid_code))?,
        );
        Ok(self)
    }
    pub fn date(mut self, date: NaiveDateTime) -> Self {
        self.date = Some(date);
        self
    }
    pub fn amount(mut self, amount: Amount) -> Self {
        self.amount = Some(amount);
        self
    }

    pub fn amount_raw(mut self, amount: f64) -> Result<Self, InvalidRecord> {
        self.amount =
            Some(Amount::new(amount).map_err(|e| InvalidRecord::InvalidAmount(e.invalid_amount))?);
        Ok(self)
    }
    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.comment = Some(comment.into());
        self
    }
    pub fn operation(mut self, operation: impl Into<String>) -> Self {
        self.operation = Some(operation.into());
        self
    }
    pub fn income_type(mut self, income_type: impl Into<String>) -> Self {
        self.income_type = Some(income_type.into());
        self
    }
    pub fn account_name(mut self, account_name: impl Into<String>) -> Self {
        self.account_name = Some(account_name.into());
        self
    }
    pub fn currency_code(mut self, currency_code: impl Into<String>) -> Self {
        self.currency_code = Some(currency_code.into());
        self
    }
    pub fn build(self) -> Result<TaxerRecord, InvalidRecord> {
        Ok(TaxerRecord {
            tax_code: self.tax_code.ok_or(InvalidRecord::MissingTaxCode)?,
            date: self.date.ok_or(InvalidRecord::MissingDate)?,
            amount: self.amount.ok_or(InvalidRecord::MissingAmount)?,
            comment: self.comment.unwrap_or_default(),
            operation: self.operation.unwrap_or_default(),
            income_type: self.income_type.unwrap_or_default(),
            account_name: self.account_name.unwrap_or_default(),
            currency_code: self.currency_code.unwrap_or_default(),
        })
    }
}

const TAXER_DATE_FORMAT: &str = "%d.%m.%Y %H:%M:%S";

fn serialize_date<S>(date: &NaiveDateTime, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let date_str = format!("{}", date.format(TAXER_DATE_FORMAT));
    s.serialize_str(&date_str)
}
