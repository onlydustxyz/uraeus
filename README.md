<div align="center">
  <h1 align="center">Uræus</h1>
  <p align="center">
    <a href="https://discord.gg/onlydust">
        <img src="https://img.shields.io/badge/Discord-6666FF?style=for-the-badge&logo=discord&logoColor=white">
    </a>
    <a href="https://twitter.com/intent/follow?screen_name=onlydust_xyz">
        <img src="https://img.shields.io/badge/Twitter-1DA1F2?style=for-the-badge&logo=twitter&logoColor=white">
    </a>       
  </p>
  
  <h3 align="center">Command line utilities to check StarkNet contracts written in Cairo.</h3>
</div>

> ## ⚠️ WARNING! ⚠️
>
> This repo contains highly experimental code.
> Expect rapid iteration.

## Prerequisites

- [Rust](https://www.rust-lang.org/)
- [Protostar](https://github.com/software-mansion/protostar)
- [Nile](https://github.com/OpenZeppelin/nile)

## 📦 Installation

## 🔧 Build

```bash
cargo build
```

## 🔬 Usage

### Verify

Verify checks if a source code matches a deployed contract.

```
uraeus-verify
verify source code of deployed smart contracts

USAGE:
    uraeus verify [OPTIONS] <address>

ARGS:
    <address>    Address of the smart contract

OPTIONS:
    -b, --builddir <BUILD_DIR>        Build directory [default: ]
    -h, --help                        Print help information
    -p, --projectdir <PROJECT_DIR>    Project root directory [default: ]
```

## 📗 Logging 

Change log level using `RUST_LOG` environment variable.

Example:

```bash
RUST_LOG=debug cargo run -- verify 0x4bfedc224c8360eaa16969c5db2944d19c32dbabdb4fc0d93bb3ea759c7198c
```

## 🌡️ Testing

```bash
cargo test --verbose
```

## 🏄‍♂️ Test coverage

```bash
sh scripts/test_coverage.sh
```

## 🛠️ Development

### Install git hooks

```bash
sh scripts/install_git_hooks.sh
```

## 📄 License

**uraeus** is released under the [MIT](LICENSE).