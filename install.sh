#!/bin/bash
# Install script for Solana Vanity Wallet Generator
# Usage: curl -fsSL https://raw.githubusercontent.com/ljacob/solana-vanity-wallet/master/install.sh | sh

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Repository info
REPO="ljacob/solana-vanity-wallet"
BINARY_NAME="solana-vanity-wallet"

print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Detect OS and architecture
detect_platform() {
    local os=$(uname -s)
    local arch=$(uname -m)
    
    case $os in
        Linux*)
            case $arch in
                x86_64) echo "linux-x64" ;;
                *) print_error "Unsupported architecture: $arch"; exit 1 ;;
            esac
            ;;
        Darwin*)
            case $arch in
                x86_64) echo "macos-x64" ;;
                arm64) echo "macos-arm64" ;;
                *) print_error "Unsupported architecture: $arch"; exit 1 ;;
            esac
            ;;
        *)
            print_error "Unsupported operating system: $os"
            exit 1
            ;;
    esac
}

# Get latest release info
get_latest_release() {
    curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/'
}

# Download and install
install_binary() {
    local platform=$(detect_platform)
    local version=$(get_latest_release)
    
    if [ -z "$version" ]; then
        print_error "Failed to get latest release version"
        exit 1
    fi
    
    print_info "Installing Solana Vanity Wallet Generator $version for $platform..."
    
    local download_url="https://github.com/$REPO/releases/download/$version/$BINARY_NAME-$platform.tar.gz"
    local temp_dir=$(mktemp -d)
    local archive_path="$temp_dir/$BINARY_NAME-$platform.tar.gz"
    
    print_info "Downloading from $download_url..."
    
    # Download the archive
    if ! curl -fsSL "$download_url" -o "$archive_path"; then
        print_error "Failed to download binary"
        rm -rf "$temp_dir"
        exit 1
    fi
    
    # Extract the archive
    print_info "Extracting binary..."
    if ! tar -xzf "$archive_path" -C "$temp_dir"; then
        print_error "Failed to extract archive"
        rm -rf "$temp_dir"
        exit 1
    fi
    
    # Install to /usr/local/bin
    local install_dir="/usr/local/bin"
    local binary_path="$temp_dir/$BINARY_NAME"
    
    if [ ! -f "$binary_path" ]; then
        print_error "Binary not found in archive"
        rm -rf "$temp_dir"
        exit 1
    fi
    
    print_info "Installing to $install_dir..."
    
    # Check if we need sudo
    if [ -w "$install_dir" ]; then
        mv "$binary_path" "$install_dir/$BINARY_NAME"
    else
        print_warning "Installing to $install_dir requires sudo permissions"
        sudo mv "$binary_path" "$install_dir/$BINARY_NAME"
        sudo chmod +x "$install_dir/$BINARY_NAME"
    fi
    
    # Clean up
    rm -rf "$temp_dir"
    
    print_success "Successfully installed $BINARY_NAME $version"
    print_info "You can now run: $BINARY_NAME --help"
}

# Main installation
main() {
    echo -e "${BLUE}🚀 Solana Vanity Wallet Generator Installer${NC}"
    echo ""
    
    # Check prerequisites
    if ! command -v curl >/dev/null 2>&1; then
        print_error "curl is required but not installed"
        exit 1
    fi
    
    if ! command -v tar >/dev/null 2>&1; then
        print_error "tar is required but not installed"
        exit 1
    fi
    
    # Install
    install_binary
    
    echo ""
    echo -e "${GREEN}🎉 Installation complete!${NC}"
    echo ""
    echo "Quick start:"
    echo "  $BINARY_NAME --help"
    echo "  $BINARY_NAME ABC"
    echo "  $BINARY_NAME Sol --with-mnemonic"
    echo ""
    echo "Documentation: https://github.com/$REPO"
}

main "$@"
