# Distribution and Deployment Guide

This document explains how to distribute and deploy the Solana Vanity Wallet Generator.

## Distribution Methods

### 1. GitHub Releases (Primary)
**Purpose**: Direct binary downloads for end users
**How it works**: Automatic releases when you push version tags

#### For End Users:
1. Go to: https://github.com/ljacob/solana-vanity-wallet/releases
2. Download the appropriate binary for your OS:
   - `solana-vanity-wallet-linux-x64.tar.gz` (Linux)
   - `solana-vanity-wallet-windows-x64.exe.zip` (Windows)
   - `solana-vanity-wallet-macos-x64.tar.gz` (macOS Intel)
   - `solana-vanity-wallet-macos-arm64.tar.gz` (macOS Apple Silicon)

#### For Developers:
```bash
# Create a new release
git tag v1.0.0
git push origin v1.0.0
# This automatically triggers the release workflow
```

### 2. Cargo (Rust Package Manager)
**Purpose**: Install via `cargo install` for Rust users
**How it works**: Automatic publishing to crates.io on releases

#### For End Users:
```bash
# Install from crates.io
cargo install solana-vanity-wallet

# Run from anywhere
solana-vanity-wallet --help
```

#### For Developers:
- Package is automatically published to crates.io on GitHub releases
- Requires `CRATES_IO_TOKEN` secret in GitHub repository settings

### 3. Package Managers (Future)
**Purpose**: Native package manager integration
**Examples**: homebrew, chocolatey, APT, etc.

#### Homebrew (macOS/Linux):
```bash
# Future implementation
brew install solana-vanity-wallet
```

#### Chocolatey (Windows):
```powershell
# Future implementation
choco install solana-vanity-wallet
```

## CI/CD Pipeline Overview

### Current Pipeline:
```
Code Push → CI Tests → Build Matrix → Release Workflow → Distribution
     ↓           ↓          ↓              ↓               ↓
  GitHub     Tests Pass   Multi-OS      Tag Push      User Downloads
             Security     Binaries      Triggers      from Releases
             Clippy       Created       Release
```

### Build Matrix:
- **Linux**: x86_64-unknown-linux-gnu
- **Windows**: x86_64-pc-windows-msvc (with vendored OpenSSL)
- **macOS Intel**: x86_64-apple-darwin
- **macOS Apple Silicon**: aarch64-apple-darwin

## Accessing CI Builds

### Method 1: GitHub Actions Artifacts (Temporary)
**Purpose**: Download builds from CI runs for testing
**Limitation**: Only available for 90 days, requires GitHub login

1. Go to: https://github.com/ljacob/solana-vanity-wallet/actions
2. Click on a successful CI run
3. Scroll down to "Artifacts" section
4. Download the build for your OS

### Method 2: GitHub Releases (Permanent)
**Purpose**: Stable releases for end users
**Benefit**: Permanent downloads, no login required

1. Go to: https://github.com/ljacob/solana-vanity-wallet/releases
2. Download the latest release
3. Extract and run the binary

## Creating a Release

### 1. Prepare Release
```bash
# Update version in Cargo.toml
vim Cargo.toml  # Change version = "0.1.0" to "1.0.0"

# Test everything works
cargo test
cargo build --release
./target/release/solana-vanity-wallet --help

# Commit version bump
git add Cargo.toml Cargo.lock
git commit -m "Bump version to 1.0.0"
```

### 2. Create and Push Tag
```bash
# Create annotated tag
git tag -a v1.0.0 -m "Release v1.0.0: Initial stable release"

# Push tag to trigger release
git push origin v1.0.0
```

### 3. Monitor Release
1. Check GitHub Actions: https://github.com/ljacob/solana-vanity-wallet/actions
2. Verify all builds succeed
3. Check GitHub Releases: https://github.com/ljacob/solana-vanity-wallet/releases
4. Test downloaded binaries

## Best Practices

### Versioning
- Use [Semantic Versioning](https://semver.org/): `MAJOR.MINOR.PATCH`
- `v1.0.0` - Initial stable release
- `v1.0.1` - Bug fixes
- `v1.1.0` - New features
- `v2.0.0` - Breaking changes

### Release Process
1. **Test thoroughly** before tagging
2. **Update CHANGELOG.md** with release notes
3. **Bump version** in Cargo.toml
4. **Create annotated tags** with descriptive messages
5. **Monitor CI/CD** pipeline for failures
6. **Test downloaded binaries** before announcing

### Security
- All releases are built in GitHub's secure environment
- Binaries are signed (checksums provided)
- No manual binary uploads (reduces tampering risk)

## Troubleshooting

### Release Failed
1. Check GitHub Actions logs
2. Common issues:
   - Windows OpenSSL problems (should be fixed with vendored OpenSSL)
   - Missing secrets (CRATES_IO_TOKEN)
   - Version conflicts on crates.io

### Binary Won't Run
1. Check architecture matches your system
2. On macOS: Run `xattr -dr com.apple.quarantine <binary>` if quarantined
3. On Linux: Ensure binary is executable: `chmod +x <binary>`

## Integration Examples

### Download Script
```bash
#!/bin/bash
# Download latest release
LATEST=$(curl -s https://api.github.com/repos/ljacob/solana-vanity-wallet/releases/latest | grep tag_name | cut -d '"' -f 4)
wget https://github.com/ljacob/solana-vanity-wallet/releases/download/${LATEST}/solana-vanity-wallet-linux-x64.tar.gz
tar -xzf solana-vanity-wallet-linux-x64.tar.gz
./solana-vanity-wallet --help
```

### Docker (Future)
```dockerfile
FROM scratch
COPY solana-vanity-wallet /
ENTRYPOINT ["/solana-vanity-wallet"]
```

## Next Steps

1. **Test current release workflow** by creating a v0.1.0 tag
2. **Set up crates.io token** for automatic publishing
3. **Create package manager integrations** (homebrew, chocolatey)
4. **Add checksums and signatures** for security
5. **Set up update notifications** for users
