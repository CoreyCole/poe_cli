use anyhow::{Context, Result};
use colored::*;
use tabled::{settings::Style, Table, Tabled};
use clap::Parser;

use poe_ninja_cli::{Cli, Commands, PoeNinjaClient, filter_currencies_by_name, filter_items_by_criteria, sort_currencies_by_value, sort_items_by_value, get_available_leagues, get_item_types, get_currency_types};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let client = PoeNinjaClient::new();

    match &cli.command {
        Commands::Currency {
            league,
            currency_type,
            name,
        } => {
            handle_currency(&client, &league, &currency_type, name.as_deref()).await?;
        }
        Commands::Item {
            league,
            item_type,
            name,
            min_chaos,
            max_chaos,
        } => {
            handle_item(&client, &league, &item_type, name.as_deref(), *min_chaos, *max_chaos)
                .await?;
        }
        Commands::Leagues => {
            handle_leagues().await?;
        }
        Commands::Types => {
            handle_types().await?;
        }
    }

    Ok(())
}

async fn handle_currency(
    client: &PoeNinjaClient,
    league: &str,
    currency_type: &str,
    name_filter: Option<&str>,
) -> Result<()> {
    println!(
        "{} {}",
        "Fetching currency data for".bright_blue(),
        format!("{} - {}", league, currency_type).bright_yellow()
    );

    let response = client
        .get_currency_overview(league, currency_type)
        .await
        .context("Failed to fetch currency data")?;

    let filtered_currencies = filter_currencies_by_name(response.lines, name_filter);
    let sorted_currencies = sort_currencies_by_value(filtered_currencies);

    let currencies: Vec<CurrencyDisplay> = sorted_currencies
        .into_iter()
        .map(|currency| CurrencyDisplay {
            name: currency.currency_type_name,
            chaos_equivalent: currency.chaos_equivalent.unwrap_or(0.0),
            pay_value: currency.pay.as_ref().map(|p| p.value).unwrap_or(0.0),
            receive_value: currency.receive.as_ref().map(|r| r.value).unwrap_or(0.0),
            pay_count: currency.pay.as_ref().map(|p| p.count).unwrap_or(0),
            receive_count: currency.receive.as_ref().map(|r| r.count).unwrap_or(0),
        })
        .collect();

    if currencies.is_empty() {
        println!("{}", "No currencies found with the given filters.".red());
        return Ok(());
    }

    let mut table = Table::new(currencies);
    table.with(Style::modern());
    println!("{}", table);

    Ok(())
}

async fn handle_item(
    client: &PoeNinjaClient,
    league: &str,
    item_type: &str,
    name_filter: Option<&str>,
    min_chaos: Option<f64>,
    max_chaos: Option<f64>,
) -> Result<()> {
    println!(
        "{} {}",
        "Fetching item data for".bright_blue(),
        format!("{} - {}", league, item_type).bright_yellow()
    );

    let response = client
        .get_item_overview(league, item_type)
        .await
        .context("Failed to fetch item data")?;

    let filtered_items = filter_items_by_criteria(response.lines, name_filter, min_chaos, max_chaos);
    let sorted_items = sort_items_by_value(filtered_items);

    let items: Vec<ItemDisplay> = sorted_items
        .into_iter()
        .map(|item| ItemDisplay {
            name: item.name,
            base_type: item.base_type.unwrap_or_else(|| "N/A".to_string()),
            chaos_value: item.chaos_value,
            divine_value: item.divine_value.unwrap_or(0.0),
            count: item.count,
            listing_count: item.listing_count.unwrap_or(0),
            level_required: item.level_required.unwrap_or(0),
        })
        .collect();

    if items.is_empty() {
        println!("{}", "No items found with the given filters.".red());
        return Ok(());
    }

    let mut table = Table::new(items);
    table.with(Style::modern());
    println!("{}", table);

    Ok(())
}

async fn handle_leagues() -> Result<()> {
    println!("{}", "Available League Names:".bright_green());
    println!();
    
    let leagues = get_available_leagues();
    for league in leagues {
        println!("  • {}", league.bright_yellow());
    }
    
    println!();
    println!("{}", "Note: League names are case-sensitive. Use exact names as shown above.".dimmed());
    Ok(())
}

async fn handle_types() -> Result<()> {
    println!("{}", "Available Data Types:".bright_green());
    println!();
    
    println!("{}", "Currency Types:".bright_blue());
    let currency_types = get_currency_types();
    for currency_type in currency_types {
        println!("  • {}", currency_type.bright_yellow());
    }
    
    println!();
    println!("{}", "Item Types:".bright_blue());
    let item_types = get_item_types();
    for item_type in item_types {
        println!("  • {}", item_type.bright_yellow());
    }
    
    println!();
    println!("{}", "Note: Type names are case-sensitive. Use exact names as shown above.".dimmed());
    Ok(())
}

#[derive(Tabled)]
struct CurrencyDisplay {
    #[tabled(rename = "Currency")]
    name: String,
    #[tabled(rename = "Chaos Value")]
    chaos_equivalent: f64,
    #[tabled(rename = "Pay Value")]
    pay_value: f64,
    #[tabled(rename = "Receive Value")]
    receive_value: f64,
    #[tabled(rename = "Pay Count")]
    pay_count: i32,
    #[tabled(rename = "Receive Count")]
    receive_count: i32,
}

#[derive(Tabled)]
struct ItemDisplay {
    #[tabled(rename = "Item Name")]
    name: String,
    #[tabled(rename = "Base Type")]
    base_type: String,
    #[tabled(rename = "Chaos Value")]
    chaos_value: f64,
    #[tabled(rename = "Divine Value")]
    divine_value: f64,
    #[tabled(rename = "Count")]
    count: i32,
    #[tabled(rename = "Listings")]
    listing_count: i32,
    #[tabled(rename = "Level")]
    level_required: i32,
}