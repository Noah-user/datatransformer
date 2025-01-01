use super::csv_handler::Record;

pub fn filter_records_by_city(records: Vec<Record>, city: &str) -> Vec<Record> {
    records
        .into_iter()
        .filter(|record| record.city.to_lowercase() == city.to_lowercase())
        .collect()
}

pub fn sort_records_by_age(mut records: Vec<Record>) -> Vec<Record> {
    records.sort_by(|a, b| a.age.cmp(&b.age));
    records
}
