use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "gbiz", about = "gBizINFO REST API CLI")]
pub struct Cli {
    /// API トークン
    #[arg(long, env = "GBIZ_API_TOKEN", global = true, hide_env_values = true)]
    pub token: Option<String>,

    /// 出力フォーマット
    #[arg(long, short = 'o', default_value = "json", global = true)]
    pub output: OutputFormat,

    /// デバッグ出力
    #[arg(long, short = 'v', global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Json,
    Table,
    Compact,
}

#[derive(Subcommand)]
pub enum Command {
    /// 法人名で検索
    Search {
        /// 検索クエリ (法人名)
        query: String,

        /// 都道府県コード
        #[arg(long)]
        prefecture: Option<String>,

        /// 資本金下限
        #[arg(long)]
        capital_from: Option<u64>,

        /// 資本金上限
        #[arg(long)]
        capital_to: Option<u64>,

        /// 従業員数下限
        #[arg(long)]
        employee_from: Option<u32>,

        /// 従業員数上限
        #[arg(long)]
        employee_to: Option<u32>,

        /// 取得件数
        #[arg(long, default_value = "10")]
        limit: u32,

        /// ページ番号
        #[arg(long, default_value = "1")]
        page: u32,
    },
    /// 法人詳細 (全カテゴリ一括)
    Get {
        /// 法人番号 (13桁)
        corporate_number: String,
    },
    /// 財務情報
    Finance {
        /// 法人番号 (13桁)
        corporate_number: String,
    },
    /// 特許・商標
    Patent {
        /// 法人番号 (13桁)
        corporate_number: String,
    },
    /// 官公需調達実績
    Procurement {
        /// 法人番号 (13桁)
        corporate_number: String,
    },
    /// 補助金情報
    Subsidy {
        /// 法人番号 (13桁)
        corporate_number: String,
    },
    /// 届出・認定
    Certification {
        /// 法人番号 (13桁)
        corporate_number: String,
    },
    /// 表彰情報
    Commendation {
        /// 法人番号 (13桁)
        corporate_number: String,
    },
    /// 職場情報
    Workplace {
        /// 法人番号 (13桁)
        corporate_number: String,
    },
    /// 事業所情報
    Corporation {
        /// 法人番号 (13桁)
        corporate_number: String,
    },
}
