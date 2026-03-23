use anyhow::{Context, Result, bail};
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::Value;

const API_BASE: &str = "https://api.info.gbiz.go.jp/hojin";

pub struct SearchParams {
    pub name: String,
    pub prefecture: Option<String>,
    pub capital_from: Option<u64>,
    pub capital_to: Option<u64>,
    pub employee_from: Option<u32>,
    pub employee_to: Option<u32>,
    pub limit: u32,
    pub page: u32,
}

pub struct GbizClient {
    http: reqwest::Client,
    base_url: String,
}

impl GbizClient {
    pub fn new(token: &str) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "X-hojinInfo-api-token",
            HeaderValue::from_str(token).context("invalid token characters")?,
        );

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .context("failed to build HTTP client")?;

        Ok(Self {
            http,
            base_url: API_BASE.to_string(),
        })
    }

    pub async fn search(&self, params: &SearchParams) -> Result<Value> {
        let url = format!("{}/v2/hojin", self.base_url);

        let encoded_name = urlencoding::encode(&params.name);
        let mut query: Vec<(&str, String)> = vec![
            ("name", encoded_name.into_owned()),
            ("limit", params.limit.to_string()),
            ("page", params.page.to_string()),
        ];

        if let Some(ref pref) = params.prefecture {
            query.push(("prefecture", pref.clone()));
        }
        if let Some(v) = params.capital_from {
            query.push(("capital_stock_from", v.to_string()));
        }
        if let Some(v) = params.capital_to {
            query.push(("capital_stock_to", v.to_string()));
        }
        if let Some(v) = params.employee_from {
            query.push(("employee_number_from", v.to_string()));
        }
        if let Some(v) = params.employee_to {
            query.push(("employee_number_to", v.to_string()));
        }

        let resp = self.http.get(&url).query(&query).send().await?;
        self.handle_response(resp).await
    }

    pub async fn get_detail(&self, corporate_number: &str) -> Result<Value> {
        let url = format!("{}/v2/hojin/{corporate_number}", self.base_url);
        let resp = self.http.get(&url).send().await?;
        self.handle_response(resp).await
    }

    pub async fn get_category(&self, corporate_number: &str, category: &str) -> Result<Value> {
        let url = format!(
            "{}/v2/hojin/{corporate_number}/{category}",
            self.base_url
        );
        let resp = self.http.get(&url).send().await?;
        self.handle_response(resp).await
    }

    async fn handle_response(&self, resp: reqwest::Response) -> Result<Value> {
        let status = resp.status();
        let body: Value = resp.json().await.context("failed to parse response JSON")?;

        if !status.is_success() {
            let message = body
                .get("message")
                .and_then(Value::as_str)
                .unwrap_or("unknown error");
            bail!("API error (HTTP {status}): {message}");
        }

        Ok(body)
    }

    #[cfg(test)]
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}

pub fn validate_corporate_number(number: &str) -> Result<()> {
    if number.len() != 13 {
        bail!("法人番号は13桁である必要があります (入力: {}桁)", number.len());
    }
    if !number.chars().all(|c| c.is_ascii_digit()) {
        bail!("法人番号は数字のみで構成される必要があります");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_corporate_number_valid() {
        assert!(validate_corporate_number("1234567890123").is_ok());
    }

    #[test]
    fn test_validate_corporate_number_too_short() {
        let err = validate_corporate_number("123456").unwrap_err();
        assert!(err.to_string().contains("13桁"));
    }

    #[test]
    fn test_validate_corporate_number_too_long() {
        let err = validate_corporate_number("12345678901234").unwrap_err();
        assert!(err.to_string().contains("13桁"));
    }

    #[test]
    fn test_validate_corporate_number_non_digit() {
        let err = validate_corporate_number("123456789012a").unwrap_err();
        assert!(err.to_string().contains("数字のみ"));
    }

    #[test]
    fn test_validate_corporate_number_empty() {
        let err = validate_corporate_number("").unwrap_err();
        assert!(err.to_string().contains("13桁"));
    }

    #[test]
    fn test_client_creation() {
        let client = GbizClient::new("test-token");
        assert!(client.is_ok());
        let client = client.unwrap();
        assert_eq!(client.base_url(), API_BASE);
    }
}
