use clap::Parser;
use poe_ninja_cli::{Cli, Commands, types::*};

#[cfg(test)]
mod cli_tests {
    use super::*;

    // Method 1: Test argument parsing directly without running commands
    #[test]
    fn test_currency_command_parsing() {
        let args = vec!["poe-ninja", "currency", "--league", "Settlers", "--name", "Exalted"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Commands::Currency { league, currency_type, name } => {
                assert_eq!(league, "Settlers");
                assert_eq!(currency_type, "Currency");
                assert_eq!(name, Some("Exalted".to_string()));
            }
            _ => panic!("Expected Currency command"),
        }
    }

    #[test]
    fn test_item_command_parsing() {
        let args = vec![
            "poe-ninja", "item", 
            "--league", "Standard", 
            "--item-type", "UniqueWeapon",
            "--min-chaos", "100",
            "--max-chaos", "1000"
        ];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Commands::Item { league, item_type, name, min_chaos, max_chaos } => {
                assert_eq!(league, "Standard");
                assert_eq!(item_type, "UniqueWeapon");
                assert_eq!(name, None);
                assert_eq!(min_chaos, Some(100.0));
                assert_eq!(max_chaos, Some(1000.0));
            }
            _ => panic!("Expected Item command"),
        }
    }

    #[test]
    fn test_default_values() {
        let args = vec!["poe-ninja", "currency"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Commands::Currency { league, currency_type, name } => {
                assert_eq!(league, "Standard"); // default value
                assert_eq!(currency_type, "Currency"); // default value
                assert_eq!(name, None);
            }
            _ => panic!("Expected Currency command"),
        }
    }

    #[test]
    fn test_invalid_args_fail() {
        let args = vec!["poe-ninja", "currency", "--invalid-flag"];
        let result = Cli::try_parse_from(args);
        assert!(result.is_err());
    }

    #[test]
    fn test_leagues_command_parsing() {
        let args = vec!["poe-ninja", "leagues"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Commands::Leagues => {
                // Successfully parsed leagues command
            }
            _ => panic!("Expected Leagues command"),
        }
    }

    #[test]
    fn test_types_command_parsing() {
        let args = vec!["poe-ninja", "types"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Commands::Types => {
                // Successfully parsed types command
            }
            _ => panic!("Expected Types command"),
        }
    }

    #[test]
    fn test_short_args_work() {
        let args = vec!["poe-ninja", "currency", "-l", "Standard", "-n", "Chaos"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Commands::Currency { league, name, .. } => {
                assert_eq!(league, "Standard");
                assert_eq!(name, Some("Chaos".to_string()));
            }
            _ => panic!("Expected Currency command"),
        }
    }

    #[test]
    fn test_item_command_requires_item_type() {
        let args = vec!["poe-ninja", "item", "--league", "Standard"];
        let result = Cli::try_parse_from(args);
        assert!(result.is_err());
    }
}

// Method 2: Test command handlers separately with mock data
#[cfg(test)]
mod handler_tests {
    use super::*;
    use poe_ninja_cli::{filter_currencies_by_name, filter_items_by_criteria, sort_currencies_by_value, sort_items_by_value};
    
    fn create_mock_currency_data() -> Vec<CurrencyLine> {
        vec![
            CurrencyLine {
                currency_type_name: "Exalted Orb".to_string(),
                chaos_equivalent: Some(180.0),
                pay: Some(CurrencyData {
                    id: 1,
                    league_id: 1,
                    pay_currency_id: 1,
                    get_currency_id: 2,
                    sample_time_utc: "2023-01-01T00:00:00Z".to_string(),
                    count: 100,
                    value: 0.0056,
                    data_point_count: Some(1),
                    includes_secondary: Some(true),
                    listing_count: Some(500),
                }),
                receive: None,
                pay_spark_line: SparkLine { data: None, total_change: Some(5.2) },
                receive_spark_line: SparkLine { data: None, total_change: None },
                low_confidence_pay_spark_line: SparkLine { data: None, total_change: None },
                low_confidence_receive_spark_line: SparkLine { data: None, total_change: None },
                details_id: "exalted-orb".to_string(),
            },
            CurrencyLine {
                currency_type_name: "Chaos Orb".to_string(),
                chaos_equivalent: Some(1.0),
                pay: None,
                receive: Some(CurrencyData {
                    id: 2,
                    league_id: 1,
                    pay_currency_id: 2,
                    get_currency_id: 1,
                    sample_time_utc: "2023-01-01T00:00:00Z".to_string(),
                    count: 1000,
                    value: 1.0,
                    data_point_count: Some(1),
                    includes_secondary: Some(true),
                    listing_count: Some(2000),
                }),
                pay_spark_line: SparkLine { data: None, total_change: None },
                receive_spark_line: SparkLine { data: None, total_change: Some(-1.2) },
                low_confidence_pay_spark_line: SparkLine { data: None, total_change: None },
                low_confidence_receive_spark_line: SparkLine { data: None, total_change: None },
                details_id: "chaos-orb".to_string(),
            },
        ]
    }

