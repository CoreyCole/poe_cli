# PoE Ninja CLI

A command-line interface for querying Path of Exile item prices from the poe.ninja API.

## Features

- 🔍 Query currency exchange rates and prices
- 📦 Search for item prices across different categories
- 🎯 Filter results by name, price range, and other criteria
- 📊 Display results in clean, formatted tables
- 🌈 Colored output for better readability
- ⚡ Fast async HTTP requests with proper error handling

## Usage

### Running the CLI

You can run the CLI in two ways:

**Using cargo run (for development/testing):**

```bash
cargo run -- [command] [options]
```

### Basic Commands

#### Get Currency Information

```bash
# Get all currency prices for Standard league
cargo run -- currency --league Standard

# Get specific currency type
cargo run -- currency --league Settlers --currency-type Fragment

# Filter by currency name
cargo run -- currency --league Standard --name "Exalted"
```

#### Get Item Information

```bash
# Get unique weapon prices
cargo run -- item --league Standard --item-type UniqueWeapon

# Filter by item name
cargo run -- item --league Settlers --item-type UniqueArmour --name "Belly"

# Filter by price range (in chaos orbs)
cargo run -- item --league Standard --item-type Essence --min-chaos 10 --max-chaos 100
```

#### List Available Options

```bash
# Show available leagues
cargo run -- leagues

# Show available item types
cargo run -- types
```

### Command Options

#### Currency Command

- `--league, -l`: League name (default: "Standard")
- `--currency-type, -c`: Currency type - "Currency" or "Fragment" (default: "Currency")
- `--name, -n`: Filter by currency name (partial match)

#### Item Command

- `--league, -l`: League name (default: "Standard")
- `--item-type, -i`: Item type (required) - see available types with `cargo run -- types`
- `--name, -n`: Filter by item name (partial match)
- `--min-chaos`: Minimum chaos value filter
- `--max-chaos`: Maximum chaos value filter

### Examples

```bash
# Find Mirror of Kalandra price in current league
cargo run -- currency --league Settlers --name "Mirror"

# Find all expensive unique weapons (>100 chaos)
cargo run -- item --league Standard --item-type UniqueWeapon --min-chaos 100

# Search for skill gems containing "Enlighten"
cargo run -- item --league Standard --item-type SkillGem --name "Enlighten"

# Get essence prices between 5-50 chaos
cargo run -- item --league Settlers --item-type Essence --min-chaos 5 --max-chaos 50

# Quick examples for testing
cargo run -- leagues
cargo run -- types
cargo run -- currency -l Standard -n "Chaos"
cargo run -- item -l Standard -i UniqueWeapon --min-chaos 50
```

## Item Types

### Currency Overview Types

- `Currency` - Standard currencies (Chaos, Exalted, Divine, etc.)
- `Fragment` - Map fragments and keys

### Item Overview Types

- `Oil` - Blight oils
- `Incubator` - Legion incubators
- `Scarab` - Atlas scarabs
- `Fossil` - Delve fossils
- `Resonator` - Delve resonators
- `Essence` - Essences
- `DivinationCard` - Divination cards
- `SkillGem` - Skill and support gems
- `BaseType` - High-level base items
- `HelmetEnchant` - Labyrinth enchants
- `UniqueMap` - Unique maps
- `Map` - Regular maps
- `UniqueJewel` - Unique jewels
- `UniqueFlask` - Unique flasks
- `UniqueWeapon` - Unique weapons
- `UniqueArmour` - Unique armour
- `UniqueAccessory` - Unique accessories
- `Beast` - Bestiary beasts
- `Vials` - Sanctum vials
- `DeliriumOrb` - Delirium orbs
- `Omen` - Omens
- `UniqueRelic` - Unique relics
- `ClusterJewel` - Cluster jewels
- `BlightedMap` - Blighted maps
- `BlightRavagedMap` - Blight-ravaged maps
- `Invitation` - Maven invitations
- `Memory` - Synthesis memories
- `Coffin` - Necropolis coffins
- `AllflameEmber` - Allflame embers

## League Names

Common league names:

- `Standard`
- `Hardcore`
- `Settlers` (current league)
- `Hardcore Settlers`
- `Solo Self-Found`
- `Hardcore Solo Self-Found`

Note: Replace spaces with `%20` when using league names with spaces in URLs.

## Output Format

The CLI displays results in formatted tables with the following information:

### Currency Table

- Currency name
- Chaos equivalent value
- Pay/receive values and counts

### Item Table

