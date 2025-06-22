use anyhow::{Context, Result};
use reqwest::Client;
use std::time::Duration;

use crate::types::*;

pub struct PoeNinjaClient {
    client: Client,
    base_url: String,
}

impl PoeNinjaClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("poe-ninja-cli/0.1.0")
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: "https://poe.ninja/api/data".to_string(),
        }
    }

    /// Get currency overview data
    pub async fn get_currency_overview(
        &self,
        league: &str,
        currency_type: &str,
    ) -> Result<CurrencyOverviewResponse> {
        let url = format!(
            "{}/currencyoverview?league={}&type={}",
            self.base_url,
            urlencoding::encode(league),
            currency_type
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to send request")?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "API request failed with status: {}",
                response.status()
            ));
        }

        let data: CurrencyOverviewResponse = response
            .json()
            .await
            .context("Failed to parse currency response")?;

        Ok(data)
    }

    /// Get item overview data
    pub async fn get_item_overview(
        &self,
        league: &str,
        item_type: &str,
    ) -> Result<ItemOverviewResponse> {
        let url = format!(
            "{}/itemoverview?league={}&type={}",
            self.base_url,
            urlencoding::encode(league),
            item_type
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to send request")?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "API request failed with status: {}",
                response.status()
            ));
        }

        let data: ItemOverviewResponse = response
            .json()
            .await
            .context("Failed to parse item response")?;

        Ok(data)
    }
}

