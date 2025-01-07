Overview:

The Rust CSV Data Transformer is a lightweight tool for processing and transforming CSV files. It filters and sorts records based on specific criteria, demonstrating the power of Rust for data handling tasks. In this implementation:

Records are filtered by city.

Filtered records are then sorted by age.

Features:

Filter Records: Extracts rows based on a specific city.
Sort Records: Orders the rows by age in ascending order.
Error Handling: Gracefully handles CSV reading/writing errors.
Modular Design: Separation of concerns with dedicated modules for CSV handling and transformation logic.

Prerequisites:

System Requirements:
Rust Toolchain: Install Rust using Rustup.
Libraries Used
The project leverages the following Rust crates:
csv: For reading and writing CSV files.
serde: For parsing CSV records into typed structs.
anyhow: For streamlined error handling.

Install dependencies with: cargo add csv serde anyhow

Files and Structure:

src/main.rs: The main entry point of the program.
csv_handler.rs: Handles CSV file operations (reading and writing).
transformer.rs: Contains the transformation logic for filtering and sorting records.
Input CSV: An example input file (e.g., data/example.csv).
Output CSV: The resulting transformed file (e.g., data/output.csv).
