use chrono::NaiveDateTime;
use serde::{Serialize, Serializer};

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

fn serialize_date<S>(date: &NaiveDateTime, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let date_str = format!("{}", date.format("%d.%m.%Y %H:%M:%S"));
    s.serialize_str(&date_str)
}
