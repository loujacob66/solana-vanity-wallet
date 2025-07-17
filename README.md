# Solana Vanity Wallet Generator

A fast and efficient Solana vanity wallet generator that creates wallets with custom prefixes using parallel processing.

## Features

- 🚀 **High Performance**: Uses all CPU cores for maximum efficiency
- 🎯 **Vanity Addresses**: Generate wallets with custom prefixes
- 🔐 **Complete Wallet Info**: Outputs both 12-word mnemonic and JSON wallet data
- 📊 **Flexible Output**: Text or JSON format output
- 📈 **Real-time Statistics**: Live progress tracking with iterations/second, ETA, and progress percentage
- 🎲 **Luck Analysis**: Shows how your result compares to statistical expectations
- 📁 **Automatic Logging**: Each run is automatically saved to timestamped JSON files in the `output/` directory
- ✅ **Input Validation**: Ensures only valid Base58 characters are used in prefixes
- ⚡ **Rust-powered**: Built with Rust for maximum performance

## Installation

1. Make sure you have Rust installed: https://rustup.rs/
2. Clone or download this project
3. Build the release version:
   ```bash
   cargo build --release
   ```

## Usage

### Basic Usage
```bash
# Generate a wallet with prefix "ABC"
./target/release/solana-vanity-wallet ABC

# Generate a wallet with prefix "Sol" and JSON output
./target/release/solana-vanity-wallet --format json Sol
```

### Output Example (Text Format)
```
🚀 Solana Vanity Wallet Generator
==================================
Prefix: ABC
Threads: 8
Expected iterations: 97.3K
Estimated difficulty: 1 in 195K

🔍 Iterations: 45.2K | Rate: 125.6K/s | Progress: 46.4% | ETA: 25.3s | Elapsed: 12.1s

🎉 SUCCESS! Vanity wallet generated!
====================================
Total iterations: 45,234
Time elapsed: 12.1s
Average rate: 125.6K/s
Luck factor: 2.15x better than expected

📝 Wallet Details:
Mnemonic: abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about
Public Key: ABCdef123456789...
Secret Key: [base58 encoded secret key]
```

### Output Example (JSON Format)
```json
{
  "mnemonic": "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
  "public_key": "ABCdef123456789...",
  "secret_key": "[base58 encoded secret key]",
  "statistics": {
    "iterations": 45234,
    "elapsed_seconds": 12.1,
    "iterations_per_second": 125634.7,
    "expected_iterations": 97336,
    "luck_factor": 2.15
  }
}
```

## Performance Notes

- **Difficulty**: Each additional character in the prefix roughly increases difficulty by 58x
- **Recommended**: Start with 1-3 character prefixes for reasonable generation times
- **Hardware**: More CPU cores = faster generation
- **Memory**: Uses minimal memory, most resources go to CPU

## Prefix Difficulty Guide

| Prefix Length | Approximate Time |
|---------------|------------------|
| 1 character   | Instant          |
| 2 characters  | Few seconds      |
| 3 characters  | Minutes          |
| 4 characters  | Hours            |
| 5+ characters | Days to weeks    |

## Security

- Uses cryptographically secure random number generation
- Generates proper BIP39 12-word mnemonics
- Compatible with all standard Solana wallets
- Secret keys are base58 encoded for direct use

## Command Line Options

```
Usage: solana-vanity-wallet [OPTIONS] <PREFIX>

Arguments:
  <PREFIX>  Desired prefix for the wallet

Options:
  -f, --format <FORMAT>  Output format (json or text) [default: text]
  -h, --help             Print help
```

## Examples

```bash
# Quick generation with single character
./target/release/solana-vanity-wallet 1

# Generate wallet starting with "Sol"
./target/release/solana-vanity-wallet Sol

# Generate with JSON output for scripting
./target/release/solana-vanity-wallet --format json MyPrefix > wallet.json
```

## Prefix Validation

The application validates that all characters in your prefix are valid Base58 characters used in Solana addresses:

### Valid Characters:
- **Numbers**: `1-9` (excludes `0` to avoid confusion)
- **Uppercase**: `A-Z` (excludes `O` to avoid confusion)
- **Lowercase**: `a-z` (excludes `l` to avoid confusion)

### Invalid Characters:
- `0` (zero), `O` (capital O), `l` (lowercase L)
- Special characters: `_`, `+`, `=`, `/`, `-`, etc.

### Examples:
```bash
# Valid prefixes
./target/release/solana-vanity-wallet ABC
./target/release/solana-vanity-wallet Sol
./target/release/solana-vanity-wallet 123
./target/release/solana-vanity-wallet MyWallet

# Invalid prefixes (will show error)
./target/release/solana-vanity-wallet 0     # Contains zero
./target/release/solana-vanity-wallet Test_ # Contains underscore
./target/release/solana-vanity-wallet SOL0  # Contains zero
```

### Error Example:
```bash
$ ./target/release/solana-vanity-wallet _
❌ Error: Invalid prefix '_'

Valid Base58 characters are:
  Numbers: 1-9 (excludes 0)
  Uppercase: A-Z (excludes O)
  Lowercase: a-z (excludes l)

Examples of valid prefixes: ABC, Sol, 123, MyWallet, IJKL
Examples of invalid prefixes: 0, O, l, _, +, =, /
```

## Automatic Logging

Every run is automatically logged to a JSON file in the `output/` directory using the first 10 characters of the generated wallet:

```bash
# Files are named with the first 10 characters of the public key
output/
├── BbGW5Yqtsa_output.json
├── C744n3594g_output.json
├── FUCKG3EWss_output.json
└── ...
```

Each log file contains:
- **Wallet data**: Mnemonic, public key, secret key, and keypair JSON array
- **Statistics**: Iterations, elapsed time, rate, and luck factor (when using `--format json`)
- **Metadata**: Timestamp and generation parameters

### Log File Format
```json
{
  "mnemonic": "word1 word2 word3 ... word12",
  "public_key": "Base58EncodedPublicKey",
  "secret_key": "Base58EncodedSecretKey",
  "keypair_json": [1, 2, 3, ...],
  "statistics": {
    "iterations": 12345,
    "elapsed_seconds": 1.23,
    "iterations_per_second": 10000.0,
    "expected_iterations": 58000,
    "luck_factor": 4.7
  }
}
```

## Technical Details

- Built with Rust for maximum performance
- Uses `rayon` for parallel processing across all CPU cores
- Implements proper BIP39 mnemonic generation
- Uses Solana SDK for keypair generation
- Base58 encoding for compatibility with Solana ecosystem

## License

This project is open source. Use at your own risk and responsibility.
