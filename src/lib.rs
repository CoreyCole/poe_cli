use clap::{Parser, Subcommand};

pub mod api;
pub mod types;

pub use api::PoeNinjaClient;
pub use types::*;

// Export CLI types for testing
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(name = "poe-ninja")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Get currency prices and exchange rates
    Currency {
        /// League name (e.g., "Settlers", "Standard", "Hardcore")
        #[arg(short, long, default_value = "Standard")]
        league: String,
        /// Currency type (Currency, Fragment)
        #[arg(short, long, default_value = "Currency")]
        currency_type: String,
        /// Filter by currency name
        #[arg(short, long)]
        name: Option<String>,
    },
    /// Get item prices
    Item {
        /// League name
        #[arg(short, long, default_value = "Standard")]
        league: String,
        /// Item type (Oil, Essence, UniqueWeapon, UniqueArmour, etc.)
        #[arg(short, long)]
        item_type: String,
        /// Filter by item name
        #[arg(short, long)]
        name: Option<String>,
        /// Minimum chaos value filter
        #[arg(long)]
        min_chaos: Option<f64>,
        /// Maximum chaos value filter  
        #[arg(long)]
        max_chaos: Option<f64>,
    },
    /// List available leagues
    Leagues,
    /// List available item types
    Types,
}

// Extract business logic into testable functions
pub fn filter_currencies_by_name(
    currencies: Vec<types::CurrencyLine>,
    name_filter: Option<&str>,
) -> Vec<types::CurrencyLine> {
    currencies
        .into_iter()
        .filter(|currency| {
            if let Some(filter) = name_filter {
                currency
                    .currency_type_name
                    .to_lowercase()
                    .contains(&filter.to_lowercase())
            } else {
                true
            }
        })
        .collect()
}

pub fn filter_items_by_criteria(
    items: Vec<types::ItemLine>,
    name_filter: Option<&str>,
    min_chaos: Option<f64>,
    max_chaos: Option<f64>,
) -> Vec<types::ItemLine> {
    items
        .into_iter()
        .filter(|item| {
            let name_matches = if let Some(filter) = name_filter {
                item.name.to_lowercase().contains(&filter.to_lowercase())
            } else {
                true
            };

            let chaos_matches = match (min_chaos, max_chaos) {
                (Some(min), Some(max)) => item.chaos_value >= min && item.chaos_value <= max,
                (Some(min), None) => item.chaos_value >= min,
                (None, Some(max)) => item.chaos_value <= max,
                (None, None) => true,
            };

            name_matches && chaos_matches
        })
        .collect()
}

pub fn sort_currencies_by_value(mut currencies: Vec<types::CurrencyLine>) -> Vec<types::CurrencyLine> {
    currencies.sort_by(|a, b| b.chaos_equivalent.partial_cmp(&a.chaos_equivalent).unwrap());
    currencies
}

pub fn sort_items_by_value(mut items: Vec<types::ItemLine>) -> Vec<types::ItemLine> {
    items.sort_by(|a, b| b.chaos_value.partial_cmp(&a.chaos_value).unwrap());
    items
}

// Helper for getting league list
pub fn get_available_leagues() -> Vec<&'static str> {
    vec![
        "Standard",
        "Hardcore", 
        "Settlers",
        "Hardcore Settlers",
        "Solo Self-Found",
        "Hardcore Solo Self-Found",
    ]
}

// Helper for getting item types
pub fn get_currency_types() -> Vec<&'static str> {
    vec!["Currency", "Fragment"]
}

