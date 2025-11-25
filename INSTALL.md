# 📦 Installation Guide

## Table of Contents
- [Linux](#linux)
- [macOS](#macos)
- [Windows](#windows)
- [BSD](#bsd)
- [Android (Termux)](#android-termux)
- [Build from Source](#build-from-source)

---

## Linux

### Ubuntu / Debian

#### Via PPA (Recommended)
```bash
# Add PPA (when available)
sudo add-apt-repository ppa:zerofetch/zerofetch
sudo apt update
sudo apt install zerofetch
```

#### Via .deb Package
```bash
# Download from releases
wget https://github.com/ofcyouritachii/zerofetch/releases/latest/download/zerofetch_amd64.deb
sudo dpkg -i zerofetch_amd64.deb
```

### Arch Linux

```bash
# Via AUR
yay -S zerofetch

# Or using paru
paru -S zerofetch

# Or manually
git clone https://aur.archlinux.org/zerofetch.git
cd zerofetch
makepkg -si
```

### Fedora / RHEL / CentOS

```bash
# Via DNF
sudo dnf install zerofetch

# Or via Copr
sudo dnf copr enable zerofetch/zerofetch
sudo dnf install zerofetch
```

### openSUSE

```bash
# Via Zypper
sudo zypper install zerofetch

# Or add repository
sudo zypper ar https://download.opensuse.org/repositories/zerofetch/openSUSE_Tumbleweed/ zerofetch
sudo zypper ref
sudo zypper in zerofetch
```

### Gentoo

```bash
# Via Portage
sudo emerge --ask app-misc/zerofetch
```

### Alpine Linux

```bash
# Via APK
apk add --upgrade zerofetch
```

### NixOS / Nix

```bash
# Temporary shell
nix-shell -p zerofetch

# System-wide (add to configuration.nix)
environment.systemPackages = [ pkgs.zerofetch ];

# User environment
nix-env -iA nixpkgs.zerofetch
```

### Void Linux

```bash
sudo xbps-install -S zerofetch
```

### Solus

```bash
sudo eopkg install zerofetch
```

### Slackware

```bash
# Via sbopkg
sbopkg -i zerofetch

# Or via slackbuilds
wget https://slackbuilds.org/slackbuilds/zerofetch.tar.gz
tar xvzf zerofetch.tar.gz
cd zerofetch
./zerofetch.SlackBuild
installpkg /tmp/zerofetch-*.tgz
```

### ALT Linux

```bash
apt-get install zerofetch
```

### Linuxbrew

```bash
brew install zerofetch
```

---

## macOS

### Homebrew (Recommended)

```bash
brew install zerofetch
```

### MacPorts

```bash
sudo port install zerofetch
```

### Build from Source

```bash
# Install Rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/ofcyouritachii/zerofetch.git
cd zerofetch
cargo build --release
sudo cp target/release/zerofetch /usr/local/bin/
```

---

## Windows

### Scoop (Recommended)

```powershell
scoop install zerofetch
```

### Chocolatey

```powershell
choco install zerofetch
```

### Winget

```powershell
winget install zerofetch
```

### MSYS2

```bash
# For 64-bit
pacman -S mingw-w64-x86_64-zerofetch

# For 32-bit
pacman -S mingw-w64-i686-zerofetch
```

### Manual Installation

1. Download the latest `.exe` from [releases](https://github.com/ofcyouritachii/zerofetch/releases)
2. Extract to a directory (e.g., `C:\Program Files\zerofetch\`)
3. Add to PATH:
   - Right-click "This PC" → Properties → Advanced System Settings
   - Environment Variables → System Variables → Path → Edit
   - Add `C:\Program Files\zerofetch\`

---

## BSD

### FreeBSD

```bash
# Via pkg
pkg install zerofetch

# Via ports
cd /usr/ports/sysutils/zerofetch
make install clean
```

### OpenBSD

```bash
pkg_add zerofetch
```

### NetBSD

```bash
pkgin install zerofetch
```

### DragonFly BSD

```bash
pkg install zerofetch
```

---

## Android (Termux)

```bash
# Update package list
pkg update

# Install zerofetch
pkg install zerofetch
```

---

## Build from Source

### Prerequisites

- **Rust** 1.70 or later
- **Cargo** (comes with Rust)

### Installation Steps

```bash
# 1. Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Clone the repository
git clone https://github.com/ofcyouritachii/zerofetch.git
cd zerofetch

# 3. Build the project
cargo build --release

# 4. Install (Linux/macOS)
sudo cp target/release/zerofetch /usr/local/bin/

# Or install via cargo
cargo install --path .

# Windows: Copy to a directory in your PATH
copy target\release\zerofetch.exe C:\Windows\System32\
```

### Build Options

```bash
# Debug build (faster compilation, slower runtime)
cargo build

# Release build (optimized)
cargo build --release

# Install system-wide
cargo install --path . --root /usr/local
```

---

## Verify Installation

```bash
# Check version
zerofetch --version

# Run zerofetch
zerofetch
```

---

## Configuration

After installation, create a config file:

```bash
# Generate default config
zerofetch --gen-config > ~/.config/zerofetch/config.jsonc

# Or manually create
mkdir -p ~/.config/zerofetch
nano ~/.config/zerofetch/config.jsonc
```

See [examples/](../examples/) for configuration templates.

---

## Uninstallation

### Package Managers

```bash
# Ubuntu/Debian
sudo apt remove zerofetch

# Arch Linux
sudo pacman -R zerofetch

# Fedora
sudo dnf remove zerofetch

# macOS (Homebrew)
brew uninstall zerofetch

# Windows (Scoop)
scoop uninstall zerofetch
```

### Manual Installation

```bash
# Linux/macOS
sudo rm /usr/local/bin/zerofetch
rm -rf ~/.config/zerofetch

# Windows
del C:\Windows\System32\zerofetch.exe
```

---

## Troubleshooting

### Command not found

Ensure the binary is in your PATH:

```bash
# Check PATH
echo $PATH

# Add to PATH (Linux/macOS - add to ~/.bashrc or ~/.zshrc)
export PATH="$PATH:/usr/local/bin"
```

### Permission denied

```bash
# Make executable
chmod +x /usr/local/bin/zerofetch
```

### Build errors

```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

---

For more help, visit: https://github.com/ofcyouritachii/zerofetch/issues
