//! Functions to write CSV

use std::io::Write;

use csv::WriterBuilder;
use thiserror::Error;

use crate::record::TaxerRecord;

#[derive(Debug, Error)]
pub enum TaxerError {
    #[error("failed to serialize taxer records. faulty record {record_no}")]
    Csv {
        record_no: usize,
        source: csv::Error,
    },
}

/// Serialize list of taxer records to CSV format suitable for import.
pub fn serialize_taxer<W>(writer: W, records: &[TaxerRecord]) -> Result<(), TaxerError>
where
    W: Write,
{
    let mut csv_writer = WriterBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .from_writer(writer);
    for (idx, record) in records.iter().enumerate() {
        csv_writer.serialize(record).map_err(|e| TaxerError::Csv {
            record_no: idx + 1,
            source: e,
        })?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::BufWriter;

    use chrono::NaiveDateTime;

    use crate::TaxerRecord;

    #[test]
    fn serialize_single_record() {
        let date =
            NaiveDateTime::parse_from_str("2025-07-22 13:24:35", "%Y-%m-%d %H:%M:%S").unwrap();
        let record = TaxerRecord::new("3141592600", date, 220394.05, "Послуги з розробки");

        let buf = vec![];
        let mut w = BufWriter::new(buf);

        serialize_taxer(&mut w, &[record]).unwrap();

        let buf = w.into_inner().unwrap();
        let raw_csv = String::from_utf8(buf).unwrap();
        assert_eq!(
            raw_csv,
            "3141592600,22.07.2025 13:24:35,220394.05,Послуги з розробки,,,,\n"
        );
    }

    #[test]
    fn serialize_complete_record() {
        let date =
            NaiveDateTime::parse_from_str("2025-07-22 13:24:35", "%Y-%m-%d %H:%M:%S").unwrap();
        let record = TaxerRecord::builder()
            .tax_code("2121049841")
            .date(date)
            .amount(220394.05)
            .comment("Послуги з розробки")
            .operation("Дохід")
            .income_type("Основний дохід")
            .account_name("ФОП")
            .currency_code("UAH")
            .build()
            .unwrap();

        let buf = vec![];
        let mut w = BufWriter::new(buf);

        serialize_taxer(&mut w, &[record]).unwrap();

        let buf = w.into_inner().unwrap();
        let raw_csv = String::from_utf8(buf).unwrap();
        assert_eq!(
            raw_csv,
            "2121049841,22.07.2025 13:24:35,220394.05,Послуги з розробки,Дохід,Основний дохід,ФОП,UAH\n"
        );
    }
}
