# Makefile for ZeroFetch
# Provides convenient shortcuts for common operations

.PHONY: all build release install clean test fmt clippy check help run

# Default target
all: build

# Build debug version
build:
	@echo "Building debug version..."
	cargo build

# Build optimized release version
release:
	@echo "Building release version..."
	cargo build --release

# Install to system (requires sudo on Linux/macOS)
install: release
	@echo "Installing zerofetch..."
ifeq ($(OS),Windows_NT)
	@echo "Please manually copy target/release/zerofetch.exe to a directory in your PATH"
else
	sudo cp target/release/zerofetch /usr/local/bin/
	@echo "Installed to /usr/local/bin/zerofetch"
endif

# Install for current user only
install-user: release
	@echo "Installing zerofetch for current user..."
	mkdir -p ~/.local/bin
	cp target/release/zerofetch ~/.local/bin/
	@echo "Installed to ~/.local/bin/zerofetch"
	@echo "Make sure ~/.local/bin is in your PATH"

# Uninstall from system
uninstall:
	@echo "Uninstalling zerofetch..."
ifeq ($(OS),Windows_NT)
	@echo "Please manually remove zerofetch.exe from your PATH"
else
	sudo rm -f /usr/local/bin/zerofetch
	@echo "Uninstalled from /usr/local/bin"
endif

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean

# Run tests
test:
	@echo "Running tests..."
	cargo test

# Run tests with output
test-verbose:
	@echo "Running tests with output..."
	cargo test -- --nocapture

# Format code
fmt:
	@echo "Formatting code..."
	cargo fmt

# Check formatting
fmt-check:
	@echo "Checking code formatting..."
	cargo fmt -- --check

# Run clippy linter
clippy:
	@echo "Running clippy..."
	cargo clippy -- -D warnings

# Check code without building
check:
	@echo "Checking code..."
	cargo check

# Run the program
run:
	cargo run

# Run with custom arguments
run-args:
	cargo run -- $(ARGS)

# Build documentation
doc:
	@echo "Building documentation..."
	cargo doc --no-deps --open

# Build for all targets (requires cross)
build-all:
	@echo "Building for all platforms..."
	cargo build --release --target x86_64-unknown-linux-gnu
	cargo build --release --target x86_64-pc-windows-gnu
	cargo build --release --target x86_64-apple-darwin

# Create distribution packages
dist: release
	@echo "Creating distribution packages..."
	mkdir -p dist
	tar -czf dist/zerofetch-linux-x86_64.tar.gz -C target/release zerofetch
	@echo "Distribution packages created in dist/"

# Install development dependencies
dev-setup:
	@echo "Installing development dependencies..."
	rustup component add rustfmt clippy
	cargo install cargo-watch cargo-audit

# Watch for changes and rebuild
watch:
	cargo watch -x build

# Watch and run tests on changes
watch-test:
	cargo watch -x test

# Security audit
audit:
	@echo "Running security audit..."
	cargo audit

# Update dependencies
update:
	@echo "Updating dependencies..."
	cargo update

# Check outdated dependencies
outdated:
	@echo "Checking for outdated dependencies..."
	cargo outdated

# Benchmark
bench:
	cargo run --release -- --benchmark

# Generate default config
gen-config:
	cargo run -- --gen-config

# Help
help:
	@echo "ZeroFetch Makefile Commands:"
	@echo ""
	@echo "  make build          - Build debug version"
	@echo "  make release        - Build optimized release version"
	@echo "  make install        - Install to /usr/local/bin (requires sudo)"
	@echo "  make install-user   - Install to ~/.local/bin"
	@echo "  make uninstall      - Remove from system"
	@echo "  make clean          - Remove build artifacts"
	@echo "  make test           - Run tests"
	@echo "  make test-verbose   - Run tests with output"
	@echo "  make fmt            - Format code"
	@echo "  make fmt-check      - Check code formatting"
	@echo "  make clippy         - Run clippy linter"
	@echo "  make check          - Check code without building"
	@echo "  make run            - Run the program"
	@echo "  make run-args ARGS='--help' - Run with arguments"
	@echo "  make doc            - Build and open documentation"
	@echo "  make dist           - Create distribution packages"
	@echo "  make dev-setup      - Install development tools"
	@echo "  make watch          - Watch and rebuild on changes"
	@echo "  make watch-test     - Watch and test on changes"
	@echo "  make audit          - Security audit"
	@echo "  make update         - Update dependencies"
	@echo "  make bench          - Run benchmarks"
	@echo "  make gen-config     - Generate default config"
	@echo "  make help           - Show this help message"
