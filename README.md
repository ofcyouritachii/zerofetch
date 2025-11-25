<div align="center">

# 🚀 ZeroFetch

**A high-performance, modular, cross-platform system information CLI tool**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Build Status](https://img.shields.io/github/actions/workflow/status/ofcyouritachii/zerofetch/ci.yml)](https://github.com/ofcyouritachii/zerofetch/actions)

*An advanced system information tool inspired by Fastfetch and Neofetch*

[Features](#-features) • [Installation](#-installation) • [Usage](#-usage) • [Configuration](#-configuration) • [Screenshots](#-screenshots)

</div>

---

## 🎯 Overview

**ZeroFetch** is a blazing-fast, modular system information tool written in Rust. It displays your system information alongside beautiful ASCII art logos with extensive customization options.

### Why ZeroFetch?

- ⚡ **Lightning Fast**: Optimized for speed, written in Rust
- 🧩 **Modular Architecture**: Enable/disable individual modules
- 🎨 **Highly Customizable**: Themes, colors, logos, and layouts
- 🌍 **Cross-Platform**: Linux, macOS, Windows, BSD, Android
- 📦 **Easy Installation**: Available for all major package managers
- 🖼️ **Image Protocol Support**: Kitty and iTerm2 graphics
- 📝 **Multiple Config Formats**: JSON, JSONC, YAML, TOML

---

## ✨ Features

### System Information Modules

- **System**: OS, distro, kernel version
- **Hardware**: CPU, GPU, memory, disk
- **Environment**: Shell, terminal, DE, WM
- **Network**: Local IP, network interfaces
- **Packages**: Package counts (apt, pacman, brew, flatpak, snap, etc.)
- **Status**: Uptime, battery, processes
- **And more**: Fully extensible module system

### Display Features

- 🎨 **ASCII Logos**: 50+ built-in logos for major distros and OSes
- 🖼️ **Image Logos**: Kitty/iTerm2 image protocol support
- 🌈 **Color Schemes**: Full color customization
- 📐 **Flexible Layout**: Adjustable padding, spacing, alignment
- 📤 **Multiple Output Formats**: Default, JSON, YAML, TOML, plain text

### Configuration

- 📂 **Config Hierarchy**: CLI → User → System → Defaults
- 📝 **Multiple Formats**: JSONC, YAML, TOML
- 🔧 **CLI Overrides**: Command-line flags override config files
- 🎛️ **Per-Module Control**: Enable/disable/reorder any module

---

## 📦 Installation

### Quick Install

#### Linux
```bash
# Arch Linux
yay -S zerofetch

# Ubuntu/Debian
sudo apt install zerofetch

# Fedora
sudo dnf install zerofetch
```

#### macOS
```bash
brew install zerofetch
```

#### Windows
```powershell
scoop install zerofetch
# or
winget install zerofetch
```

### Other Methods

See [INSTALL.md](INSTALL.md) for comprehensive installation instructions for:
- All Linux distributions (Ubuntu, Debian, Arch, Fedora, Gentoo, Alpine, NixOS, etc.)
- macOS (Homebrew, MacPorts)
- Windows (Scoop, Chocolatey, Winget, MSYS2)
- BSD systems (FreeBSD, OpenBSD, NetBSD, DragonFly)
- Android (Termux)
- Building from source

---

## 🚀 Usage

### Basic Usage

```bash
# Run with auto-detected settings
zerofetch

# Use custom config file
zerofetch --config ~/.config/zerofetch/my-config.jsonc

# Show specific modules only
zerofetch --modules os,kernel,cpu,memory

# Hide specific modules
zerofetch --hide battery,network

# Use a specific logo
zerofetch --logo arch

# Disable logo
zerofetch --logo-type none

# Compact output
zerofetch --compact

# JSON output
zerofetch --format json
```

### Advanced Usage

```bash
# Custom logo file
zerofetch --logo-file ~/my-logo.txt

# Remote logo URL
zerofetch --logo-url https://example.com/logo.png

# Custom separator
zerofetch --separator " → "

# Custom colors
zerofetch --color blue

# Disable colors
zerofetch --no-color

# List available modules
zerofetch --list-modules

# List available logos
zerofetch --list-logos

# Generate default config
zerofetch --gen-config > ~/.config/zerofetch/config.jsonc

# Benchmark mode
zerofetch --benchmark
```

---

## ⚙️ Configuration

### Configuration Files

ZeroFetch looks for configuration files in the following order:

1. **CLI arguments** (highest priority)
2. **User config**: `~/.config/zerofetch/config.jsonc`
3. **System config**: `/etc/zerofetch/config.jsonc` (Linux/BSD)
4. **Defaults** (lowest priority)

### Configuration Format

ZeroFetch supports **JSONC** (JSON with comments), **YAML**, and **TOML** formats:

#### JSONC Example

```jsonc
{
  "general": {
    "separator": ":",
    "compact": false,
    "format": "default"
  },
  "logo": {
    "logo_type": "ascii",
    "padding": 2
  },
  "modules": {
    "order": [
      "os",
      "host",
      "kernel",
      "uptime",
      "packages",
      "shell",
      "terminal",
      "cpu",
      "gpu",
      "memory",
      "disk"
    ],
    "disabled": ["battery"]
  },
  "colors": {
    "title": "cyan",
    "key": "blue",
    "value": "white"
  }
}
```

See [examples/](examples/) for complete configuration templates.

### Available Modules

| Module | Description |
|--------|-------------|
| `os` | Operating system |
| `distro` | Distribution (Linux) or version (macOS/Windows) |
| `host` | Username@hostname |
| `kernel` | Kernel version |
| `uptime` | System uptime |
| `packages` | Package counts (apt, pacman, brew, etc.) |
| `shell` | Current shell |
| `de` | Desktop environment |
| `wm` | Window manager |
| `terminal` | Terminal emulator |
| `cpu` | CPU model and cores |
| `gpu` | GPU model |
| `memory` | RAM usage |
| `disk` | Disk usage |
| `battery` | Battery status and percentage |
| `network` | Local IP address |

---

## 🎨 Supported Logos

ZeroFetch includes ASCII logos for:

### Linux Distributions
- Ubuntu, Debian, Arch, Fedora, Gentoo, Manjaro
- Linux Mint, CentOS, Alpine, NixOS, openSUSE
- Pop!_OS, Kali, Elementary, Solus, Void
- And 30+ more!

### Operating Systems
- macOS
- Windows
- FreeBSD, OpenBSD, NetBSD, DragonFly BSD
- Android

### Custom Logos
- Load from local file: `--logo-file /path/to/logo.txt`
- Load from URL: `--logo-url https://example.com/logo.png`
- Kitty/iTerm2 image protocol support

---

## 📸 Screenshots

### Ubuntu
```
             _                user@hostname
         ---(_)               OS: Ubuntu 22.04 LTS
     _/  ---  \               Kernel: 5.15.0-58-generic
    (_) |   |                 Uptime: 2 hours, 34 mins
      \  --- _/               Packages: 1834 (dpkg), 3 (snap)
         ---(_)               Shell: bash
                              DE: GNOME
                              Terminal: gnome-terminal
                              CPU: Intel i7-9700K (8 cores)
                              GPU: NVIDIA GeForce RTX 2080
                              Memory: 8463 MiB / 16384 MiB
                              Disk: 234 GiB / 500 GiB
```

### Arch Linux
```
                   -`           user@hostname
                  .o+`          OS: Arch Linux
                 `ooo/          Kernel: 6.1.1-arch1-1
                `+oooo:         Uptime: 5 days, 12 hours
               `+oooooo:        Packages: 789 (pacman)
               -+oooooo+:       Shell: zsh
             `/:-:++oooo+:     DE: KDE Plasma
            `/++++/+++++++:    WM: KWin
           `/++++++++++++++:   Terminal: konsole
          `/+++ooooooooooooo/` CPU: AMD Ryzen 9 5900X (24 cores)
         ./ooosssso++osssssso+`GPU: AMD Radeon RX 6800 XT
        .oossssso-````/ossssss+`Memory: 12845 MiB / 32768 MiB
       -osssssso.      :ssssssso.Disk: 1024 GiB / 2048 GiB
```

---

## 🏗️ Architecture

### Project Structure

```
zerofetch/
├── src/
│   ├── main.rs           # Entry point
│   ├── cli.rs            # CLI argument parser
│   ├── config.rs         # Configuration system
│   ├── display.rs        # Output rendering
│   ├── logo/
│   │   ├── mod.rs        # Logo engine
│   │   └── ascii.rs      # ASCII logo database
│   ├── modules/
│   │   ├── mod.rs        # Module system
│   │   ├── os.rs         # OS detection
│   │   ├── cpu.rs        # CPU information
│   │   ├── memory.rs     # Memory information
│   │   └── ...           # Other modules
│   └── utils.rs          # Utility functions
├── examples/
│   ├── config.jsonc      # Example JSONC config
│   ├── config.yaml       # Example YAML config
│   └── config.toml       # Example TOML config
├── Cargo.toml            # Rust dependencies
├── README.md             # This file
└── INSTALL.md            # Installation guide
```

### Module System

Each module is completely independent and implements the `Module` trait:

```rust
pub trait Module: Send + Sync {
    fn name(&self) -> &str;
    fn value(&self) -> Result<String>;
    fn enabled(&self, config: &Config) -> bool;
}
```

This allows for easy addition of new modules without modifying core code.

---

## 🛠️ Development

### Building from Source

```bash
# Clone repository
git clone https://github.com/ofcyouritachii/zerofetch.git
cd zerofetch

# Build debug version
cargo build

# Build release version (optimized)
cargo build --release

# Run
./target/release/zerofetch
```

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_config
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Check for issues
cargo check
```

### Adding New Modules

1. Create new file in `src/modules/` (e.g., `temperature.rs`)
2. Implement the `Module` trait
3. Register in `src/modules/mod.rs`
4. Add to default module order in config

Example:

```rust
// src/modules/temperature.rs
use super::Module;
use anyhow::Result;

pub struct TemperatureModule {
    value: String,
}

impl TemperatureModule {
    pub fn new() -> Result<Self> {
        let value = Self::get_temperature()?;
        Ok(Self { value })
    }

    fn get_temperature() -> Result<String> {
        // Implementation
        Ok("45°C".to_string())
    }
}

impl Module for TemperatureModule {
    fn name(&self) -> &str {
        "Temperature"
    }

    fn value(&self) -> Result<String> {
        Ok(self.value.clone())
    }
}
```

---

## 📝 CLI Reference

```
USAGE:
    zerofetch [OPTIONS]

OPTIONS:
    -c, --config <FILE>           Custom configuration file
    -l, --logo <NAME>             Logo to display
        --logo-type <TYPE>        Logo type (auto, ascii, kitty, iterm, none)
        --color <COLOR>           Color scheme
        --no-color                Disable all colors
    -s, --separator <CHAR>        Separator character
    -m, --modules <LIST>          Show only specific modules (comma-separated)
        --hide <LIST>             Hide specific modules (comma-separated)
    -f, --format <FORMAT>         Output format (default, json, yaml, toml, plain)
        --compact                 Compact mode
        --logo-padding <NUM>      Logo padding
        --logo-file <PATH>        Custom logo file
        --logo-url <URL>          Remote logo URL
        --logo-width <NUM>        Logo width
        --logo-height <NUM>       Logo height
        --list-modules            List all available modules
        --list-logos              List all available logos
        --gen-config              Generate default config file
        --benchmark               Run benchmark mode
    -h, --help                    Print help
    -V, --version                 Print version
```

---

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Commit changes**: `git commit -m 'Add amazing feature'`
4. **Push to branch**: `git push origin feature/amazing-feature`
5. **Open a Pull Request**

### Contribution Ideas

- Add new system information modules
- Add ASCII logos for more distributions
- Improve cross-platform compatibility
- Optimize performance
- Improve documentation
- Report bugs

---

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- Inspired by [Fastfetch](https://github.com/fastfetch-cli/fastfetch) and [Neofetch](https://github.com/dylanaraps/neofetch)
- ASCII logos adapted from various sources in the community
- Built with the amazing Rust ecosystem

---

## 📧 Support

- **Issues**: [GitHub Issues](https://github.com/ofcyouritachii/zerofetch/issues)
- **Discussions**: [GitHub Discussions](https://github.com/ofcyouritachii/zerofetch/discussions)
- **Documentation**: [Wiki](https://github.com/ofcyouritachii/zerofetch/wiki)

---

<div align="center">

Made with ❤️ and 🦀 Rust

⭐ Star us on GitHub if you find this useful!

</div>
