#!/usr/bin/env bash

# ZeroFetch Installation Script for Linux
# Supports: Ubuntu, Debian, Arch, Fedora, and other major distributions

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print functions
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if running on Linux
check_os() {
    if [[ "$(uname -s)" != "Linux" ]]; then
        print_error "This script is designed for Linux systems only."
        exit 1
    fi
    print_success "Running on Linux"
}

# Detect Linux distribution
detect_distro() {
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        DISTRO=$ID
        DISTRO_VERSION=$VERSION_ID
    else
        print_error "Cannot detect Linux distribution"
        exit 1
    fi
    print_info "Detected: $NAME"
}

# Check if Rust is installed
check_rust() {
    if command -v cargo &> /dev/null; then
        RUST_VERSION=$(cargo --version | awk '{print $2}')
        print_success "Rust $RUST_VERSION is already installed"
        return 0
    else
        print_warning "Rust is not installed"
        return 1
    fi
}

# Install Rust
install_rust() {
    print_info "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    
    # Source cargo environment
    source "$HOME/.cargo/env"
    
    print_success "Rust installed successfully"
}

# Build ZeroFetch
build_zerofetch() {
    print_info "Building ZeroFetch..."
    
    cargo build --release
    
    if [ -f "target/release/zerofetch" ]; then
        print_success "ZeroFetch built successfully"
    else
        print_error "Build failed"
        exit 1
    fi
}

# Install ZeroFetch
install_zerofetch() {
    print_info "Installing ZeroFetch..."
    
    # Ask for installation type
    echo ""
    echo "Select installation type:"
    echo "1) System-wide installation (requires sudo) - /usr/local/bin"
    echo "2) User installation - ~/.local/bin"
    echo ""
    read -p "Enter choice [1-2]: " install_choice
    
    case $install_choice in
        1)
            sudo cp target/release/zerofetch /usr/local/bin/
            sudo chmod +x /usr/local/bin/zerofetch
            print_success "ZeroFetch installed to /usr/local/bin/zerofetch"
            ;;
        2)
            mkdir -p "$HOME/.local/bin"
            cp target/release/zerofetch "$HOME/.local/bin/"
            chmod +x "$HOME/.local/bin/zerofetch"
            print_success "ZeroFetch installed to $HOME/.local/bin/zerofetch"
            
            # Check if ~/.local/bin is in PATH
            if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
                print_warning "$HOME/.local/bin is not in your PATH"
                print_info "Add this line to your ~/.bashrc or ~/.zshrc:"
                echo ""
                echo "export PATH=\"\$PATH:\$HOME/.local/bin\""
                echo ""
            fi
            ;;
        *)
            print_error "Invalid choice"
            exit 1
            ;;
    esac
}

# Create default config directory
setup_config() {
    CONFIG_DIR="$HOME/.config/zerofetch"
    
    if [ ! -d "$CONFIG_DIR" ]; then
        print_info "Creating config directory: $CONFIG_DIR"
        mkdir -p "$CONFIG_DIR"
    fi
    
    if [ ! -f "$CONFIG_DIR/config.jsonc" ] && [ -f "examples/config.jsonc" ]; then
        print_info "Installing default configuration"
        cp examples/config.jsonc "$CONFIG_DIR/"
        print_success "Default config installed to $CONFIG_DIR/config.jsonc"
    fi
}

# Main installation process
main() {
    echo ""
    echo "======================================"
    echo "  ZeroFetch Installation Script"
    echo "======================================"
    echo ""
    
    # Check OS
    check_os
    
    # Detect distribution
    detect_distro
    
    # Check and install Rust if needed
    if ! check_rust; then
        read -p "Would you like to install Rust now? (y/n): " install_rust_choice
        if [[ "$install_rust_choice" =~ ^[Yy]$ ]]; then
            install_rust
        else
            print_error "Rust is required to build ZeroFetch"
            print_info "Install Rust manually from: https://rustup.rs"
            exit 1
        fi
    fi
    
    # Build ZeroFetch
    build_zerofetch
    
    # Install ZeroFetch
    install_zerofetch
    
    # Setup config
    setup_config
    
    echo ""
    print_success "Installation complete!"
    echo ""
    print_info "Run 'zerofetch' to get started"
    print_info "Run 'zerofetch --help' for usage information"
    echo ""
}

# Run main installation
main
