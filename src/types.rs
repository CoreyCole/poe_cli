use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrencyOverviewResponse {
    pub lines: Vec<CurrencyLine>,
    #[serde(rename = "currencyDetails")]
    pub currency_details: Vec<CurrencyDetail>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CurrencyLine {
    #[serde(rename = "currencyTypeName")]
    pub currency_type_name: String,
    pub pay: Option<CurrencyData>,
    pub receive: Option<CurrencyData>,
    #[serde(rename = "paySparkLine")]
    pub pay_spark_line: SparkLine,
    #[serde(rename = "receiveSparkLine")]
    pub receive_spark_line: SparkLine,
    #[serde(rename = "chaosEquivalent")]
    pub chaos_equivalent: Option<f64>,
    #[serde(rename = "lowConfidencePaySparkLine")]
    pub low_confidence_pay_spark_line: SparkLine,
    #[serde(rename = "lowConfidenceReceiveSparkLine")]
    pub low_confidence_receive_spark_line: SparkLine,
    #[serde(rename = "detailsId")]
    pub details_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CurrencyData {
    pub id: i32,
    #[serde(rename = "league_id")]
    pub league_id: i32,
    #[serde(rename = "pay_currency_id")]
    pub pay_currency_id: i32,
    #[serde(rename = "get_currency_id")]
    pub get_currency_id: i32,
    #[serde(rename = "sample_time_utc")]
    pub sample_time_utc: String,
    pub count: i32,
    pub value: f64,
    #[serde(rename = "data_point_count")]
    pub data_point_count: Option<i32>,
    #[serde(rename = "includes_secondary")]
    pub includes_secondary: Option<bool>,
    #[serde(rename = "listing_count")]
    pub listing_count: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrencyDetail {
    pub id: i32,
    pub icon: Option<String>,
    pub name: String,
    #[serde(rename = "tradeId")]
    pub trade_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ItemOverviewResponse {
    pub lines: Vec<ItemLine>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ItemLine {
    pub id: i32,
    pub name: String,
    pub icon: String,
    #[serde(rename = "mapTier")]
    pub map_tier: Option<i32>,
    #[serde(rename = "levelRequired")]
    pub level_required: Option<i32>,
    #[serde(rename = "baseType")]
    pub base_type: Option<String>,
    #[serde(rename = "stackSize")]
    pub stack_size: Option<i32>,
    pub variant: Option<String>,
    #[serde(rename = "itemClass")]
    pub item_class: Option<i32>,
    pub sparkline: SparkLine,
    #[serde(rename = "lowConfidenceSparkline")]
    pub low_confidence_sparkline: SparkLine,
    #[serde(rename = "implicitModifiers")]
    pub implicit_modifiers: Vec<Modifier>,
    #[serde(rename = "explicitModifiers")]
    pub explicit_modifiers: Vec<Modifier>,
    #[serde(rename = "flavourText")]
    pub flavour_text: String,
    pub corrupted: Option<bool>,
    #[serde(rename = "gemLevel")]
    pub gem_level: Option<i32>,
    #[serde(rename = "gemQuality")]
    pub gem_quality: Option<i32>,
    #[serde(rename = "itemType")]
    pub item_type: Option<String>,
    #[serde(rename = "chaosValue")]
    pub chaos_value: f64,
    #[serde(rename = "exaltedValue")]
    pub exalted_value: Option<f64>,
    #[serde(rename = "divineValue")]
    pub divine_value: Option<f64>,
    pub count: i32,
    #[serde(rename = "detailsId")]
    pub details_id: String,
    #[serde(rename = "listingCount")]
    pub listing_count: Option<i32>,
    pub links: Option<i32>,
    #[serde(rename = "tradeInfo")]
    pub trade_info: Option<Vec<TradeInfo>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SparkLine {
    pub data: Option<Vec<Option<f64>>>,
    #[serde(rename = "totalChange")]
    pub total_change: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Modifier {
    pub text: String,
    pub optional: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TradeInfo {
    #[serde(rename = "mod")]
    pub mod_name: String,
    pub min: i32,
    pub max: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    // Helper structures for display formatting (test-only)
    #[derive(Debug)]
    pub struct PriceInfo {
        pub chaos_value: f64,
        pub divine_value: Option<f64>,
        pub exalted_value: Option<f64>,
        pub change_24h: Option<f64>,
    }

    impl PriceInfo {
        pub fn from_item(item: &ItemLine) -> Self {
            Self {
                chaos_value: item.chaos_value,
                divine_value: item.divine_value,
                exalted_value: item.exalted_value,
                change_24h: item.sparkline.total_change,
            }
        }

        pub fn from_currency(currency: &CurrencyLine) -> Self {
            Self {
                chaos_value: currency.chaos_equivalent.unwrap_or(0.0),
                divine_value: None,
                exalted_value: None,
                change_24h: currency.pay_spark_line.total_change,
            }
        }
    }

    #[test]
    fn test_price_info_from_item() {
        let item = ItemLine {
            id: 1,
            name: "Test Item".to_string(),
            icon: "test.png".to_string(),
            map_tier: None,
            level_required: Some(60),
            base_type: Some("Weapon".to_string()),
            stack_size: None,
            variant: None,
            item_class: Some(1),
            sparkline: SparkLine {
                data: Some(vec![Some(1.0), Some(2.0), Some(3.0)]),
                total_change: Some(10.5),
            },
            low_confidence_sparkline: SparkLine {
                data: None,
                total_change: None,
            },
            implicit_modifiers: vec![],
            explicit_modifiers: vec![],
            flavour_text: "Test flavour".to_string(),
            corrupted: Some(false),
            gem_level: None,
            gem_quality: None,
            item_type: Some("Unique".to_string()),
            chaos_value: 150.0,
            exalted_value: Some(0.5),
            divine_value: Some(0.1),
            count: 10,
            details_id: "test".to_string(),
            listing_count: Some(5),
            links: Some(6),
            trade_info: None,
        };

        let price_info = PriceInfo::from_item(&item);
        
        assert_eq!(price_info.chaos_value, 150.0);
        assert_eq!(price_info.divine_value, Some(0.1));
        assert_eq!(price_info.exalted_value, Some(0.5));
        assert_eq!(price_info.change_24h, Some(10.5));
        
        println!("✓ PriceInfo::from_item works correctly");
    }

    #[test]
    fn test_price_info_from_currency() {
        let currency = CurrencyLine {
            currency_type_name: "Divine Orb".to_string(),
            pay: Some(CurrencyData {
                id: 1,
                league_id: 1,
                pay_currency_id: 1,
                get_currency_id: 2,
                sample_time_utc: "2024-01-01T00:00:00Z".to_string(),
                count: 100,
                value: 200.0,
                data_point_count: Some(50),
                includes_secondary: Some(false),
                listing_count: Some(25),
            }),
            receive: None,
            pay_spark_line: SparkLine {
                data: Some(vec![Some(1.0)]),
                total_change: Some(0.0),
            },
            receive_spark_line: SparkLine {
                data: None,
                total_change: None,
            },
            chaos_equivalent: Some(200.0),
            low_confidence_pay_spark_line: SparkLine {
                data: None,
                total_change: None,
            },
            low_confidence_receive_spark_line: SparkLine {
                data: None,
                total_change: None,
            },
            details_id: "divine-orb".to_string(),
        };

        let price_info = PriceInfo::from_currency(&currency);
        
        assert_eq!(price_info.chaos_value, 200.0);
        assert_eq!(price_info.divine_value, None);
        assert_eq!(price_info.exalted_value, None);
        assert_eq!(price_info.change_24h, Some(0.0));
        
        println!("✓ PriceInfo::from_currency works correctly");
    }

    #[test]
    fn test_json_serialization_deserialization() {
        // Test CurrencyLine serialization
        let currency = CurrencyLine {
            currency_type_name: "Chaos Orb".to_string(),
            pay: None,
            receive: None,
            pay_spark_line: SparkLine {
                data: Some(vec![Some(1.0)]),
                total_change: Some(0.0),
            },
            receive_spark_line: SparkLine {
                data: None,
                total_change: None,
            },
            chaos_equivalent: Some(1.0),
            low_confidence_pay_spark_line: SparkLine {
                data: None,
                total_change: None,
            },
            low_confidence_receive_spark_line: SparkLine {
                data: None,
                total_change: None,
            },
            details_id: "chaos-orb".to_string(),
        };

        let json = serde_json::to_string(&currency).expect("Failed to serialize");
        let deserialized: CurrencyLine = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(currency.currency_type_name, deserialized.currency_type_name);
        assert_eq!(currency.chaos_equivalent, deserialized.chaos_equivalent);
        assert_eq!(currency.details_id, deserialized.details_id);
        
        println!("✓ CurrencyLine JSON serialization/deserialization works");
    }

    #[test]
    fn test_struct_field_mapping() {
        // Test that serde field mapping works correctly with snake_case to camelCase
        let json_data = r#"{
            "currencyTypeName": "Test Currency",
            "chaosEquivalent": 5.5,
            "detailsId": "test-currency",
            "paySparkLine": {
                "data": [1.0, 2.0, 3.0],
                "totalChange": 15.5
            },
            "receiveSparkLine": {
                "data": null,
                "totalChange": null
            },
            "lowConfidencePaySparkLine": {
                "data": null,
                "totalChange": null
            },
            "lowConfidenceReceiveSparkLine": {
                "data": null,
                "totalChange": null
            }
        }"#;

        let currency: CurrencyLine = serde_json::from_str(json_data)
            .expect("Failed to deserialize test currency JSON");
        
        assert_eq!(currency.currency_type_name, "Test Currency");
        assert_eq!(currency.chaos_equivalent, Some(5.5));
        assert_eq!(currency.details_id, "test-currency");
        assert_eq!(currency.pay_spark_line.total_change, Some(15.5));
        
        if let Some(data) = currency.pay_spark_line.data {
            assert_eq!(data, vec![Some(1.0), Some(2.0), Some(3.0)]);
        }
        
        println!("✓ Serde field mapping works correctly");
    }

    #[test] 
    fn test_item_json_field_mapping() {
        let json_data = r#"{
            "id": 123,
            "name": "Test Weapon",
            "icon": "test.png",
            "mapTier": null,
            "levelRequired": 60,
            "baseType": "Long Sword",
            "stackSize": null,
            "variant": null,
            "itemClass": 6,
            "sparkline": {
                "data": [100.0, 110.0, 120.0],
                "totalChange": 20.0
            },
            "lowConfidenceSparkline": {
                "data": null,
                "totalChange": null
            },
            "implicitModifiers": [],
            "explicitModifiers": [
                {
                    "text": "+50 to maximum Life",
                    "optional": false
                }
            ],
            "flavourText": "Test item",
            "corrupted": false,
            "gemLevel": null,
            "gemQuality": null,
            "itemType": "weapon.sword",
            "chaosValue": 150.75,
            "exaltedValue": null,
            "divineValue": 0.75,
            "count": 15,
            "detailsId": "test-weapon",
            "listingCount": 8,
            "links": 6,
            "tradeInfo": []
        }"#;

        let item: ItemLine = serde_json::from_str(json_data)
            .expect("Failed to deserialize test item JSON");
        
        assert_eq!(item.id, 123);
        assert_eq!(item.name, "Test Weapon");
        assert_eq!(item.level_required, Some(60));
        assert_eq!(item.base_type, Some("Long Sword".to_string()));
        assert_eq!(item.chaos_value, 150.75);
        assert_eq!(item.divine_value, Some(0.75));
        assert_eq!(item.links, Some(6));
        assert_eq!(item.explicit_modifiers.len(), 1);
        assert_eq!(item.explicit_modifiers[0].text, "+50 to maximum Life");
        assert!(!item.explicit_modifiers[0].optional);
        
        if let Some(data) = item.sparkline.data {
            assert_eq!(data, vec![Some(100.0), Some(110.0), Some(120.0)]);
        }
        assert_eq!(item.sparkline.total_change, Some(20.0));
        
        println!("✓ ItemLine JSON field mapping works correctly");
    }

    #[test]
    fn test_optional_fields_handling() {
        // Test that optional fields are handled correctly when they're null or missing
        let minimal_item_json = r#"{
            "id": 1,
            "name": "Minimal Item",
            "icon": "minimal.png",
            "sparkline": {
                "data": null,
                "totalChange": null
            },
            "lowConfidenceSparkline": {
                "data": null,
                "totalChange": null
            },
            "implicitModifiers": [],
            "explicitModifiers": [],
            "flavourText": "",
            "chaosValue": 1.0,
            "count": 1,
            "detailsId": "minimal",
            "tradeInfo": []
        }"#;

        let item: ItemLine = serde_json::from_str(minimal_item_json)
            .expect("Failed to deserialize minimal item JSON");
        
        assert_eq!(item.id, 1);
        assert_eq!(item.name, "Minimal Item");
        assert_eq!(item.chaos_value, 1.0);
        
        // Check that optional fields are None
        assert!(item.map_tier.is_none());
        assert!(item.level_required.is_none());
        assert!(item.base_type.is_none());
        assert!(item.divine_value.is_none());
        assert!(item.links.is_none());
        assert!(item.gem_level.is_none());
        
        println!("✓ Optional fields handling works correctly");
    }
}