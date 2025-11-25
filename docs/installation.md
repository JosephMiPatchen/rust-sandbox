# Installing Rust

This guide will walk you through installing Rust on your system. Rust works on Windows, macOS, and Linux.

## Quick Start (Recommended Method)

The easiest and recommended way to install Rust is using **rustup**, the official Rust installer and version manager.

### macOS and Linux

Open your terminal and run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This will:
1. Download the rustup installer
2. Install the latest stable version of Rust
3. Install Cargo (Rust's package manager and build tool)
4. Add Rust to your system PATH

**Follow the on-screen prompts** (usually just press Enter to accept defaults).

### Windows

1. Go to https://rustup.rs
2. Download `rustup-init.exe`
3. Run the installer
4. Follow the on-screen instructions

**Note:** You may need to install the Visual Studio C++ Build Tools first. The installer will guide you through this.

## What Gets Installed?

When you install Rust via rustup, you get:

| Tool | Purpose |
|------|---------|
| `rustc` | The Rust compiler |
| `cargo` | Package manager and build tool |
| `rustup` | Toolchain manager (updates, versions) |
| `rustfmt` | Code formatter |
| `clippy` | Linter for catching common mistakes |
| Standard library | Core Rust functionality |

## Verifying Installation

After installation, **restart your terminal** (or run `source $HOME/.cargo/env` on macOS/Linux), then verify:

```bash
# Check Rust compiler version
rustc --version
# Output: rustc 1.75.0 (or whatever the latest version is)

# Check Cargo version
cargo --version
# Output: cargo 1.75.0

# Check rustup version
rustup --version
# Output: rustup 1.26.0
```

If all three commands work, you're ready to go! 🎉

## Your First Rust Program

Let's create a simple "Hello, World!" program to test everything:

```bash
# Create a new project
cargo new hello_world
cd hello_world

# Run the project
cargo run
```

You should see:
```
   Compiling hello_world v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
     Running `target/debug/hello_world`
Hello, world!
```

Congratulations! Rust is working! 🦀

## Understanding rustup

**rustup** is Rust's toolchain manager. It lets you:
- Install/update Rust
- Switch between Rust versions
- Install additional components
- Manage cross-compilation targets

### Common rustup Commands

```bash
# Update Rust to the latest version
rustup update

# Show installed toolchains
rustup show

# Install a specific version
rustup install stable
rustup install nightly
rustup install 1.75.0

# Set default toolchain
rustup default stable

# Install additional components
rustup component add rustfmt    # Code formatter
rustup component add clippy     # Linter
rustup component add rust-src   # Source code (for IDE features)

# Install cross-compilation targets
rustup target add x86_64-pc-windows-gnu    # Windows
rustup target add x86_64-apple-darwin      # macOS
rustup target add wasm32-unknown-unknown   # WebAssembly
```

## Rust Versions: Stable, Beta, Nightly

Rust has three release channels:

### Stable (Recommended)
- Released every 6 weeks
- Fully tested and production-ready
- **Use this for most projects**
- Example: `1.75.0`

```bash
rustup default stable
```

### Beta
- Next version being tested
- For testing before stable release
- Usually stable, but not guaranteed

```bash
rustup default beta
```

### Nightly
- Bleeding edge features
- Updated daily
- Experimental features not yet in stable
- May break your code

```bash
rustup default nightly
```

**Recommendation:** Stick with **stable** unless you need experimental features.

## IDE Setup

### Visual Studio Code (Recommended)

1. Install VS Code: https://code.visualstudio.com
2. Install the **rust-analyzer** extension:
   - Open VS Code
   - Go to Extensions (Cmd+Shift+X on macOS, Ctrl+Shift+X on Windows/Linux)
   - Search for "rust-analyzer"
   - Click Install

**rust-analyzer** provides:
- Code completion
- Error checking
- Go to definition
- Inline documentation
- Refactoring tools

### Other IDEs

| IDE | Plugin/Extension |
|-----|------------------|
| IntelliJ IDEA / CLion | IntelliJ Rust plugin |
| Sublime Text | Rust Enhanced |
| Vim/Neovim | rust.vim + coc-rust-analyzer |
| Emacs | rust-mode + rustic |

## Updating Rust

Rust releases a new stable version every 6 weeks. To update:

```bash
# Update all installed toolchains
rustup update

# Update just stable
rustup update stable
```

This is safe and won't break your existing projects!

## Uninstalling Rust

If you ever need to uninstall Rust:

```bash
rustup self uninstall
```

This removes:
- All Rust toolchains
- Cargo
- rustup itself
- All installed components

## Troubleshooting

### "command not found: cargo" or "command not found: rustc"

**Solution:** Restart your terminal or run:

```bash
# macOS/Linux
source $HOME/.cargo/env

# Windows (PowerShell)
$env:Path += ";$env:USERPROFILE\.cargo\bin"
```

If that doesn't work, rustup may not have added Rust to your PATH. Manually add:
- **macOS/Linux:** `~/.cargo/bin` to your PATH
- **Windows:** `%USERPROFILE%\.cargo\bin` to your PATH

### Windows: "Visual C++ Build Tools not found"

**Solution:** Install Visual Studio C++ Build Tools:
1. Go to https://visualstudio.microsoft.com/downloads/
2. Download "Build Tools for Visual Studio"
3. Run installer and select "C++ build tools"
4. Restart and run rustup again

### Permission Denied Errors

**Solution:** Don't use `sudo` with rustup or cargo. They should install to your home directory.

If you accidentally used sudo:
```bash
# Fix ownership (macOS/Linux)
sudo chown -R $USER:$USER ~/.cargo ~/.rustup
```

### Slow Compilation on macOS

**Solution:** Install a faster linker:

```bash
# Install lld via Homebrew
brew install llvm

# Add to ~/.cargo/config.toml
mkdir -p ~/.cargo
cat >> ~/.cargo/config.toml << EOF
[target.x86_64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
EOF
```

## Additional Tools

### Cargo Extensions

Install useful cargo extensions:

```bash
# cargo-edit: Add/remove dependencies from command line
cargo install cargo-edit

# cargo-watch: Auto-recompile on file changes
cargo install cargo-watch

# cargo-audit: Check for security vulnerabilities
cargo install cargo-audit

# cargo-outdated: Check for outdated dependencies
cargo install cargo-outdated
```

Usage:
```bash
# Add a dependency
cargo add serde

# Remove a dependency
cargo rm serde

# Auto-recompile on changes
cargo watch -x run

# Check for vulnerabilities
cargo audit

# Check for outdated dependencies
cargo outdated
```

## System Requirements

### Minimum Requirements
- **Disk Space:** ~1.5 GB (for Rust toolchain)
- **RAM:** 2 GB minimum, 4 GB recommended
- **OS:** 
  - macOS 10.12 (Sierra) or later
  - Windows 7 or later
  - Linux (most distributions)

### Supported Architectures
- x86_64 (64-bit Intel/AMD)
- ARM64 (Apple Silicon, Raspberry Pi 4+)
- ARM (32-bit)
- And many more via cross-compilation

## Next Steps

Now that Rust is installed, you can:

1. **Read the Rust Book** (free online): https://doc.rust-lang.org/book/
   - The official guide to learning Rust
   - Comprehensive and beginner-friendly

2. **Try Rust by Example**: https://doc.rust-lang.org/rust-by-example/
   - Learn by doing with interactive examples

3. **Explore crates.io**: https://crates.io
   - Browse available Rust libraries

4. **Join the Community**:
   - Official Forum: https://users.rust-lang.org
   - Discord: https://discord.gg/rust-lang
   - Reddit: https://reddit.com/r/rust

5. **Build Something!**
   - Check out our Hangman game in this repository
   - Try creating a CLI tool
   - Build a web server

## Quick Reference

```bash
# Install Rust (macOS/Linux)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version

# Update Rust
rustup update

# Create new project
cargo new my_project
cd my_project
cargo run

# Build project
cargo build          # Debug build
cargo build --release # Optimized build

# Check code without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy

# Run tests
cargo test

# Build documentation
cargo doc --open
```

## Resources

- **Official Website**: https://www.rust-lang.org
- **Install Page**: https://rustup.rs
- **Documentation**: https://doc.rust-lang.org
- **Rust Book**: https://doc.rust-lang.org/book/
- **Rust by Example**: https://doc.rust-lang.org/rust-by-example/
- **Standard Library Docs**: https://doc.rust-lang.org/std/
- **Cargo Book**: https://doc.rust-lang.org/cargo/

## Summary

1. Install via rustup: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Restart terminal
3. Verify: `rustc --version` and `cargo --version`
4. Install rust-analyzer for your IDE
5. Create your first project: `cargo new hello_world`
6. Start coding! 🦀

Welcome to the Rust community! 🎉