pub fn get_item_types() -> Vec<&'static str> {
    vec![
        "Oil", "Incubator", "Scarab", "Fossil", "Resonator", "Essence",
        "DivinationCard", "SkillGem", "BaseType", "HelmetEnchant",
        "UniqueMap", "Map", "UniqueJewel", "UniqueFlask", "UniqueWeapon",
        "UniqueArmour", "UniqueAccessory", "Beast", "Vials", "DeliriumOrb",
        "Omen", "UniqueRelic", "ClusterJewel", "BlightedMap", "BlightRavagedMap",
        "Invitation", "Memory", "Coffin", "AllflameEmber"
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_currencies_by_name() {
        let currencies = vec![
            types::CurrencyLine {
                currency_type_name: "Exalted Orb".to_string(),
                chaos_equivalent: Some(180.0),
                pay: None,
                receive: None,
                pay_spark_line: types::SparkLine { data: None, total_change: None },
                receive_spark_line: types::SparkLine { data: None, total_change: None },
                low_confidence_pay_spark_line: types::SparkLine { data: None, total_change: None },
                low_confidence_receive_spark_line: types::SparkLine { data: None, total_change: None },
                details_id: "exalted-orb".to_string(),
            },
            types::CurrencyLine {
                currency_type_name: "Chaos Orb".to_string(),
                chaos_equivalent: Some(1.0),
                pay: None,
                receive: None,
                pay_spark_line: types::SparkLine { data: None, total_change: None },
                receive_spark_line: types::SparkLine { data: None, total_change: None },
                low_confidence_pay_spark_line: types::SparkLine { data: None, total_change: None },
                low_confidence_receive_spark_line: types::SparkLine { data: None, total_change: None },
                details_id: "chaos-orb".to_string(),
            },
        ];

        let filtered = filter_currencies_by_name(currencies, Some("exalted"));
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].currency_type_name, "Exalted Orb");
    }

    #[test]
    fn test_filter_items_by_price_range() {
        let items = vec![
            types::ItemLine {
                id: 1,
                name: "Expensive Item".to_string(),
                chaos_value: 500.0,
                icon: "".to_string(),
                map_tier: None,
                level_required: None,
                base_type: None,
                stack_size: None,
                variant: None,
                item_class: None,
                sparkline: types::SparkLine { data: None, total_change: None },
                low_confidence_sparkline: types::SparkLine { data: None, total_change: None },
                implicit_modifiers: vec![],
                explicit_modifiers: vec![],
                flavour_text: "".to_string(),
                corrupted: None,
                gem_level: None,
                gem_quality: None,
                item_type: None,
                exalted_value: None,
                divine_value: None,
                count: 1,
                details_id: "expensive".to_string(),
                listing_count: None,
                links: None,
                trade_info: None,
            },
            types::ItemLine {
                id: 2,
                name: "Cheap Item".to_string(),
                chaos_value: 5.0,
                icon: "".to_string(),
                map_tier: None,
                level_required: None,
                base_type: None,
                stack_size: None,
                variant: None,
                item_class: None,
                sparkline: types::SparkLine { data: None, total_change: None },
                low_confidence_sparkline: types::SparkLine { data: None, total_change: None },
                implicit_modifiers: vec![],
                explicit_modifiers: vec![],
                flavour_text: "".to_string(),
                corrupted: None,
                gem_level: None,
                gem_quality: None,
                item_type: None,
                exalted_value: None,
                divine_value: None,
                count: 1,
                details_id: "cheap".to_string(),
                listing_count: None,
                links: None,
                trade_info: None,
            },
        ];

        let filtered = filter_items_by_criteria(items, None, Some(10.0), Some(1000.0));
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Expensive Item");
    }

    #[test]
    fn test_available_leagues_not_empty() {
        let leagues = get_available_leagues();
        assert!(!leagues.is_empty());
        assert!(leagues.contains(&"Standard"));
        assert!(leagues.contains(&"Hardcore"));
    }

    #[test]
    fn test_item_types_comprehensive() {
        let types = get_item_types();
        assert!(types.contains(&"UniqueWeapon"));
        assert!(types.contains(&"UniqueArmour"));
        assert!(types.contains(&"Essence"));
        assert!(types.contains(&"DivinationCard"));
    }
} 