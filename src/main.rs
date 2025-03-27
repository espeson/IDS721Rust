use clap::Parser;
use polars::prelude::*;
use std::error::Error;

#[derive(Parser)]
#[command(author, version, about = "Rust CML - Show Polars Simple", long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,
}

fn read_csv(path: &str) -> Result<DataFrame, Box<dyn Error>> {
    let file = std::fs::File::open(path)?;
    let df = CsvReader::new(file)
        .infer_schema(Some(100))
        .has_header(true)
        .finish()?;
    Ok(df)
}

fn process_data(df: &DataFrame) -> String {
    let shape = format!("Shape: ({}, {})", df.height(), df.width());
    let schema = format!("Schema: {:?}", df.schema());
    let head = format!("Head:\n{}", df.head(Some(5)));
    format!("{}\n{}\n{}", shape, schema, head)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let df = read_csv(&args.input)?;
    println!("Original Data:\n{}", df);

    let output = process_data(&df);
    println!("Processed Data:\n{}", output);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use polars::prelude::*;

    #[test]
    fn test_read_csv_from_string() {
        let csv_data = "name,age\nAlice,30\nBob,25\n";
        let cursor = std::io::Cursor::new(csv_data);
        let df = CsvReader::new(cursor)
            .infer_schema(Some(10))
            .has_header(true)
            .finish()
            .expect("CSV reading failed");
        assert_eq!(df.height(), 2);
        assert_eq!(df.width(), 2);
    }

    #[test]
    fn test_process_data() {
        let s1 = Series::new("name", &["Alice", "Bob", "Charlie"]);
        let s2 = Series::new("age", &[30, 25, 40]);
        let df = DataFrame::new(vec![s1, s2]).expect("DataFrame creation failed");
        let result = process_data(&df);
        assert!(result.contains("Shape: (3, 2)"));
        assert!(result.contains("Schema:"));
        assert!(result.contains("Head:"));
    }
}