// URL encoding helper
mod urlencoding {
    pub fn encode(input: &str) -> String {
        input
            .chars()
            .map(|c| match c {
                ' ' => "%20".to_string(),
                c if c.is_ascii_alphanumeric() => c.to_string(),
                c => format!("%{:02X}", c as u8),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    // Test-only utility methods
    impl PoeNinjaClient {
        /// Get specific item by name (test utility)
        #[allow(dead_code)]
        async fn find_item_by_name(
            &self,
            league: &str,
            item_type: &str,
            item_name: &str,
        ) -> Result<Option<ItemLine>> {
            let response = self.get_item_overview(league, item_type).await?;
            
            let item = response
                .lines
                .into_iter()
                .find(|item| item.name.to_lowercase() == item_name.to_lowercase());

            Ok(item)
        }

        /// Get specific currency by name (test utility)
        #[allow(dead_code)]
        async fn find_currency_by_name(
            &self,
            league: &str,
            currency_type: &str,
            currency_name: &str,
        ) -> Result<Option<CurrencyLine>> {
            let response = self.get_currency_overview(league, currency_type).await?;
            
            let currency = response
                .lines
                .into_iter()
                .find(|currency| currency.currency_type_name.to_lowercase() == currency_name.to_lowercase());

            Ok(currency)
        }
    }

    #[tokio::test]
    async fn test_currency_api_raw_response() {
        println!("\n=== Testing Currency API Raw Response ===");
        
        let client = PoeNinjaClient::new();
        let url = format!(
            "{}/currencyoverview?league={}&type={}",
            client.base_url,
            urlencoding::encode("Standard"),
            "Currency"
        );

        println!("Making request to: {}", url);
        
        let response = client
            .client
            .get(&url)
            .send()
            .await
            .expect("Failed to send request");

        assert!(response.status().is_success(), "API request failed: {}", response.status());
        
        let raw_text = response.text().await.expect("Failed to get response text");
        println!("Raw API Response (first 500 chars):");
        println!("{}", &raw_text[..std::cmp::min(500, raw_text.len())]);
        println!("...(truncated)");
        
        // Parse as generic JSON to inspect structure
        let json_value: serde_json::Value = serde_json::from_str(&raw_text)
            .expect("Failed to parse as JSON");
        
        println!("\nJSON Structure:");
        println!("{}", serde_json::to_string_pretty(&json_value).unwrap_or_else(|_| "Failed to pretty print".to_string()));
    }

    #[tokio::test]
    async fn test_currency_struct_deserialization() {
        println!("\n=== Testing Currency Struct Deserialization ===");
        
        let client = PoeNinjaClient::new();
        
        match client.get_currency_overview("Standard", "Currency").await {
            Ok(response) => {
                println!("✓ Successfully deserialized CurrencyOverviewResponse");
                println!("Lines count: {}", response.lines.len());
                println!("Currency details count: {}", response.currency_details.len());
                
                if let Some(first_currency) = response.lines.first() {
                    println!("\nFirst currency details:");
                    println!("  Name: {}", first_currency.currency_type_name);
                    println!("  Chaos Equivalent: {:?}", first_currency.chaos_equivalent);
                    println!("  Details ID: {}", first_currency.details_id);
                    
                    if let Some(pay) = &first_currency.pay {
                        println!("  Pay - Count: {}, Value: {}", pay.count, pay.value);
                    }
                    
                    if let Some(receive) = &first_currency.receive {
                        println!("  Receive - Count: {}, Value: {}", receive.count, receive.value);
                    }
                    
                    // Test sparkline data
                    if let Some(data) = &first_currency.pay_spark_line.data {
                        println!("  Pay sparkline data points: {}", data.len());
                        let non_null_count = data.iter().filter(|x| x.is_some()).count();
                        println!("  Non-null pay sparkline points: {}", non_null_count);
                    }
                }
                
                // Test specific currency lookup
                match client.find_currency_by_name("Standard", "Currency", "Chaos Orb").await {
                    Ok(Some(chaos_orb)) => {
                        println!("\n✓ Successfully found Chaos Orb");
                        println!("  Chaos Equivalent: {:?}", chaos_orb.chaos_equivalent);
                    }
                    Ok(None) => println!("⚠ Chaos Orb not found in response"),
                    Err(e) => println!("✗ Error finding Chaos Orb: {}", e),
                }
            }
            Err(e) => {
                panic!("Failed to deserialize currency response: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_item_api_raw_response() {
        println!("\n=== Testing Item API Raw Response ===");
        
        let client = PoeNinjaClient::new();
        let url = format!(
            "{}/itemoverview?league={}&type={}",
            client.base_url,
            urlencoding::encode("Standard"),
            "UniqueWeapon"
        );

        println!("Making request to: {}", url);
        
        let response = client
            .client
            .get(&url)
            .send()
            .await
            .expect("Failed to send request");

        assert!(response.status().is_success(), "API request failed: {}", response.status());
        
        let raw_text = response.text().await.expect("Failed to get response text");
        println!("Raw API Response (first 500 chars):");
        println!("{}", &raw_text[..std::cmp::min(500, raw_text.len())]);
        println!("...(truncated)");
        
        // Parse as generic JSON to inspect structure
        let json_value: serde_json::Value = serde_json::from_str(&raw_text)
            .expect("Failed to parse as JSON");
        
        println!("\nJSON Structure (first item only):");
        if let Some(lines) = json_value.get("lines") {
            if let Some(first_item) = lines.get(0) {
                println!("{}", serde_json::to_string_pretty(first_item).unwrap_or_else(|_| "Failed to pretty print".to_string()));
            }
        }
    }

    #[tokio::test]
    async fn test_item_struct_deserialization() {
        println!("\n=== Testing Item Struct Deserialization ===");
        
        let client = PoeNinjaClient::new();
        
        match client.get_item_overview("Standard", "UniqueWeapon").await {
            Ok(response) => {
                println!("✓ Successfully deserialized ItemOverviewResponse");
                println!("Lines count: {}", response.lines.len());
                
                if let Some(first_item) = response.lines.first() {
                    println!("\nFirst item details:");
                    println!("  ID: {}", first_item.id);
                    println!("  Name: {}", first_item.name);
                    println!("  Base Type: {:?}", first_item.base_type);
                    println!("  Chaos Value: {}", first_item.chaos_value);
                    println!("  Divine Value: {:?}", first_item.divine_value);
                    println!("  Count: {}", first_item.count);
                    println!("  Level Required: {:?}", first_item.level_required);
                    println!("  Links: {:?}", first_item.links);
                    println!("  Implicit Modifiers: {}", first_item.implicit_modifiers.len());
                    println!("  Explicit Modifiers: {}", first_item.explicit_modifiers.len());
                    println!("  Trade Info: {}", first_item.trade_info.as_ref().map_or(0, |v| v.len()));
                    
                    // Test sparkline data
                    if let Some(data) = &first_item.sparkline.data {
                        println!("  Sparkline data points: {}", data.len());
                        let non_null_count = data.iter().filter(|x| x.is_some()).count();
                        println!("  Non-null sparkline points: {}", non_null_count);
                    }
                    println!("  Total change: {:?}", first_item.sparkline.total_change);
                }
                
                // Find an expensive item for testing
                let expensive_items: Vec<_> = response.lines
                    .iter()
                    .filter(|item| item.chaos_value > 100.0)
                    .take(3)
                    .collect();
                
                println!("\nTop expensive items (>100 chaos):");
                for item in expensive_items {
                    println!("  {} - {} chaos", item.name, item.chaos_value);
                }
            }
            Err(e) => {
                panic!("Failed to deserialize item response: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_multiple_item_types() {
        println!("\n=== Testing Multiple Item Types ===");
        
        let client = PoeNinjaClient::new();
        let item_types = vec!["Oil", "Essence", "DivinationCard", "SkillGem"];
        
        for item_type in item_types {
            println!("\nTesting item type: {}", item_type);
            
            match client.get_item_overview("Standard", item_type).await {
                Ok(response) => {
                    println!("  ✓ {} - {} items found", item_type, response.lines.len());
                    
                    if let Some(first) = response.lines.first() {
                        println!("    Sample: {} - {} chaos", first.name, first.chaos_value);
                    }
                }
                Err(e) => {
                    println!("  ✗ {} - Error: {}", item_type, e);
                }
            }
        }
    }

    #[tokio::test]
    async fn test_currency_types() {
        println!("\n=== Testing Currency Types ===");
        
        let client = PoeNinjaClient::new();
        let currency_types = vec!["Currency", "Fragment"];
        
        for currency_type in currency_types {
            println!("\nTesting currency type: {}", currency_type);
            
            match client.get_currency_overview("Standard", currency_type).await {
                Ok(response) => {
                    println!("  ✓ {} - {} currencies found", currency_type, response.lines.len());
                    println!("  ✓ {} - {} currency details found", currency_type, response.currency_details.len());
                    
                    if let Some(first) = response.lines.first() {
                        println!("    Sample: {} - {:?} chaos equivalent", 
                               first.currency_type_name, first.chaos_equivalent);
                    }
                }
                Err(e) => {
                    println!("  ✗ {} - Error: {}", currency_type, e);
                }
            }
        }
    }

    #[tokio::test]
    async fn test_field_compatibility() {
        println!("\n=== Testing Field Compatibility ===");
        
        let client = PoeNinjaClient::new();
        
        // Test currency fields
        match client.get_currency_overview("Standard", "Currency").await {
            Ok(response) => {
                println!("✓ Currency API compatibility test passed");
                
                for currency in response.lines.iter().take(3) {
                    println!("Currency: {}", currency.currency_type_name);
                    
                    // Test all optional fields
                    if currency.chaos_equivalent.is_some() {
                        println!("  ✓ chaos_equivalent field present");
                    }
                    
                    if currency.pay.is_some() {
                        println!("  ✓ pay field present");
                    }
                    
                    if currency.receive.is_some() {
                        println!("  ✓ receive field present");
                    }
                    
                    // Test sparkline structure
                    if currency.pay_spark_line.data.is_some() {
                        println!("  ✓ pay_spark_line.data present");
                    }
                    
                    if currency.pay_spark_line.total_change.is_some() {
                        println!("  ✓ pay_spark_line.total_change present");
                    }
                }
            }
            Err(e) => panic!("Currency field compatibility test failed: {}", e),
        }
        
        // Test item fields
        match client.get_item_overview("Standard", "UniqueArmour").await {
            Ok(response) => {
                println!("✓ Item API compatibility test passed");
                
                for item in response.lines.iter().take(3) {
                    println!("Item: {}", item.name);
                    
                    // Test required fields
                    assert!(!item.name.is_empty(), "Item name should not be empty");
                    assert!(item.chaos_value >= 0.0, "Chaos value should be non-negative");
                    
                    // Test optional fields
                    if item.base_type.is_some() {
                        println!("  ✓ base_type field present");
                    }
                    
                    if item.level_required.is_some() {
                        println!("  ✓ level_required field present");
                    }
                    
                    if item.divine_value.is_some() {
                        println!("  ✓ divine_value field present");
                    }
                    
                    if item.links.is_some() {
                        println!("  ✓ links field present");
                    }
                    
                    if item.gem_level.is_some() {
                        println!("  ✓ gem_level field present");
                    }
                    
                    if item.gem_quality.is_some() {
                        println!("  ✓ gem_quality field present");
                    }
                }
            }
            Err(e) => panic!("Item field compatibility test failed: {}", e),
        }
    }
}