<h1 align="center">Rust Windows Testing with Wine GitHub Action</h1>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License"></a>
</p>

This GitHub Action allows you to test Windows Rust projects on Linux runners using Wine.
It supports both GNU targets (via Wine) and MSVC targets (via cargo-xwin).

## Features

- 🍷 Test Windows Rust projects on `ubuntu-latest` using latest Wine
- 🦀 Support for GNU targets (`x86_64-pc-windows-gnu`, `i686-pc-windows-gnu`) via Wine
- 🔧 Optional MSVC target support (`x86_64-pc-windows-msvc`, `i686-pc-windows-msvc`) via cargo-xwin
- 🎯 Optimized caching with single target per call
- 🛠️ Customize Rust toolchain (stable, nightly, beta, specific versions)
- 📦 Enable or disable default features
- 🔧 Specify additional features to test with
- ⚡ Fast installation via `cargo-binstall`
- 💾 Optional Rust toolchain caching for faster builds

## Usage

To use this action in your GitHub workflow, add the following step:

```yaml
- name: Test Windows Project with Wine
   uses: Reloaded-Project/devops-rust-test-in-latest-wine@v1
   with:
     rust-project-path: '.'
     rust-toolchain: 'stable'
     target: 'x86_64-pc-windows-gnu'
```

## Inputs

| Input                    | Description                                                                                        | Required | Default                   |
| ------------------------ | -------------------------------------------------------------------------------------------------- | -------- | ------------------------- |
| `rust-project-path`      | Path to the Rust project                                                                           | No       | `'.'`                     |
| `rust-toolchain`         | Rust toolchain to use for building and testing (e.g., stable, nightly, 1.75.0, nightly-2024-02-08) | No       | `'stable'`                |
| `target`                 | Windows target to test                                                                             | No       | `'x86_64-pc-windows-gnu'` |
| `install-rust-toolchain` | Whether to install the specified Rust toolchain                                                    | No       | `true`                    |
| `setup-rust-cache`       | Whether to set up Rust caching                                                                     | No       | `true`                    |
| `use-xwin`               | Whether to use cargo-xwin for MSVC targets                                                         | No       | `false`                   |
| `use-binstall`           | Whether to use cargo-binstall for installing components. If false, uses cargo install.             | No       | `true`                    |
| `install-binstall`       | Whether to install cargo-binstall. If false, assumes it is already available in the environment.   | No       | `true`                    |
| `features`               | Space-separated list of features to enable during testing                                          | No       | `''`                      |
| `no-default-features`    | Disable default features during testing                                                            | No       | `false`                   |
| `additional-test-args`   | Additional arguments to pass to the cargo test command                                             | No       | `''`                      |
| `wine-version`           | Wine version to install from WineHQ repository (e.g., stable, devel, staging)                      | No       | `'stable'`                |

Windows targets tested are `x86_64-pc-windows-gnu`, `i686-pc-windows-gnu`, `x86_64-pc-windows-msvc`, `i686-pc-windows-msvc`.

## Example Workflows

### Basic GNU Target Testing

Test a single GNU target using Wine:

```yaml
name: Test Windows GNU

on: [push]

jobs:
   test:
     runs-on: ubuntu-latest
     steps:
       - uses: actions/checkout@v4
       - name: Test Windows x86_64 GNU
         uses: Reloaded-Project/devops-rust-test-in-latest-wine@v1
         with:
           rust-project-path: '.'
           target: 'x86_64-pc-windows-gnu'
```

### Test Multiple Targets with Matrix

Test both 64-bit and 32-bit GNU targets using a matrix strategy:

```yaml
name: Test Multiple GNU Targets

on: [push]

jobs:
   test:
     strategy:
       matrix:
         target:
           - x86_64-pc-windows-gnu
           - i686-pc-windows-gnu
     runs-on: ubuntu-latest
     steps:
       - uses: actions/checkout@v4
       - name: Test ${{ matrix.target }}
         uses: Reloaded-Project/devops-rust-test-in-latest-wine@v1
         with:
           rust-project-path: '.'
           target: ${{ matrix.target }}
```

### MSVC Targets with cargo-xwin

Test MSVC targets using cargo-xwin:

```yaml
name: Test Windows MSVC

on: [push]

jobs:
   test:
     runs-on: ubuntu-latest
     steps:
       - uses: actions/checkout@v4
       - name: Test Windows MSVC
         uses: Reloaded-Project/devops-rust-test-in-latest-wine@v1
         with:
           rust-project-path: '.'
           target: 'x86_64-pc-windows-msvc'
           use-xwin: true
```

> [!NOTE]
> cargo-xwin builds using MSVC headers and MSVC tools. Closest you can get to true MSVC testing on Linux.

## How It Works

**GNU Targets** (`x86_64-pc-windows-gnu`, `i686-pc-windows-gnu`): Compiled with the Rust GNU toolchain and tested under Wine, which provides Windows API compatibility on Linux.

**MSVC Targets** (`x86_64-pc-windows-msvc`, `i686-pc-windows-msvc`): Compiled with cargo-xwin (requires `use-xwin: true`) using MSVC headers and tools with a Clang backend, enabling MSVC target support without a Windows environment.

**Single Target per Call**: Each action call tests one target to optimize caching. Use a GitHub Actions matrix to test multiple targets in parallel with isolated caches.

## CI and Testing

This action uses a dual-tag strategy for development and production:

- **`@v1-test` tag**: Used internally for testing the action itself in CI workflows
- **`@v1` tag**: Production-ready version for end users

## Rust Toolchain Examples

You can specify various Rust toolchain versions:

```yaml
# Use stable channel
rust-toolchain: 'stable'

# Use nightly channel
rust-toolchain: 'nightly'

# Use specific version
rust-toolchain: '1.75.0'

# Use specific nightly date
rust-toolchain: 'nightly-2024-02-08'

# Use beta channel
rust-toolchain: 'beta'
```

## License

This GitHub Action is released under the [MIT License](LICENSE).
