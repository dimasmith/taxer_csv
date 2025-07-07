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
}

fn serialize_date<S>(date: &NaiveDateTime, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let date_str = format!("{}", date.format("%d.%m.%Y %H:%M:%S"));
    s.serialize_str(&date_str)
}
