mod csv_handler;
mod transformer;

use csv_handler::{read_csv, write_csv};
use transformer::{filter_records_by_city, sort_records_by_age};
use std::process;

fn main() {
    let input_file = "data/example.csv";
    let output_file = "data/output.csv";
    let filter_city = "New York"; // Adjust this to test filtering

    match read_csv(input_file) {
        Ok(records) => {
            // Apply filtering and sorting
            let filtered_records = filter_records_by_city(records, filter_city);
            let sorted_records = sort_records_by_age(filtered_records);

            // Write the transformed data to a new CSV file
            if let Err(err) = write_csv(output_file, sorted_records) {
                eprintln!("Error writing CSV: {}", err);
                process::exit(1);
            }

            println!("Output written to {}", output_file);
        }
        Err(err) => {
            eprintln!("Error reading CSV: {}", err);
            process::exit(1);
        }
    }
}
