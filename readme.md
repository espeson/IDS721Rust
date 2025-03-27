# Show Polars

**Show Polars** is a simple command-line tool built in Rust that leverages the [Polars](https://www.pola.rs/) library for CSV data ingestion and basic data processing. This project demonstrates how to read CSV files, extract key information (such as the shape, schema, and a preview of the data), and includes unit tests to ensure code quality.

## Features

- **CSV Data Ingestion**: Reads data from a CSV file and constructs a DataFrame.
- **Data Processing**: Outputs the DataFrame's shape (number of rows and columns), its schema, and the first 5 rows of data.
- **Unit Testing**: Includes test cases to validate CSV reading and data processing functionalities.

## Dependencies

- [Rust](https://www.rust-lang.org/) (latest stable version recommended)
- [Cargo](https://doc.rust-lang.org/cargo/)
- [clap](https://crates.io/crates/clap) - For parsing command-line arguments
- [polars](https://crates.io/crates/polars) - For data processing

## Installation and Build

1. **Install Rust**: Follow the instructions on the [Rust website](https://www.rust-lang.org/tools/install) to install Rust and Cargo.
2. **Clone the Repository**:

   ```bash
   git clone <your-repository-url>
   cd show_polars
   ```
3. Build the Project

   ```
   cargo build --release
   ```

## Usage

**Prepare a CSV File** : Create a CSV file (e.g., `sample.csv`) in the project root directory. Example content:

```
name,age
Alice,30
Bob,25
Charlie,40
David,35
Eve,28
Frank,33
```

Run the Program

```
cargo run -- --input sample.csv

```
