use std::{error::Error, thread, time::Duration};

use crate::data_models::{CompanyInformation, StockCategory};

pub fn do_some_processing(company_info: &CompanyInformation) -> Result<u64, Box<dyn Error>> {
    let min_sleep_time = 100;
    let max_sleep_time = 700;

    let timeout_after = 600;

    let random_sleep_timer = rand::random_range(min_sleep_time..=max_sleep_time);
    if random_sleep_timer > timeout_after {
        return Err("Timed out".into());
    }

    println!(
        "doing some processing for: {}, {}",
        company_info.symbol, company_info.category
    );
    thread::sleep(Duration::from_millis(random_sleep_timer));
    let _mapped_category: StockCategory = company_info.category.as_str().try_into()?;

    println!(
        "got a result for: {}, {} in {} ms",
        company_info.symbol, company_info.category, random_sleep_timer
    );
    Ok(random_sleep_timer)
}
