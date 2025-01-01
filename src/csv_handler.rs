use csv::{ReaderBuilder, WriterBuilder};
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize, serde::Serialize)]
pub struct Record {
    pub id: u32,
    pub name: String,
    pub age: u32,
    pub city: String,
    pub income: u32,
    pub employed: String,
    pub department: String,
}

pub fn read_csv(file_path: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().from_path(file_path)?;
    let mut records = Vec::new();

    for result in reader.deserialize() {
        let record: Record = result?;
        records.push(record);
    }

    Ok(records)
}

pub fn write_csv(file_path: &str, data: Vec<Record>) -> Result<(), Box<dyn Error>> {
    let mut writer = WriterBuilder::new().from_path(file_path)?;

    for record in data {
        writer.serialize(record)?;
    }

    writer.flush()?;
    Ok(())
}
