use std::{collections::HashMap, fs, path::Path, process::exit, sync::Arc, time::Instant};

use tokio::{sync::Mutex, task::JoinSet};

use crate::{
    data_models::{
        CategoryProcessingResults, CompanyInformation, StockCategory, seed_category_statistics,
    },
    third_party::do_some_processing,
};
pub mod data_models;
pub mod third_party;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data_file_path: &Path = Path::new("src/stock_data.json");
    let json_str = fs::read_to_string(data_file_path).expect("should be able to read file");
    let company_data: Vec<CompanyInformation> =
        serde_json::from_str(&json_str).expect("should be able to deserialize json str");
    let category_statistics: Arc<Mutex<HashMap<StockCategory, CategoryProcessingResults>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let mut guarded_stats = category_statistics.lock().await;
    seed_category_statistics(&mut guarded_stats);
    println!("{:?}", guarded_stats);

    let start_time = Instant::now();

    let mut tasks = JoinSet::new();
    for c in company_data {
        let guarded_stats = Arc::clone(&category_statistics);
        tasks.spawn(async move {
            let result = do_some_processing(&c).is_ok();
            let category_to_lookup: Result<StockCategory, &str> = c.category.as_str().try_into();

            if category_to_lookup.is_err() {
                println!("Failed to determine category, skipping: {:?}", c);
                return;
            }

            let category_to_lookup = category_to_lookup.expect("should have determined category");
            match result {
                true => {
                    let mut locked_stats = guarded_stats.lock().await;
                    locked_stats
                        .get_mut(&category_to_lookup)
                        .expect("should find category")
                        .processed += 1;
                }
                false => {
                    let mut locked_stats = guarded_stats.lock().await;
                    locked_stats
                        .get_mut(&category_to_lookup)
                        .expect("should find category")
                        .processed += 1;
                }
            }
        });
    }

    tasks.join_all().await;
    let end_time = Instant::now();
    let total_processing_time = end_time.duration_since(start_time);
    println!("total_processing_time = {:?}", total_processing_time);

    println!("results");
    let category_statistics = category_statistics.lock().await;
    for (key, val) in category_statistics.iter() {
        println!("Category: {:?} -> Results: {:?}", key, val);
    }
    Ok(())
}
