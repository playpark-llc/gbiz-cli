mod cli;
mod client;
mod commands;
mod output;

use anyhow::{Result, anyhow};
use clap::Parser;

use cli::{Cli, Command};
use client::{GbizClient, SearchParams};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    if let Err(e) = run(cli).await {
        eprintln!("Error: {e:#}");
        std::process::exit(1);
    }
}

async fn run(cli: Cli) -> Result<()> {
    let Cli {
        token,
        output: output_format,
        verbose,
        command,
    } = cli;

    let token = token.ok_or_else(|| {
        anyhow!("API token is required. Set GBIZ_API_TOKEN or use --token")
    })?;

    let client = GbizClient::new(&token)?;

    let (result, command_name) = match &command {
        Command::Search {
            query,
            prefecture,
            capital_from,
            capital_to,
            employee_from,
            employee_to,
            limit,
            page,
        } => {
            let params = SearchParams {
                name: query.clone(),
                prefecture: prefecture.clone(),
                capital_from: *capital_from,
                capital_to: *capital_to,
                employee_from: *employee_from,
                employee_to: *employee_to,
                limit: *limit,
                page: *page,
            };
            (
                commands::search::search(&client, &params).await,
                "search",
            )
        }
        Command::Get { corporate_number } => (
            commands::detail::get_detail(&client, corporate_number).await,
            "get",
        ),
        Command::Finance { corporate_number } => (
            commands::detail::get_category(&client, corporate_number, "finance").await,
            "finance",
        ),
        Command::Patent { corporate_number } => (
            commands::detail::get_category(&client, corporate_number, "patent").await,
            "patent",
        ),
        Command::Procurement { corporate_number } => (
            commands::detail::get_category(&client, corporate_number, "procurement").await,
            "procurement",
        ),
        Command::Subsidy { corporate_number } => (
            commands::detail::get_category(&client, corporate_number, "subsidy").await,
            "subsidy",
        ),
        Command::Certification { corporate_number } => (
            commands::detail::get_category(&client, corporate_number, "certification").await,
            "certification",
        ),
        Command::Commendation { corporate_number } => (
            commands::detail::get_category(&client, corporate_number, "commendation").await,
            "commendation",
        ),
        Command::Workplace { corporate_number } => (
            commands::detail::get_category(&client, corporate_number, "workplace").await,
            "workplace",
        ),
        Command::Corporation { corporate_number } => (
            commands::detail::get_category(&client, corporate_number, "corporation").await,
            "corporation",
        ),
    };

    let value = result?;

    if verbose {
        eprintln!("Raw response:\n{}", serde_json::to_string_pretty(&value)?);
    }

    let rendered = output::render(&value, output_format, command_name)?;
    println!("{rendered}");

    Ok(())
}
