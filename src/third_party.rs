use std::{error::Error, thread, time::Duration};

use crate::data_models::{CompanyInformation, StockCategory};

pub fn do_some_processing(company_info: &CompanyInformation) -> Result<u64, Box<dyn Error>> {
    let min_sleep_time = 100;
    let max_sleep_time = 500;

    let timeout_after = ((max_sleep_time as f64) * 0.90) as u64;

    let random_sleep_timer = rand::random_range(min_sleep_time..max_sleep_time);
    if random_sleep_timer > timeout_after {
        return Err("Timed out".into());
    }

    println!(
        "doing some processing for: {}, {}",
        company_info.symbol, company_info.category
    );
    thread::sleep(Duration::from_millis(random_sleep_timer));

    let _mapped_category: StockCategory = company_info.category.as_str().try_into()?;

    Ok(random_sleep_timer)
}
