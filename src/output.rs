use anyhow::Result;
use serde_json::Value;
use tabled::settings::Style;

use crate::cli::OutputFormat;

pub fn render(value: &Value, format: OutputFormat, command: &str) -> Result<String> {
    match format {
        OutputFormat::Json => Ok(serde_json::to_string_pretty(value)?),
        OutputFormat::Table => Ok(render_table(value, command)),
        OutputFormat::Compact => Ok(render_compact(value, command)),
    }
}

fn render_table(value: &Value, command: &str) -> String {
    match command {
        "search" => render_search_table(value),
        "get" => render_get_table(value),
        "finance" => render_finance_table(value),
        "subsidy" => render_subsidy_table(value),
        "workplace" => render_workplace_table(value),
        _ => serde_json::to_string_pretty(value).unwrap_or_default(),
    }
}

fn render_compact(value: &Value, command: &str) -> String {
    match command {
        "search" => render_search_compact(value),
        "get" => render_get_compact(value),
        "finance" => render_finance_compact(value),
        "subsidy" => render_subsidy_compact(value),
        "workplace" => render_workplace_compact(value),
        _ => serde_json::to_string_pretty(value).unwrap_or_default(),
    }
}

// ── Search ──

fn render_search_table(value: &Value) -> String {
    let items = extract_array(value, "hojin-infos");
    if items.is_empty() {
        return "検索結果なし".to_string();
    }

    let rows: Vec<Vec<String>> = items
        .iter()
        .map(|item| {
            vec![
                str_field(item, "corporate_number"),
                truncate_str(&str_field(item, "name"), 30),
                truncate_str(&str_field(item, "location"), 20),
                str_field(item, "capital_stock"),
                str_field(item, "employee_number"),
            ]
        })
        .collect();

    let header = vec!["法人番号", "法人名", "所在地", "資本金", "従業員数"];
    build_table_from_rows(&header, &rows)
}

