#!/usr/bin/env bash
# Installation script for ZeroFetch

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}"
cat << "EOF"
 ______           ______    _       _     
|___  /          |  ____|  | |     | |    
   / / ___ _ __ | |__ ___ | |_ ___| |__  
  / / / _ \ '__||  __/ _ \| __/ __| '_ \ 
 / /_|  __/ |   | | |  __/| || (__| | | |
/_____\___|_|   |_|  \___| \__\___|_| |_|
                                          
EOF
echo -e "${NC}"

echo -e "${GREEN}ZeroFetch Installation Script${NC}"
echo ""

# Check if running as root
if [ "$EUID" -eq 0 ]; then 
    echo -e "${YELLOW}Warning: Running as root${NC}"
    INSTALL_DIR="/usr/local/bin"
    NEED_SUDO=false
else
    INSTALL_DIR="$HOME/.local/bin"
    NEED_SUDO=false
fi

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${YELLOW}Rust not found. Installing Rust...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

echo -e "${YELLOW}Building ZeroFetch...${NC}"
cargo build --release

echo -e "${YELLOW}Installing ZeroFetch to $INSTALL_DIR...${NC}"
mkdir -p "$INSTALL_DIR"
cp target/release/zerofetch "$INSTALL_DIR/"

# Make executable
chmod +x "$INSTALL_DIR/zerofetch"

# Create config directory
CONFIG_DIR="$HOME/.config/zerofetch"
mkdir -p "$CONFIG_DIR"

echo -e "${GREEN}Installation complete!${NC}"
echo ""
echo -e "${YELLOW}Configuration:${NC}"
echo "  Config directory: $CONFIG_DIR"
echo "  To create a config: zerofetch --gen-config > $CONFIG_DIR/config.jsonc"
echo ""
echo -e "${YELLOW}Usage:${NC}"
echo "  zerofetch"
echo "  zerofetch --help"
echo ""

# Check if install dir is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo -e "${YELLOW}Note: $INSTALL_DIR is not in your PATH${NC}"
    echo "Add this line to your ~/.bashrc or ~/.zshrc:"
    echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
fi

echo -e "${GREEN}Enjoy ZeroFetch! 🚀${NC}"
