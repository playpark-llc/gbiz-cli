use anyhow::Result;
use serde_json::Value;

use crate::client::{GbizClient, validate_corporate_number};

pub async fn get_detail(client: &GbizClient, corporate_number: &str) -> Result<Value> {
    validate_corporate_number(corporate_number)?;
    client.get_detail(corporate_number).await
}

pub async fn get_category(
    client: &GbizClient,
    corporate_number: &str,
    category: &str,
) -> Result<Value> {
    validate_corporate_number(corporate_number)?;
    client.get_category(corporate_number, category).await
}
