//! ./external_coupling.rs
//!
//! External coupling occurs when a module's behavior is dependent on
//! an externally imposed data format, communication protocol, or hardware interface
//!
//! Problem

mod data_loader {
    use serde::Deserialize;
    use std::fs::File;

    #[derive(Deserialize, Debug)]
    pub struct Record {
        pub id: u32,
        pub data: f64,
    }

    pub fn load_data(file_path: &str) -> Result<Vec<Record>, csv::Error> {
        let file = File::open(file_path)?;
        let mut rdr = csv::Reader::from_reader(file);
        let mut records = Vec::new();
        for result in rdr.deserialize() {
            let record: Record = result?;
            records.push(record);
        }
        Ok(records)
    }
}

mod data_processor {
    use super::data_loader::Record;

    pub fn process_records(records: &[Record]) -> f64 {
        let sum: f64 = records.iter().map(|rec| rec.value).sum();
        sum / (records.len() as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::{data_loader, data_processor};

    #[test]
    fn data_pipeline_from_csv_file() {
        // Both modules depend on the same external format/library.
        let file_path = "data.csv";
        // `data_loader` is responsible for interacting with the external format.
        match data_loader::load_data(file_path) {
            Ok(records) => {
                // `data_processor` depends on the data structure defined for the external format.
                let average = data_processor::process_records(&records);
                println!("Average value: {}", average);
            }
            Err(e) => {
                eprintln!("Error loading data: {}", e);
            }
        }
    }
}

///Solution: Adpater pattern
mod app_models {
    // This is the application's own, internal data structure.
    pub struct AppRecord {
        pub value: f64,
    }
}

mod parser {
    use super::app_models::AppRecord;
    use serde::Deserialize;
    use std::fs::File;

    // This internal struct is only for reading the external format.
    #[derive(Deserialize, Debug)]
    struct CsvRecord {
        id: u32, // Note: This internal field can change without affecting other modules.
        value: f64,
    }

    pub fn load_records_from_csv(file_path: &str) -> Result<Vec<AppRecord>, csv::Error> {
        let file = File::open(file_path)?;
        let mut rdr = csv::Reader::from_reader(file);
        let mut records = Vec::new();
        for result in rdr.deserialize() {
            let record: CsvRecord = result?;
            // Map the external format to the internal application format.
            records.push(AppRecord {
                value: record.value,
            });
        }
        Ok(records)
    }
}

mod data_processor_v2 {
    use super::app_models::AppRecord;

    pub fn process_records(records: &[AppRecord]) -> f64 {
        let sum: f64 = records.iter().map(|rec| rec.value).sum();
        sum / (records.len() as f64)
    }
}
