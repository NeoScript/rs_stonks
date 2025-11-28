use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CompanyInformation {
    pub symbol: String,
    pub category: String,
}

#[derive(Debug, Default)]
pub struct CategoryProcessingResults {
    pub processed: usize,
    pub failed: usize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum StockCategory {
    CategoryA,
    CategoryB,
    CategoryC,
    CategoryD,
    CategoryE,
}

impl TryFrom<&str> for StockCategory {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" => Ok(Self::CategoryA),
            "B" => Ok(Self::CategoryB),
            "C" => Ok(Self::CategoryC),
            "D" => Ok(Self::CategoryD),
            "E" => Ok(Self::CategoryE),
            _ => Err("unknown category input"),
        }
    }
}

pub fn seed_category_statistics(
    category_stats: &mut HashMap<StockCategory, CategoryProcessingResults>,
) {
    category_stats.insert(
        "A".try_into().expect("should translate A to enum"),
        CategoryProcessingResults::default(),
    );
    category_stats.insert(
        "B".try_into().expect("should translate B to enum"),
        CategoryProcessingResults::default(),
    );

    category_stats.insert(
        "C".try_into().expect("should translate C to enum"),
        CategoryProcessingResults::default(),
    );
    category_stats.insert(
        "D".try_into().expect("should translate D"),
        CategoryProcessingResults::default(),
    );
    category_stats.insert(
        "E".try_into().expect("should translate E"),
        CategoryProcessingResults::default(),
    );
}
