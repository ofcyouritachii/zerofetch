#!/usr/bin/env bash
#
# Quick installer script for Zerofetch
# This script provides an easy way to install Zerofetch without make

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default paths
PREFIX="${PREFIX:-/usr/local}"
BINDIR="$PREFIX/bin"
MANDIR="$PREFIX/share/man/man1"
CONFDIR="$HOME/.config/zerofetch"

echo -e "${BLUE}Zerofetch Installer${NC}"
echo "===================="
echo ""

# Check if running as root for system-wide install
if [[ "$BINDIR" == /usr/* ]] || [[ "$BINDIR" == /opt/* ]]; then
    if [[ $EUID -ne 0 ]]; then
        echo -e "${RED}Error: System-wide installation requires root privileges${NC}"
        echo "Please run with sudo or set PREFIX to a user directory:"
        echo "  sudo ./install.sh"
        echo "  OR"
        echo "  PREFIX=~/.local ./install.sh"
        exit 1
    fi
fi

# Install dependencies
echo "Installing required dependencies..."
if command -v pacman >/dev/null 2>&1; then
    pacman -S --noconfirm --needed pciutils xorg-xrandr lm_sensors playerctl 2>/dev/null || true
elif command -v apt >/dev/null 2>&1; then
    apt install -y pciutils x11-xserver-utils lm-sensors playerctl 2>/dev/null || true
elif command -v dnf >/dev/null 2>&1; then
    dnf install -y pciutils xrandr lm_sensors playerctl 2>/dev/null || true
elif command -v zypper >/dev/null 2>&1; then
    zypper install -y pciutils xrandr sensors playerctl 2>/dev/null || true
fi

# Create directories
echo "Creating directories..."
mkdir -p "$BINDIR"
mkdir -p "$MANDIR"
mkdir -p "$CONFDIR"

# Install binary
echo "Installing zerofetch to $BINDIR..."
install -m 755 zerofetch "$BINDIR/zerofetch"

# Install man page
if [[ -f zerofetch.1 ]]; then
    echo "Installing man page to $MANDIR..."
    install -m 644 zerofetch.1 "$MANDIR/zerofetch.1"
    if command -v gzip >/dev/null 2>&1; then
        gzip -f "$MANDIR/zerofetch.1"
    fi
fi

# Install config file
if [[ -f config.conf ]]; then
    if [[ ! -f "$CONFDIR/config.conf" ]]; then
        echo "Installing example config to $CONFDIR..."
        install -m 644 config.conf "$CONFDIR/config.conf"
    else
        echo "Config file already exists at $CONFDIR/config.conf, skipping..."
    fi
fi

echo ""
echo -e "${GREEN}✓ Zerofetch installed successfully!${NC}"
echo ""
echo "Installation paths:"
echo "  Binary:  $BINDIR/zerofetch"
echo "  Man:     $MANDIR/zerofetch.1.gz"
echo "  Config:  $CONFDIR/config.conf"
echo ""
echo "You can now run: zerofetch"
echo ""

# Check if binary is in PATH
if ! command -v zerofetch >/dev/null 2>&1; then
    echo -e "${RED}Warning: $BINDIR is not in your PATH${NC}"
    echo "Add this to your ~/.bashrc or ~/.zshrc:"
    echo "  export PATH=\"$BINDIR:\$PATH\""
fi
