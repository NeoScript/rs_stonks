use std::{fs, path::Path, time::Instant};

use crate::data_models::CompanyInformation;
pub mod data_models;

fn main() {
    let data_file_path: &Path = Path::new("src/stock_data.json");
    let json_str = fs::read_to_string(&data_file_path).expect("should be able to read file");
    let company_data: Vec<CompanyInformation> =
        serde_json::from_str(&json_str).expect("should be able to deserialize json str");

    for c in company_data {
        println!("symbol: {}, category: {}", c.symbol, c.category);
    }
}
