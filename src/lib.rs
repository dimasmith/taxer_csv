//! A simple library to save CSV records in [Taxer](https://taxer.ua/uk/kb/import-vipiski-v-csv-fajl-samostijne-stvorennya-csv-fajlu) format.

pub mod csv;
pub mod record;
pub mod value;

pub use csv::serialize_taxer;
pub use record::TaxerRecord;
