# Contributing to ZeroFetch

Thank you for your interest in contributing to ZeroFetch! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Submitting Changes](#submitting-changes)
- [Coding Standards](#coding-standards)
- [Adding New Modules](#adding-new-modules)
- [Testing](#testing)
- [Documentation](#documentation)

---

## Code of Conduct

By participating in this project, you agree to abide by our Code of Conduct:

- Be respectful and inclusive
- Welcome newcomers and help them learn
- Focus on what is best for the community
- Show empathy towards other community members

---

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/zerofetch.git
   cd zerofetch
   ```
3. **Add upstream remote**:
   ```bash
   git remote add upstream https://github.com/ofcyouritachii/zerofetch.git
   ```

---

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Git
- A code editor (VS Code, Vim, etc.)

### Setup Steps

```bash
# Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install development tools
rustup component add rustfmt clippy

# Build the project
cargo build

# Run tests
cargo test

# Run the program
cargo run
```

---

## Making Changes

### Branch Naming

Use descriptive branch names:
- `feature/add-temperature-module`
- `fix/memory-leak-in-gpu-detection`
- `docs/update-installation-guide`

### Workflow

1. **Create a new branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes**

3. **Test your changes**:
   ```bash
   cargo test
   cargo clippy
   cargo fmt
   ```

4. **Commit your changes**:
   ```bash
   git add .
   git commit -m "Add: Your descriptive commit message"
   ```

### Commit Messages

Follow these guidelines:
- Use present tense ("Add feature" not "Added feature")
- Use imperative mood ("Move cursor to..." not "Moves cursor to...")
- Start with a verb (Add, Fix, Update, Remove, etc.)
- Keep first line under 50 characters
- Add detailed description if needed

Examples:
```
Add temperature monitoring module

Implements CPU and GPU temperature detection for Linux and macOS
using sysfs and SMC respectively.
```

```
Fix memory leak in GPU detection

Properly free allocated memory after lspci command execution.
```

---

## Submitting Changes

1. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

2. **Create a Pull Request** on GitHub

3. **PR Title**: Should be descriptive and follow commit message guidelines

4. **PR Description**: Should include:
   - What changes were made
   - Why the changes were made
   - How to test the changes
   - Screenshots (if applicable)
   - Related issues (if any)

### PR Template

```markdown
## Description
Brief description of changes

## Motivation
Why are these changes needed?

## Changes Made
- Change 1
- Change 2

## Testing
How to test these changes

## Screenshots (if applicable)

## Checklist
- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] Code formatted with rustfmt
- [ ] No clippy warnings
```

---

## Coding Standards

### Rust Style Guide

Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/):

- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Avoid `unsafe` code unless absolutely necessary
- Document all public APIs
- Use meaningful variable names

### Code Organization

```rust
// Imports
use std::fs;
use anyhow::Result;

// Constants
const MAX_RETRIES: u32 = 3;

// Type definitions
pub struct MyModule {
    // fields
}

// Implementations
impl MyModule {
    pub fn new() -> Result<Self> {
        // constructor
    }
    
    fn private_helper(&self) -> String {
        // private methods
    }
}

// Trait implementations
impl Module for MyModule {
    // trait methods
}
```

### Error Handling

Use `anyhow::Result` for error propagation:

```rust
use anyhow::{Result, Context};

pub fn read_config() -> Result<Config> {
    let content = fs::read_to_string(path)
        .context("Failed to read config file")?;
    Ok(parse_config(&content)?)
}
```

---

## Adding New Modules

To add a new information module:

### 1. Create Module File

Create `src/modules/your_module.rs`:

```rust
use super::Module;
use anyhow::Result;

pub struct YourModule {
    value: String,
}

impl YourModule {
    pub fn new() -> Result<Self> {
        let value = Self::detect_info()?;
        Ok(Self { value })
    }

    fn detect_info() -> Result<String> {
        // Implementation
        Ok("detected value".to_string())
    }
}

impl Module for YourModule {
    fn name(&self) -> &str {
        "Your Module"
    }

    fn value(&self) -> Result<String> {
        Ok(self.value.clone())
    }
}
```

### 2. Register Module

In `src/modules/mod.rs`:

```rust
// Add to module declarations
pub mod your_module;

// Add to SystemInfo::new()
modules.insert(
    "your_module".to_string(),
    Box::new(your_module::YourModule::new()?)
);
```

### 3. Add to Default Config

In `src/config.rs`, add to `default_module_order()`:

```rust
vec![
    // ... existing modules
    "your_module".to_string(),
]
```

### 4. Add Tests

In `src/modules/your_module.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_your_module() {
        let module = YourModule::new().unwrap();
        assert_eq!(module.name(), "Your Module");
    }
}
```

### 5. Update Documentation

Add your module to:
- `README.md` (Available Modules section)
- Example config files
- `ARCHITECTURE.md` if needed

---

## Testing

### Run All Tests

```bash
cargo test
```

### Run Specific Test

```bash
cargo test test_name
```

### Run Tests with Output

```bash
cargo test -- --nocapture
```

### Platform-Specific Testing

Test on multiple platforms if possible:
- Linux (various distros)
- macOS
- Windows
- BSD (if accessible)

### Manual Testing

```bash
# Build and run
cargo build --release
./target/release/zerofetch

# Test with different flags
./target/release/zerofetch --modules cpu,memory
./target/release/zerofetch --format json
./target/release/zerofetch --no-color
```

---

## Documentation

### Code Documentation

Use Rustdoc comments:

```rust
/// Detects the current operating system
///
/// # Returns
/// 
/// Returns the OS name as a String
///
/// # Examples
///
/// ```
/// let os = detect_os();
/// assert!(!os.is_empty());
/// ```
pub fn detect_os() -> String {
    // implementation
}
```

### User Documentation

Update relevant documentation files:
- `README.md`: User-facing features
- `INSTALL.md`: Installation instructions
- `ARCHITECTURE.md`: Technical details
- Config examples: New configuration options

---

## Need Help?

- **Questions**: Open a [Discussion](https://github.com/ofcyouritachii/zerofetch/discussions)
- **Bugs**: Open an [Issue](https://github.com/ofcyouritachii/zerofetch/issues)
- **Chat**: Join our community chat (link TBD)

---

## Recognition

Contributors are recognized in:
- Git commit history
- Release notes
- Contributors section (future)

Thank you for contributing to ZeroFetch! 🚀