- Item name and base type
- Chaos and divine values
- Item count and listing count
- Level requirement

## Error Handling

The CLI provides clear error messages for common issues:

- Invalid league names
- Unknown item types
- API connection failures
- No results found

## Rate Limiting

The client respects poe.ninja's rate limits by:

- Using a reasonable timeout (30 seconds)
- Setting proper User-Agent headers
- Avoiding concurrent requests

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## Testing

### Running Tests

The project includes comprehensive tests to ensure reliability:

```bash
# Run all unit tests (library functions)
cargo test

# Run comprehensive integration tests (CLI parsing, handlers, edge cases)
cargo test --test integration_tests

# Run API compatibility tests (requires internet connection)
cargo test --test integration_tests --features network-tests

# Run specific test categories
cargo test cli_tests          # CLI argument parsing tests
cargo test handler_tests      # Business logic with mock data
cargo test property_tests     # Property-based testing with random inputs
cargo test helper_tests       # Utility function tests
cargo test snapshot_tests     # Help output validation
cargo test edge_case_tests    # Edge cases and error conditions
```

### Test Coverage

**Unit Tests (17 tests in `src/lib.rs`):**

- Currency and item filtering functions
- Sorting algorithms
- Helper functions for leagues and types
- Data structure validation

**Integration Tests (28 tests in `tests/integration_tests.rs`):**

**CLI Argument Parsing Tests (8 tests):**

- Command parsing validation
- Default value handling
- Short argument forms
- Invalid argument rejection
- Required parameter validation

**Handler Tests (6 tests):**

- Currency filtering and sorting with mock data
- Item price range filtering
- Name-based filtering (case-insensitive)
- Combined filtering operations
- Business logic validation

**Property-based Tests (3 tests):**

- Random input validation using `proptest`
- League name parsing with arbitrary strings
- Chaos value parsing with random floats
- Item type validation

**Helper Function Tests (4 tests):**

- Available leagues list validation
- Currency types verification
- Item types comprehensive checking
- Static data integrity

**Snapshot Tests (3 tests):**

- Help output structure validation
- Subcommand help text verification
- CLI documentation consistency

**Edge Case Tests (4 tests):**

- Empty data handling
- Case-insensitive filtering
- Extreme price values (0.0 to 999999.0)
- Boundary condition testing

**API Compatibility Tests (6 tests - requires `network-tests` feature):**

- Live poe.ninja API integration
- Currency and item endpoint validation
- Data structure compatibility
- Multiple item/currency type testing
- Field mapping verification

### Test Examples

```bash
# Quick validation (no network required) - 45 tests
cargo test

# Test CLI argument parsing specifically
cargo test test_currency_command_parsing
cargo test test_item_command_parsing

# Test business logic with mock data
cargo test test_currency_filtering
cargo test test_item_price_filtering

# Test edge cases
cargo test test_case_insensitive_filtering
cargo test test_extreme_chaos_values

# Full integration testing with live API calls
cargo test --features network-tests
```

### Test Categories Explained

**Fast Tests (no network):** All library unit tests, CLI parsing, mock data handlers, property-based tests, and edge cases. These run quickly and can be executed frequently during development.

**Network Tests:** Live API calls to poe.ninja for end-to-end validation. These require internet connection and test real API compatibility.

**Property-based Tests:** Use `proptest` to generate random inputs and verify that parsing and validation logic handles edge cases correctly.

**Snapshot Tests:** Verify that help output and CLI documentation remain consistent across changes.

### Testing Philosophy

The testing approach follows multiple strategies:

1. **Unit Testing:** Core business logic with deterministic inputs
2. **Integration Testing:** CLI parsing and command validation
3. **Mock Testing:** Handler logic with controlled test data
4. **Property Testing:** Random input validation for robustness
5. **End-to-end Testing:** Live API compatibility verification
6. **Edge Case Testing:** Boundary conditions and error handling

This comprehensive approach ensures the CLI remains reliable and functional as the codebase evolves.

## License

MIT License - see LICENSE file for details

## Disclaimer

This tool is not affiliated with or endorsed by Grinding Gear Games. Path of Exile is a trademark of Grinding Gear Games.

## API Reference

This CLI uses the poe.ninja API:

- Base URL: `https://poe.ninja/api/data`
- Currency endpoint: `/currencyoverview?league={league}&type={type}`
- Item endpoint: `/itemoverview?league={league}&type={type}`

For more information about the API, visit [poe.ninja](https://poe.ninja).
