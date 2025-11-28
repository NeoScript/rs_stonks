use serde::Deserialize;

#[derive(Deserialize)]
pub struct CompanyInformation {
    pub symbol: String,
    pub category: String,
}