fn render_search_compact(value: &Value) -> String {
    let items = extract_array(value, "hojin-infos");
    if items.is_empty() {
        return "検索結果なし".to_string();
    }

    items
        .iter()
        .map(|item| {
            format!(
                "[{}] {} ({})",
                str_field(item, "corporate_number"),
                str_field(item, "name"),
                str_field(item, "location"),
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// ── Get (detail) ──

fn render_get_table(value: &Value) -> String {
    let items = extract_array(value, "hojin-infos");
    let Some(item) = items.first() else {
        return "データなし".to_string();
    };

    let fields = [
        ("法人番号", str_field(item, "corporate_number")),
        ("法人名", str_field(item, "name")),
        ("所在地", str_field(item, "location")),
        ("ステータス", str_field(item, "status")),
        ("資本金", str_field(item, "capital_stock")),
        ("従業員数", str_field(item, "employee_number")),
        ("設立年月日", str_field(item, "date_of_establishment")),
        ("代表者", str_field(item, "representative_name")),
        ("事業概要", str_field(item, "business_summary")),
        ("URL", str_field(item, "company_url")),
    ];

    let rows: Vec<Vec<String>> = fields
        .iter()
        .map(|(k, v)| vec![(*k).to_string(), v.clone()])
        .collect();
    let header = vec!["項目", "値"];
    build_table_from_rows(&header, &rows)
}

fn render_get_compact(value: &Value) -> String {
    let items = extract_array(value, "hojin-infos");
    let Some(item) = items.first() else {
        return "データなし".to_string();
    };

    let fields = [
        ("法人番号", str_field(item, "corporate_number")),
        ("法人名", str_field(item, "name")),
        ("所在地", str_field(item, "location")),
        ("資本金", str_field(item, "capital_stock")),
        ("従業員数", str_field(item, "employee_number")),
        ("代表者", str_field(item, "representative_name")),
    ];

    fields
        .iter()
        .map(|(k, v)| format!("{k}: {v}"))
        .collect::<Vec<_>>()
        .join("\n")
}

// ── Finance ──

fn render_finance_table(value: &Value) -> String {
    let items = extract_array(value, "hojin-infos");
    let Some(item) = items.first() else {
        return "データなし".to_string();
    };

    let finances = extract_array(item, "finance");
    if finances.is_empty() {
        return "財務情報なし".to_string();
    }

    let rows: Vec<Vec<String>> = finances
        .iter()
        .map(|f| {
            vec![
                str_field(f, "date_of_establishment"),
                str_field(f, "accounting_standards"),
                str_field(f, "net_sales"),
                str_field(f, "net_income"),
                str_field(f, "total_assets"),
            ]
        })
        .collect();

    let header = vec!["決算期", "会計基準", "売上高", "純利益", "総資産"];
    build_table_from_rows(&header, &rows)
}

fn render_finance_compact(value: &Value) -> String {
    let items = extract_array(value, "hojin-infos");
    let Some(item) = items.first() else {
        return "データなし".to_string();
    };

    let finances = extract_array(item, "finance");
    if finances.is_empty() {
        return "財務情報なし".to_string();
    }

    finances
        .iter()
        .map(|f| {
            format!(
                "{}: 売上={} 純利益={} 総資産={}",
                str_field(f, "date_of_establishment"),
                str_field(f, "net_sales"),
                str_field(f, "net_income"),
                str_field(f, "total_assets"),
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// ── Subsidy ──

fn render_subsidy_table(value: &Value) -> String {
    let items = extract_array(value, "hojin-infos");
    let Some(item) = items.first() else {
        return "データなし".to_string();
    };

    let subsidies = extract_array(item, "subsidy");
    if subsidies.is_empty() {
        return "補助金情報なし".to_string();
    }

    let rows: Vec<Vec<String>> = subsidies
        .iter()
        .map(|s| {
            vec![
                truncate_str(&str_field(s, "title"), 40),
                str_field(s, "date_of_approval"),
                str_field(s, "subsidy_amount"),
            ]
        })
        .collect();

    let header = vec!["補助金名", "認定日", "金額"];
    build_table_from_rows(&header, &rows)
}

fn render_subsidy_compact(value: &Value) -> String {
    let items = extract_array(value, "hojin-infos");
    let Some(item) = items.first() else {
        return "データなし".to_string();
    };

    let subsidies = extract_array(item, "subsidy");
    if subsidies.is_empty() {
        return "補助金情報なし".to_string();
    }

    subsidies
        .iter()
        .map(|s| {
            format!(
                "[{}] {} ({})",
                str_field(s, "date_of_approval"),
                str_field(s, "title"),
                str_field(s, "subsidy_amount"),
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// ── Workplace ──

fn render_workplace_table(value: &Value) -> String {
    let items = extract_array(value, "hojin-infos");
    let Some(item) = items.first() else {
        return "データなし".to_string();
    };

    let workplaces = extract_array(item, "workplace_info");
    if workplaces.is_empty() {
        return "職場情報なし".to_string();
    }

    let wp = &workplaces[0];
    let base = wp.get("base_infos").and_then(Value::as_object);

    let fields = if let Some(base) = base {
        let base_val = Value::Object(base.clone());
        vec![
            ("平均年齢", str_field(&base_val, "average_age")),
            ("平均勤続年数", str_field(&base_val, "average_continuous_service_years")),
            ("月平均残業時間", str_field(&base_val, "average_days_for_overtime")),
            ("女性比率", str_field(&base_val, "female_workers_proportion")),
        ]
    } else {
        vec![
            ("平均年齢", str_field(wp, "average_age")),
            ("平均勤続年数", str_field(wp, "average_continuous_service_years")),
            ("月平均残業時間", str_field(wp, "average_days_for_overtime")),
            ("女性比率", str_field(wp, "female_workers_proportion")),
        ]
    };

    let rows: Vec<Vec<String>> = fields
        .iter()
        .map(|(k, v)| vec![(*k).to_string(), v.clone()])
        .collect();
    let header = vec!["項目", "値"];
    build_table_from_rows(&header, &rows)
}

fn render_workplace_compact(value: &Value) -> String {
    let items = extract_array(value, "hojin-infos");
    let Some(item) = items.first() else {
        return "データなし".to_string();
    };

    let workplaces = extract_array(item, "workplace_info");
    if workplaces.is_empty() {
        return "職場情報なし".to_string();
    }

    let wp = &workplaces[0];
    format!(
        "平均年齢: {} / 勤続年数: {} / 残業: {} / 女性比率: {}",
        str_field(wp, "average_age"),
        str_field(wp, "average_continuous_service_years"),
        str_field(wp, "average_days_for_overtime"),
        str_field(wp, "female_workers_proportion"),
    )
}

// ── Helpers ──

fn extract_array<'a>(value: &'a Value, key: &str) -> Vec<&'a Value> {
    value
        .get(key)
        .and_then(Value::as_array)
        .map(|arr| arr.iter().collect())
        .unwrap_or_default()
}

fn str_field(value: &Value, key: &str) -> String {
    match value.get(key) {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Number(n)) => n.to_string(),
        Some(Value::Null) | None => "-".to_string(),
        Some(v) => v.to_string(),
    }
}

fn truncate_str(s: &str, max_chars: usize) -> String {
    if s.chars().count() <= max_chars {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max_chars - 3).collect();
        format!("{truncated}...")
    }
}

fn build_table_from_rows(header: &[&str], rows: &[Vec<String>]) -> String {
    let mut builder = tabled::builder::Builder::new();
    builder.push_record(header.iter().map(|h| (*h).to_string()));
    for row in rows {
        builder.push_record(row.clone());
    }
    let mut table = builder.build();
    table.with(Style::rounded());
    table.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_truncate_str_short() {
        assert_eq!(truncate_str("abc", 10), "abc");
    }

    #[test]
    fn test_truncate_str_long() {
        let result = truncate_str("abcdefghijklmnop", 10);
        assert_eq!(result, "abcdefg...");
    }

    #[test]
    fn test_str_field_string() {
        let v = json!({"name": "テスト"});
        assert_eq!(str_field(&v, "name"), "テスト");
    }

    #[test]
    fn test_str_field_number() {
        let v = json!({"capital": 1000000});
        assert_eq!(str_field(&v, "capital"), "1000000");
    }

    #[test]
    fn test_str_field_missing() {
        let v = json!({});
        assert_eq!(str_field(&v, "name"), "-");
    }

    #[test]
    fn test_str_field_null() {
        let v = json!({"name": null});
        assert_eq!(str_field(&v, "name"), "-");
    }

    #[test]
    fn test_render_search_table_empty() {
        let v = json!({"hojin-infos": []});
        let result = render_search_table(&v);
        assert_eq!(result, "検索結果なし");
    }

    #[test]
    fn test_render_search_compact() {
        let v = json!({
            "hojin-infos": [{
                "corporate_number": "1234567890123",
                "name": "テスト株式会社",
                "location": "東京都"
            }]
        });
        let result = render_search_compact(&v);
        assert!(result.contains("1234567890123"));
        assert!(result.contains("テスト株式会社"));
    }

    #[test]
    fn test_render_json() {
        let v = json!({"key": "value"});
        let result = render(&v, OutputFormat::Json, "search").unwrap();
        assert!(result.contains("\"key\""));
        assert!(result.contains("\"value\""));
    }
}