    fn create_mock_item_data() -> Vec<ItemLine> {
        vec![
            ItemLine {
                id: 1,
                name: "Belly of the Beast".to_string(),
                icon: "test.png".to_string(),
                base_type: Some("Full Wyrmscale".to_string()),
                chaos_value: 150.0,
                divine_value: Some(0.6),
                exalted_value: None,
                count: 50,
                listing_count: Some(200),
                level_required: Some(46),
                map_tier: None,
                stack_size: None,
                variant: None,
                item_class: Some(1),
                sparkline: SparkLine { data: None, total_change: Some(-2.1) },
                low_confidence_sparkline: SparkLine { data: None, total_change: None },
                implicit_modifiers: vec![],
                explicit_modifiers: vec![],
                flavour_text: "".to_string(),
                corrupted: None,
                gem_level: None,
                gem_quality: None,
                item_type: Some("Body Armour".to_string()),
                details_id: "belly-of-the-beast".to_string(),
                links: None,
                trade_info: None,
            },
            ItemLine {
                id: 2,
                name: "Kaom's Heart".to_string(),
                icon: "test2.png".to_string(),
                base_type: Some("Glorious Plate".to_string()),
                chaos_value: 80.0,
                divine_value: Some(0.3),
                exalted_value: None,
                count: 25,
                listing_count: Some(100),
                level_required: Some(68),
                map_tier: None,
                stack_size: None,
                variant: None,
                item_class: Some(1),
                sparkline: SparkLine { data: None, total_change: Some(3.5) },
                low_confidence_sparkline: SparkLine { data: None, total_change: None },
                implicit_modifiers: vec![],
                explicit_modifiers: vec![],
                flavour_text: "".to_string(),
                corrupted: None,
                gem_level: None,
                gem_quality: None,
                item_type: Some("Body Armour".to_string()),
                details_id: "kaoms-heart".to_string(),
                links: None,
                trade_info: None,
            },
        ]
    }

    #[test]
    fn test_currency_filtering() {
        let currencies = create_mock_currency_data();
        
        // Test filtering by name
        let filtered = filter_currencies_by_name(currencies.clone(), Some("exalted"));
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].currency_type_name, "Exalted Orb");
        assert_eq!(filtered[0].chaos_equivalent, Some(180.0));
        
        // Test no filter
        let all = filter_currencies_by_name(currencies, None);
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn test_currency_sorting() {
        let currencies = create_mock_currency_data();
        let sorted = sort_currencies_by_value(currencies);
        
        // Should be sorted by chaos_equivalent descending
        assert_eq!(sorted[0].currency_type_name, "Exalted Orb");
        assert_eq!(sorted[1].currency_type_name, "Chaos Orb");
    }

    #[test]
    fn test_item_price_filtering() {
        let items = create_mock_item_data();
        
        // Test price range filtering
        let filtered = filter_items_by_criteria(items.clone(), None, Some(100.0), Some(200.0));
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Belly of the Beast");
        assert_eq!(filtered[0].chaos_value, 150.0);
        
        // Test min chaos only
        let min_filtered = filter_items_by_criteria(items.clone(), None, Some(100.0), None);
        assert_eq!(min_filtered.len(), 1);
        assert_eq!(min_filtered[0].name, "Belly of the Beast");
        
        // Test max chaos only
        let max_filtered = filter_items_by_criteria(items, None, None, Some(100.0));
        assert_eq!(max_filtered.len(), 1);
        assert_eq!(max_filtered[0].name, "Kaom's Heart");
    }

    #[test]
    fn test_item_name_filtering() {
        let items = create_mock_item_data();
        
        let filtered = filter_items_by_criteria(items, Some("belly"), None, None);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Belly of the Beast");
    }

    #[test]
    fn test_item_sorting() {
        let items = create_mock_item_data();
        let sorted = sort_items_by_value(items);
        
        // Should be sorted by chaos_value descending
        assert_eq!(sorted[0].name, "Belly of the Beast");
        assert_eq!(sorted[1].name, "Kaom's Heart");
    }

    #[test]
    fn test_combined_filtering_and_sorting() {
        let items = create_mock_item_data();
        
        // Filter by name and price range
        let filtered = filter_items_by_criteria(items, Some("a"), Some(50.0), Some(200.0));
        let sorted = sort_items_by_value(filtered);
        
        // Should find both items (both contain 'a'), sorted by price
        assert_eq!(sorted.len(), 2);
        assert_eq!(sorted[0].name, "Belly of the Beast"); // 150 chaos
        assert_eq!(sorted[1].name, "Kaom's Heart"); // 80 chaos
    }
}

