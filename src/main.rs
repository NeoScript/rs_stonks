use std::{collections::HashMap, fs, path::Path, time::Instant};

use crate::{
    data_models::{
        CategoryProcessingResults, CompanyInformation, StockCategory, seed_category_statistics,
    },
    third_party::do_some_processing,
};
pub mod data_models;
pub mod third_party;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data_file_path: &Path = Path::new("src/stock_data.json");
    let json_str = fs::read_to_string(data_file_path).expect("should be able to read file");
    let company_data: Vec<CompanyInformation> =
        serde_json::from_str(&json_str).expect("should be able to deserialize json str");
    let mut category_statistics: HashMap<StockCategory, CategoryProcessingResults> = HashMap::new();
    seed_category_statistics(&mut category_statistics);

    println!("{:?}", category_statistics);

    let start_time = Instant::now();
    for c in company_data {
        let result = do_some_processing(&c);
        let category_to_lookup: Result<StockCategory, &str> = c.category.as_str().try_into();

        if category_to_lookup.is_err() {
            println!("Failed to determine category, skipping: {:?}", c);
            continue;
        }

        let category_to_lookup = category_to_lookup.expect("should have determined category");
        match result {
            Ok(_) => {
                let reference = category_statistics
                    .get_mut(&category_to_lookup)
                    .expect("should find category");
                reference.processed += 1;
            }
            Err(_) => {
                let reference = category_statistics
                    .get_mut(&category_to_lookup)
                    .expect("should find category");
                reference.failed += 1;
            }
        }
    }

    let end_time = Instant::now();
    let total_processing_time = end_time.duration_since(start_time);
    println!("total_processing_time = {:?}", total_processing_time);

    println!("results");
    for (key, val) in category_statistics.iter() {
        println!("Category: {:?} -> Results: {:?}", key, val);
    }
    Ok(())
}
