use anyhow::Result;
use serde_json::Value;

use crate::client::{GbizClient, SearchParams};

pub async fn search(client: &GbizClient, params: &SearchParams) -> Result<Value> {
    client.search(params).await
}