// Method 3: Property-based testing for edge cases
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_currency_command_with_random_league_names(
            league in "[a-zA-Z0-9 ]{1,20}"
        ) {
            let args = vec!["poe-ninja", "currency", "--league", &league];
            let result = Cli::try_parse_from(args);
            
            // Should always parse successfully regardless of league name
            prop_assert!(result.is_ok());
            
            if let Ok(cli) = result {
                if let Commands::Currency { league: parsed_league, .. } = cli.command {
                    prop_assert_eq!(parsed_league, league);
                }
            }
        }

        #[test]
        fn test_chaos_value_parsing(
            min_chaos in 0.0f64..1000000.0,
            max_chaos in 0.0f64..1000000.0
        ) {
            let min_str = min_chaos.to_string();
            let max_str = max_chaos.to_string();
            
            let args = vec![
                "poe-ninja", "item", 
                "--item-type", "UniqueWeapon",
                "--min-chaos", &min_str,
                "--max-chaos", &max_str
            ];
            
            let result = Cli::try_parse_from(args);
            prop_assert!(result.is_ok());
            
            if let Ok(cli) = result {
                if let Commands::Item { min_chaos: parsed_min, max_chaos: parsed_max, .. } = cli.command {
                    prop_assert_eq!(parsed_min, Some(min_chaos));
                    prop_assert_eq!(parsed_max, Some(max_chaos));
                }
            }
        }

        #[test]
        fn test_item_type_parsing(
            item_type in "[a-zA-Z]{1,20}"
        ) {
            let args = vec!["poe-ninja", "item", "--item-type", &item_type];
            let result = Cli::try_parse_from(args);
            
            prop_assert!(result.is_ok());
            
            if let Ok(cli) = result {
                if let Commands::Item { item_type: parsed_type, .. } = cli.command {
                    prop_assert_eq!(parsed_type, item_type);
                }
            }
        }
    }
}

// Method 4: Helper function testing
#[cfg(test)]
mod helper_tests {
    use poe_ninja_cli::{get_available_leagues, get_currency_types, get_item_types};

    #[test]
    fn test_get_available_leagues() {
        let leagues = get_available_leagues();
        assert!(!leagues.is_empty());
        assert!(leagues.contains(&"Standard"));
        assert!(leagues.contains(&"Hardcore"));
        assert!(leagues.contains(&"Settlers"));
    }

    #[test]
    fn test_get_currency_types() {
        let types = get_currency_types();
        assert!(!types.is_empty());
        assert!(types.contains(&"Currency"));
        assert!(types.contains(&"Fragment"));
    }

    #[test]
    fn test_get_item_types() {
        let types = get_item_types();
        assert!(!types.is_empty());
        assert!(types.contains(&"UniqueWeapon"));
        assert!(types.contains(&"UniqueArmour"));
        assert!(types.contains(&"Essence"));
        assert!(types.contains(&"DivinationCard"));
        assert!(types.contains(&"Oil"));
    }

    #[test]
    fn test_item_types_comprehensive() {
        let types = get_item_types();
        
        // Check that we have the most common types
        let expected_types = vec![
            "Oil", "Essence", "UniqueWeapon", "UniqueArmour", "Map",
            "DivinationCard", "SkillGem", "UniqueJewel", "Fossil"
        ];
        
        for expected in expected_types {
            assert!(types.contains(&expected), "Missing item type: {}", expected);
        }
    }
}

// Method 5: Snapshot testing for help output
#[cfg(test)]
mod snapshot_tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_help_output_unchanged() {
        let mut cmd = Cli::command();
        let help_output = cmd.render_help().to_string();
        
        // This would use insta crate for snapshot testing
        // insta::assert_snapshot!(help_output);
        
