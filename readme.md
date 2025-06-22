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
# Run all unit tests
cargo test

# Run integration tests (CLI command validation)
cargo test --test cli_tests

# Run network-dependent integration tests (requires internet)
cargo test --test cli_tests --features network-tests

# Run all tests including comprehensive integration tests
cargo test --test integration_tests --features network-tests
```

### Test Coverage

**Unit Tests:**

- API client functionality
- Data structure serialization/deserialization
- Type compatibility with poe.ninja API

**Integration Tests:**

- CLI argument parsing and validation
- Command help text and structure
- Error handling for invalid inputs
- Basic command functionality (leagues, types)

**Network Integration Tests** (optional):

- Live API calls to poe.ninja
- End-to-end command execution
- Real data fetching and formatting

### Test Examples

```bash
# Quick validation (no network required)
cargo test test_help_displays_correctly
cargo test test_leagues_command_works
cargo test test_types_command_works

# Comprehensive testing with network calls
cargo test --features network-tests
```

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
