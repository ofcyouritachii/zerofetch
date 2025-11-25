#!/usr/bin/env bash
# Build script for ZeroFetch

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}=== ZeroFetch Build Script ===${NC}"

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Cargo/Rust not found${NC}"
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

# Parse arguments
BUILD_TYPE="release"
INSTALL=false
CLEAN=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --debug)
            BUILD_TYPE="debug"
            shift
            ;;
        --install)
            INSTALL=true
            shift
            ;;
        --clean)
            CLEAN=true
            shift
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            exit 1
            ;;
    esac
done

# Clean if requested
if [ "$CLEAN" = true ]; then
    echo -e "${YELLOW}Cleaning build artifacts...${NC}"
    cargo clean
fi

# Build
echo -e "${YELLOW}Building ZeroFetch ($BUILD_TYPE)...${NC}"
if [ "$BUILD_TYPE" = "release" ]; then
    cargo build --release
else
    cargo build
fi

echo -e "${GREEN}Build successful!${NC}"

# Install if requested
if [ "$INSTALL" = true ]; then
    echo -e "${YELLOW}Installing ZeroFetch...${NC}"
    
    if [ "$BUILD_TYPE" = "release" ]; then
        BINARY_PATH="target/release/zerofetch"
    else
        BINARY_PATH="target/debug/zerofetch"
    fi
    
    # Determine install location
    if [ -w "/usr/local/bin" ]; then
        cp "$BINARY_PATH" /usr/local/bin/zerofetch
        echo -e "${GREEN}Installed to /usr/local/bin/zerofetch${NC}"
    else
        mkdir -p "$HOME/.local/bin"
        cp "$BINARY_PATH" "$HOME/.local/bin/zerofetch"
        echo -e "${GREEN}Installed to $HOME/.local/bin/zerofetch${NC}"
        echo -e "${YELLOW}Make sure $HOME/.local/bin is in your PATH${NC}"
    fi
    
    # Create config directory
    mkdir -p "$HOME/.config/zerofetch"
    
    echo -e "${GREEN}Installation complete!${NC}"
fi

echo ""
echo -e "${GREEN}To run ZeroFetch:${NC}"
if [ "$INSTALL" = true ]; then
    echo "  zerofetch"
else
    echo "  ./$BINARY_PATH"
fi
