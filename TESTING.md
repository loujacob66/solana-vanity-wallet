# Testing Documentation

This document describes the comprehensive testing strategy for the Solana Vanity Wallet Generator.

## Test Categories

### 1. Unit Tests (`cargo test`)
- **Location**: `src/lib.rs`
- **Coverage**: Core functionality, cryptographic operations, validation
- **Tests**: 18 comprehensive tests covering all modes and edge cases

#### Key Test Areas:
- ✅ **Fast Mode Validation** - Ensures fast mode generates valid keypairs
- ✅ **Mnemonic Mode Validation** - Ensures mnemonic mode generates wallet-compatible keypairs
- ✅ **BIP44 Derivation** - Tests correct Solana derivation path (m/44'/501'/0'/0')
- ✅ **Cryptographic Integrity** - Signature verification, Base58 encoding
- ✅ **Deterministic Behavior** - Same inputs produce same outputs
- ✅ **Uniqueness** - Different inputs produce different outputs
- ✅ **Serialization** - Keypair serialization/deserialization
- ✅ **Known Test Vectors** - Compatibility with standard wallet implementations

### 2. Integration Tests (CI/CD)
- **Location**: `.github/workflows/ci.yml`
- **Coverage**: End-to-end functionality, CLI behavior, real-world usage

#### Test Scenarios:
- ✅ **CLI Help** - `--help` flag works correctly
- ✅ **Invalid Input Handling** - Proper error messages for invalid prefixes
- ✅ **Output Formats** - JSON and text output work correctly
- ✅ **Mode Switching** - Fast and mnemonic modes work as expected
- ✅ **File Output** - Output directory and file creation

### 3. Security Tests
- **Location**: `.cargo/audit.toml` + CI workflow
- **Coverage**: Dependency vulnerabilities, known security issues

#### Security Checks:
- ✅ **Vulnerability Scanning** - `cargo audit` with configured exceptions
- ✅ **Dependency Monitoring** - Automated alerts for new vulnerabilities
- ✅ **Cryptographic Validation** - Proper Ed25519 signature verification
- ✅ **Randomness Quality** - Cryptographically secure random number generation

### 4. Performance Tests
- **Location**: CI workflow performance job
- **Coverage**: Speed comparison, resource usage

#### Performance Metrics:
- ✅ **Fast Mode Speed** - Direct keypair generation performance
- ✅ **Mnemonic Mode Speed** - BIP44 derivation performance comparison
- ✅ **Memory Usage** - Efficient resource utilization
- ✅ **Cross-Platform Performance** - Consistent behavior across OS

### 5. Cross-Platform Tests
- **Location**: CI build matrix
- **Coverage**: Linux, Windows, macOS compatibility

#### Platform Coverage:
- ✅ **Linux** (Ubuntu Latest) - Primary development platform
- ✅ **Windows** (Windows Latest) - Windows compatibility
- ✅ **macOS** (macOS Latest) - macOS compatibility
- ✅ **Multiple Rust Versions** - Stable, Beta, Nightly

## Running Tests Locally

### All Tests
```bash
cargo test
```

### Specific Test Categories
```bash
# Unit tests only
cargo test --lib

# Integration tests only  
cargo test --test integration

# Tests with output
cargo test -- --nocapture

# Specific test
cargo test test_known_mnemonic_compatibility -- --nocapture
```

### Security Audit
```bash
# Install audit tool
cargo install cargo-audit

# Run security audit
cargo audit

# Run with configured exceptions
cargo audit --ignore RUSTSEC-2024-0344 --ignore RUSTSEC-2022-0093
```

### Performance Testing
```bash
# Build optimized release
cargo build --release

# Test fast mode
./target/release/solana-vanity-wallet A

# Test mnemonic mode
./target/release/solana-vanity-wallet A --with-mnemonic

# Compare performance
time ./target/release/solana-vanity-wallet A
time ./target/release/solana-vanity-wallet A --with-mnemonic
```

## CI/CD Pipeline

### Trigger Events
- **Push** to `main`, `master`, or `develop` branches
- **Pull Request** to `main` or `master` branches

### Pipeline Jobs

#### 1. Test Job
- **Runs on**: Ubuntu Latest
- **Matrix**: Rust stable, beta, nightly
- **Steps**:
  - Code formatting check (`cargo fmt`)
  - Linting (`cargo clippy`)
  - Unit tests (`cargo test`)
  - Release build (`cargo build --release`)

#### 2. Build Job
- **Runs on**: Ubuntu, Windows, macOS
- **Steps**:
  - Cross-platform compilation
  - Artifact generation
  - Binary upload

#### 3. Security Job
- **Runs on**: Ubuntu Latest
- **Steps**:
  - Security audit (`cargo audit`)
  - Vulnerability scanning with configured exceptions
  - Dependency monitoring

#### 4. Integration Job
- **Runs on**: Ubuntu Latest
- **Depends on**: Test job
- **Steps**:
  - CLI functionality testing
  - Error handling verification
  - Output format validation
  - Mode switching tests

#### 5. Performance Job
- **Runs on**: Ubuntu Latest
- **Depends on**: Test job
- **Steps**:
  - Performance comparison
  - Resource usage monitoring
  - Speed benchmarking

## Test Configuration

### Security Audit Configuration
File: `.cargo/audit.toml`
```toml
[advisories]
# Ignore timing variability in curve25519-dalek - doesn't affect our key generation
# Ignore ed25519-dalek oracle attack - we don't expose vulnerable signing APIs
ignore = ["RUSTSEC-2024-0344", "RUSTSEC-2022-0093"]

# Still show informational warnings for other issues
informational_warnings = ["unmaintained", "unsound"]
```

### Known Test Vectors
- **Test Mnemonic**: `"abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"`
- **Expected Address**: `HAgk14JpMQLgt6rVgv7cBQFJWFto5Dqxi472uT3DKpqk`
- **Derivation Path**: `m/44'/501'/0'/0'`

## Test Results Interpretation

### Success Criteria
- ✅ All unit tests pass (18/18)
- ✅ Security audit passes with configured exceptions
- ✅ Integration tests validate CLI behavior
- ✅ Cross-platform builds succeed
- ✅ Performance tests complete within timeout

### Failure Handling
- ❌ **Test Failures**: CI will fail, PR cannot be merged
- ❌ **Security Issues**: New vulnerabilities will fail CI
- ❌ **Performance Regression**: Tracked but non-blocking
- ❌ **Cross-Platform Issues**: Platform-specific investigation needed

## Continuous Monitoring

### Automated Checks
- **Daily**: Dependency vulnerability scanning
- **Weekly**: Performance regression testing
- **On Release**: Full test suite + security audit

### Manual Testing
- **Before Release**: Manual verification of key features
- **Cross-Platform**: Manual testing on different OS
- **Wallet Compatibility**: Manual import testing with real wallets

## Contributing

When adding new features:
1. Add corresponding unit tests
2. Update integration tests if needed
3. Ensure all tests pass locally
4. Verify CI pipeline passes
5. Add performance considerations if applicable

## Security Considerations

### Test Security
- Tests use deterministic seeds for reproducibility
- No private keys are logged or exposed
- Test vectors are publicly known and safe to use
- Real entropy is used for randomness tests

### Production Security
- Tests validate cryptographic correctness
- Security audit catches known vulnerabilities
- Manual review required for cryptographic changes
- Performance tests ensure no timing attacks via observable delays
