# Amazon Q CLI

> [!IMPORTANT]
> This open source project is no longer being actively maintained and will only receive critical security fixes. Amazon Q Developer CLI is now available as [Kiro CLI](https://kiro.dev/cli/), a closed-source product. For the latest features and updates, please use Kiro CLI.

## Installation

- **macOS**:
  - **DMG**: [Download now](https://desktop-release.q.us-east-1.amazonaws.com/latest/Amazon%20Q.dmg)
  - **HomeBrew**: ```brew install --cask amazon-q ```
- **Linux**:
  - [Ubuntu/Debian](https://docs.aws.amazon.com/amazonq/latest/qdeveloper-ug/command-line-installing.html#command-line-installing-ubuntu)
  - [AppImage](https://docs.aws.amazon.com/amazonq/latest/qdeveloper-ug/command-line-installing.html#command-line-installing-appimage)
  - [Alternative Linux builds](https://docs.aws.amazon.com/amazonq/latest/qdeveloper-ug/command-line-installing.html#command-line-installing-alternative-linux)
- **Windows**:
  - Download the latest Windows build from [Releases](https://github.com/aws/amazon-q-developer-cli/releases)
  - Extract `qchat-windows-x64.zip`
  - Run `qchat.exe` from the extracted folder

## Contributing

Thank you so much for considering to contribute to Amazon Q.

Before getting started, see our [contributing docs](CONTRIBUTING.md#security-issue-notifications).

### Prerequisites

- **macOS**
  - Xcode 13 or later
  - Brew
- **Windows**
  - Visual Studio 2019 or later (with C++ build tools) OR Visual Studio Build Tools 2019+
  - Python 3.8 or later
  - Rust toolchain with MSVC target (see step 2 below)

#### 1. Clone repo

```shell
git clone https://github.com/aws/amazon-q-developer-cli.git
```

#### 2. Install the Rust toolchain using [Rustup](https://rustup.rs):

**macOS/Linux:**
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
rustup toolchain install nightly
cargo install typos-cli
```

**Windows:**
```powershell
# Download and run rustup-init.exe from https://rustup.rs
# Or use winget:
winget install Rustlang.Rustup

# After installation, add the MSVC target:
rustup default stable
rustup toolchain install nightly
rustup target add x86_64-pc-windows-msvc
cargo install typos-cli
```

#### 3. Develop locally

- To compile and run: `cargo run --bin chat_cli`.
- To run tests: `cargo test`.
- To run lints: `cargo clippy`.
- To format rust files: `cargo +nightly fmt`.
- To run subcommands: `cargo run --bin chat_cli -- {subcommand}`.
  - Login would then be: `cargo run --bin chat_cli -- login`

#### 4. Build release binaries

**macOS/Linux:**
```shell
python scripts/main.py build --release
```

**Windows:**
```powershell
python scripts/main.py build --release
```

The build output will be in the `build/` directory:
- **macOS**: `build/qchat.zip`
- **Linux**: `build/qchat.tar.gz` and `build/qchat.zip`
- **Windows**: `build/qchat-windows-x64.zip`

### Troubleshooting

#### Windows Build Issues

**Missing MSVC compiler:**
```
error: linker `link.exe` not found
```
Solution: Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022) with C++ build tools.

**Python not found:**
```
'python' is not recognized as an internal or external command
```
Solution: Install Python from [python.org](https://www.python.org/downloads/) or Microsoft Store, and ensure it's in your PATH.

**Rust target not installed:**
```
error: can't find crate for `std`
```
Solution: Run `rustup target add x86_64-pc-windows-msvc`

## Project Layout

- [`chat_cli`](crates/chat-cli/) - the `q` CLI, allows users to interface with Amazon Q Developer from
  the command line
- [`scripts/`](scripts/) - Contains ops and build related scripts
- [`crates/`](crates/) - Contains all rust crates
- [`docs/`](docs/) - Contains technical documentation

## Security

For security related concerns, see [here](SECURITY.md).

## Licensing

This repo is dual licensed under MIT and Apache 2.0 licenses.

Those licenses can be found [here](LICENSE.MIT) and [here](LICENSE.APACHE).

“Amazon Web Services” and all related marks, including logos, graphic designs, and service names, are trademarks or trade dress of AWS in the U.S. and other countries. AWS’s trademarks and trade dress may not be used in connection with any product or service that is not AWS’s, in any manner that is likely to cause confusion among customers, or in any manner that disparages or discredits AWS.