        // For now, just check it contains expected sections
        assert!(help_output.contains("Usage:"));
        assert!(help_output.contains("Commands:"));
        assert!(help_output.contains("currency"));
        assert!(help_output.contains("item"));
        assert!(help_output.contains("leagues"));
        assert!(help_output.contains("types"));
    }

    #[test]
    fn test_currency_subcommand_help() {
        let args = vec!["poe-ninja", "currency", "--help"];
        let result = Cli::try_parse_from(args);
        
        // This should fail with help, but we can check the error contains help text
        assert!(result.is_err());
        let err = result.unwrap_err();
        let help_text = err.to_string();
        assert!(help_text.contains("currency"));
        assert!(help_text.contains("--league"));
        assert!(help_text.contains("--currency-type"));
    }

    #[test]
    fn test_item_subcommand_help() {
        let args = vec!["poe-ninja", "item", "--help"];
        let result = Cli::try_parse_from(args);
        
        assert!(result.is_err());
        let err = result.unwrap_err();
        let help_text = err.to_string();
        assert!(help_text.contains("item"));
        assert!(help_text.contains("--league"));
        assert!(help_text.contains("--item-type"));
        assert!(help_text.contains("--min-chaos"));
        assert!(help_text.contains("--max-chaos"));
    }
}

// Method 6: Edge case testing
#[cfg(test)]
mod edge_case_tests {
    use super::*;
    use poe_ninja_cli::{filter_currencies_by_name, filter_items_by_criteria};

    #[test]
    fn test_empty_currency_list() {
        let currencies = vec![];
        let filtered = filter_currencies_by_name(currencies, Some("exalted"));
        assert!(filtered.is_empty());
    }

    #[test]
    fn test_empty_item_list() {
        let items = vec![];
        let filtered = filter_items_by_criteria(items, Some("belly"), Some(100.0), Some(200.0));
        assert!(filtered.is_empty());
    }

    #[test]
    fn test_case_insensitive_filtering() {
        let currencies = vec![
            CurrencyLine {
                currency_type_name: "Exalted Orb".to_string(),
                chaos_equivalent: Some(180.0),
                pay: None,
                receive: None,
                pay_spark_line: SparkLine { data: None, total_change: None },
                receive_spark_line: SparkLine { data: None, total_change: None },
                low_confidence_pay_spark_line: SparkLine { data: None, total_change: None },
                low_confidence_receive_spark_line: SparkLine { data: None, total_change: None },
                details_id: "exalted-orb".to_string(),
            },
        ];

        // Test various cases
        let filtered_lower = filter_currencies_by_name(currencies.clone(), Some("exalted"));
        let filtered_upper = filter_currencies_by_name(currencies.clone(), Some("EXALTED"));
        let filtered_mixed = filter_currencies_by_name(currencies, Some("ExAlTeD"));
        
        assert_eq!(filtered_lower.len(), 1);
        assert_eq!(filtered_upper.len(), 1);
        assert_eq!(filtered_mixed.len(), 1);
    }

    #[test]
    fn test_extreme_chaos_values() {
        let items = vec![
            ItemLine {
                id: 1,
                name: "Expensive Item".to_string(),
                chaos_value: 999999.0,
                icon: "".to_string(),
                map_tier: None,
                level_required: None,
                base_type: None,
                stack_size: None,
                variant: None,
                item_class: None,
                sparkline: SparkLine { data: None, total_change: None },
                low_confidence_sparkline: SparkLine { data: None, total_change: None },
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
            ItemLine {
                id: 2,
                name: "Free Item".to_string(),
                chaos_value: 0.0,
                icon: "".to_string(),
                map_tier: None,
                level_required: None,
                base_type: None,
                stack_size: None,
                variant: None,
                item_class: None,
                sparkline: SparkLine { data: None, total_change: None },
                low_confidence_sparkline: SparkLine { data: None, total_change: None },
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
                details_id: "free".to_string(),
                listing_count: None,
                links: None,
                trade_info: None,
            },
        ];

        // Test extreme values
        let expensive_only = filter_items_by_criteria(items.clone(), None, Some(999999.0), None);
        assert_eq!(expensive_only.len(), 1);
        assert_eq!(expensive_only[0].name, "Expensive Item");
        
        let free_only = filter_items_by_criteria(items, None, None, Some(0.0));
        assert_eq!(free_only.len(), 1);
        assert_eq!(free_only[0].name, "Free Item");
    }
} 