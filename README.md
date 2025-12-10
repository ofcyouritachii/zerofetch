# Zerofetch

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Bash](https://img.shields.io/badge/Bash-4.0%2B-green.svg)](https://www.gnu.org/software/bash/)
[![Platform](https://img.shields.io/badge/Platform-Linux-orange.svg)](https://www.linux.org/)

A lightweight, fast system information tool for Linux written in pure Bash. Zero dependencies, highly customizable, with beautiful ASCII art logos for 50+ distributions.

## Features

- **Zero Dependencies** - Pure Bash implementation
- **Fast Execution** - Optimized performance with minimal overhead
- **Customizable** - Extensive configuration options
- **Cross-Distribution** - Works on any Linux distribution
- **30+ Info Fields** - Comprehensive system information
- **Beautiful Output** - ASCII art logos with color customization

## Installation

**Requirements:** Bash 4.0+

### Quick Install

```bash
git clone https://github.com/ofcyouritachii/zerofetch.git
cd zerofetch
sudo make install
```

### Manual Install

```bash
sudo cp zerofetch /usr/local/bin/
sudo chmod +x /usr/local/bin/zerofetch
```

### User Install (No Root)

```bash
mkdir -p ~/.local/bin
cp zerofetch ~/.local/bin/
chmod +x ~/.local/bin/zerofetch
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
```

## Usage

```bash
zerofetch              # Display system information
zerofetch --logo arch  # Use specific distribution logo
zerofetch --off        # Text-only mode
zerofetch --help       # Show all options
```

## Configuration

Config file locations (in priority order):
1. `--config <file>` flag
2. `~/.config/zerofetch/config`
3. `/etc/zerofetch/config`

### Example Configuration

```bash
ascii_distro="arch"
separator=":"
cpu_temp="on"
cpu_speed="on"
memory_percent="on"
disk_show=('/' '/home')
gpu_type="all"
shell_version="on"
```

See [zerofetch.conf](zerofetch.conf) for all options.

## Command-Line Options

| Option | Description |
|--------|-------------|
| `--config <file>` | Use custom config file |
| `--logo <distro>` | Override distribution logo |
| `--off` | Text-only mode (no ASCII) |
| `--cpu_temp on/off` | Show CPU temperature |
| `--memory_percent on/off` | Show memory usage % |
| `--disk_percent on/off` | Show disk usage % |
| `--version`, `-v` | Show version |
| `--help`, `-h` | Display help |

Run `zerofetch --help` for complete list.

## Supported Distributions

Arch, Ubuntu, Debian, Fedora, Gentoo, Manjaro, Linux Mint, Pop!_OS, openSUSE, Void Linux, Alpine, Kali, and 50+ more.

## Contributing

1. Fork the repository
2. Create feature branch (`git checkout -b feature/improvement`)
3. Commit changes (`git commit -am 'Add feature'`)
4. Push to branch (`git push origin feature/improvement`)
5. Open Pull Request

**Bug Reports:** [Issues](https://github.com/ofcyouritachii/zerofetch/issues)

## License

MIT License - see [LICENSE](LICENSE) file.

## Author

**ofcyouritachii** - [GitHub](https://github.com/ofcyouritachii)