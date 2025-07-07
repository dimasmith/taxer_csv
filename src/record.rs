//! Data records to serialize to Taxer format.

use chrono::NaiveDateTime;
use serde::{Serialize, Serializer};

/// Taxer record with all supported fields.
#[derive(Debug, Clone, Serialize, Default)]
pub struct TaxerRecord {
    pub tax_code: String,
    #[serde(serialize_with = "serialize_date")]
    pub date: NaiveDateTime,
    pub amount: f64,
    pub comment: String,
    pub operation: String,
    pub income_type: String,
    pub account_name: String,
    pub currency_code: String,
}

impl TaxerRecord {
    /// Create record with required data. Other fields will be empty.
    pub fn new(
        tax_code: impl Into<String>,
        date: NaiveDateTime,
        amount: f64,
        comment: impl Into<String>,
    ) -> Self {
        TaxerRecord {
            tax_code: tax_code.into(),
            date,
            amount,
            comment: comment.into(),
            ..Default::default()
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
    tax_code: Option<String>,
    date: Option<NaiveDateTime>,
    amount: Option<f64>,
    comment: Option<String>,
    operation: Option<String>,
    income_type: Option<String>,
    account_name: Option<String>,
    currency_code: Option<String>,
}

impl TaxerRecordBuilder {
    pub fn tax_code(mut self, tax_code: impl Into<String>) -> Self {
        self.tax_code = Some(tax_code.into());
        self
    }
    pub fn date(mut self, date: NaiveDateTime) -> Self {
        self.date = Some(date);
        self
    }
    pub fn amount(mut self, amount: f64) -> Self {
        self.amount = Some(amount);
        self
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
    pub fn build(self) -> Result<TaxerRecord, &'static str> {
        Ok(TaxerRecord {
            tax_code: self.tax_code.ok_or("tax_code is required")?,
            date: self.date.ok_or("date is required")?,
            amount: self.amount.ok_or("amount is required")?,
            comment: self.comment.unwrap_or_default(),
            operation: self.operation.unwrap_or_default(),
            income_type: self.income_type.unwrap_or_default(),
            account_name: self.account_name.unwrap_or_default(),
            currency_code: self.currency_code.unwrap_or_default(),
        })
    }
}

fn serialize_date<S>(date: &NaiveDateTime, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let date_str = format!("{}", date.format("%d.%m.%Y %H:%M:%S"));
    s.serialize_str(&date_str)
}
