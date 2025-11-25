# Publishing Guide for ZeroFetch

This guide explains how to build and publish ZeroFetch packages for various Linux distributions.

## Prerequisites

- Git
- Rust 1.70+ and Cargo
- Distribution-specific build tools

---

## 🦀 Publish to crates.io

### 1. Prepare for Publishing

```bash
# Ensure you're logged in to crates.io
cargo login

# Verify the package builds correctly
cargo build --release
cargo test
cargo doc

# Check for any issues
cargo clippy -- -D warnings
cargo fmt --check

# Do a dry run
cargo publish --dry-run
```

### 2. Publish

```bash
# Publish to crates.io
cargo publish
```

### 3. Install from crates.io

Users can then install with:
```bash
cargo install zerofetch
```

---

## 🏗️ Arch Linux (AUR)

### 1. Build Package

```bash
# Copy PKGBUILD to a clean directory
mkdir -p ~/zerofetch-aur
cp PKGBUILD ~/zerofetch-aur/

cd ~/zerofetch-aur

# Update checksums
updpkgsums

# Build and test package
makepkg -si
```

### 2. Publish to AUR

```bash
# Clone AUR repository (first time only)
git clone ssh://aur@aur.archlinux.org/zerofetch.git aur-zerofetch
cd aur-zerofetch

# Copy files
cp ~/zerofetch-aur/PKGBUILD .
cp ~/zerofetch-aur/.SRCINFO .

# Generate .SRCINFO
makepkg --printsrcinfo > .SRCINFO

# Commit and push
git add PKGBUILD .SRCINFO
git commit -m "Initial release: zerofetch 0.1.0"
git push
```

### 3. Install from AUR

Users can install with:
```bash
yay -S zerofetch
# or
paru -S zerofetch
```

---

## 📦 Ubuntu/Debian (.deb)

### 1. Install Build Dependencies

```bash
sudo apt install debhelper devscripts build-essential cargo rustc
```

### 2. Build Package

```bash
# From project root
dpkg-buildpackage -us -uc -b

# Package will be created in parent directory
ls -la ../zerofetch_*.deb
```

### 3. Test Package

```bash
# Install locally
sudo dpkg -i ../zerofetch_0.1.0-1_amd64.deb

# Check installation
zerofetch --version

# Uninstall
sudo apt remove zerofetch
```

### 4. Publish to PPA (Ubuntu)

```bash
# Create Launchpad account and GPG key
# Upload to PPA
dput ppa:your-username/zerofetch ../zerofetch_0.1.0-1_source.changes
```

### 5. Publish to Debian Repository

Submit package to Debian mentors:
https://mentors.debian.net/

---

## 🎩 Fedora/RHEL (.rpm)

### 1. Install Build Dependencies

```bash
sudo dnf install rpm-build rpmdevtools cargo rust
```

### 2. Setup RPM Build Environment

```bash
rpmdev-setuptree
```

### 3. Build Package

```bash
# Copy spec file
cp zerofetch.spec ~/rpmbuild/SPECS/

# Download source tarball
cd ~/rpmbuild/SOURCES
wget https://github.com/ofcyouritachii/zerofetch/archive/v0.1.0.tar.gz

# Build RPM
cd ~/rpmbuild/SPECS
rpmbuild -ba zerofetch.spec

# Package will be in ~/rpmbuild/RPMS/x86_64/
```

### 4. Test Package

```bash
# Install locally
sudo dnf install ~/rpmbuild/RPMS/x86_64/zerofetch-0.1.0-1.*.rpm

# Check installation
zerofetch --version

# Uninstall
sudo dnf remove zerofetch
```

### 5. Publish to Copr (Fedora)

```bash
# Create Copr account
# Install copr-cli
sudo dnf install copr-cli

# Create new project
copr-cli create zerofetch --chroot fedora-39-x86_64 --chroot fedora-40-x86_64

# Build package
copr-cli build zerofetch ~/rpmbuild/SRPMS/zerofetch-0.1.0-1.*.src.rpm
```

---

## 🐧 Generic Linux (Binary Release)

### 1. Build Portable Binary

```bash
# Build static binary
cargo build --release --target x86_64-unknown-linux-musl

# Or use regular build
cargo build --release
```

### 2. Create Release Archive

```bash
# Create release directory
mkdir -p zerofetch-0.1.0-linux-x86_64
cd zerofetch-0.1.0-linux-x86_64

# Copy files
cp ../target/release/zerofetch .
cp ../LICENSE .
cp ../README.md .
mkdir examples
cp ../examples/*.jsonc examples/
cp ../examples/*.yaml examples/
cp ../examples/*.toml examples/

# Create tarball
cd ..
tar czf zerofetch-0.1.0-linux-x86_64.tar.gz zerofetch-0.1.0-linux-x86_64/

# Create checksum
sha256sum zerofetch-0.1.0-linux-x86_64.tar.gz > zerofetch-0.1.0-linux-x86_64.tar.gz.sha256
```

### 3. Create GitHub Release

```bash
# Tag the release
git tag -a v0.1.0 -m "Release version 0.1.0"
git push origin v0.1.0

# Upload to GitHub Releases
# Use GitHub web interface or gh CLI
gh release create v0.1.0 \
  zerofetch-0.1.0-linux-x86_64.tar.gz \
  zerofetch-0.1.0-linux-x86_64.tar.gz.sha256 \
  --title "ZeroFetch v0.1.0" \
  --notes "Initial release"
```

---

## 🚀 Quick Install Script

Users can install with a single command:

```bash
curl -fsSL https://raw.githubusercontent.com/ofcyouritachii/zerofetch/main/install.sh | bash
```

Or download and run:

```bash
wget https://raw.githubusercontent.com/ofcyouritachii/zerofetch/main/install.sh
chmod +x install.sh
./install.sh
```

---

## 📋 Pre-Publishing Checklist

- [ ] Update version in Cargo.toml
- [ ] Update version in PKGBUILD
- [ ] Update version in debian/changelog
- [ ] Update version in zerofetch.spec
- [ ] Update README.md with new features
- [ ] Run all tests: `cargo test`
- [ ] Check code quality: `cargo clippy`
- [ ] Format code: `cargo fmt`
- [ ] Build release binary: `cargo build --release`
- [ ] Test on multiple distributions (Ubuntu, Arch, Fedora)
- [ ] Update CHANGELOG.md
- [ ] Create git tag
- [ ] Build packages for all distributions
- [ ] Upload to GitHub Releases
- [ ] Publish to crates.io
- [ ] Submit to AUR
- [ ] Submit to distribution repositories

---

## 🎯 Distribution-Specific Installation

Once published, users can install using:

### Arch Linux
```bash
yay -S zerofetch
```

### Ubuntu/Debian
```bash
sudo add-apt-repository ppa:zerofetch/zerofetch
sudo apt update
sudo apt install zerofetch
```

### Fedora
```bash
sudo dnf copr enable zerofetch/zerofetch
sudo dnf install zerofetch
```

### From Source
```bash
cargo install zerofetch
```

### Binary Release
```bash
curl -fsSL https://raw.githubusercontent.com/ofcyouritachii/zerofetch/main/install.sh | bash
```

---

## 📞 Support

For publishing issues:
- GitHub Issues: https://github.com/ofcyouritachii/zerofetch/issues
- Discussions: https://github.com/ofcyouritachii/zerofetch/discussions
