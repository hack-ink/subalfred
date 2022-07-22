<div align="center">

<!-- Logo -->
<!-- ![Subalfred]() -->

# Subalfred
### An All-In-One [Substrate](https://github.com/paritytech/substrate) Development Toolbox.

[![License](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Checks](https://github.com/hack-ink/subalfred/workflows/checks/badge.svg)](https://github.com/hack-ink/subalfred/actions/workflows/checks.yml)
[![Release](https://github.com/hack-ink/subalfred/workflows/release/badge.svg)](https://github.com/hack-ink/subalfred/actions/workflows/release.yml)
[![GitHub tag (latest by date)](https://img.shields.io/github/v/tag/hack-ink/subalfred)](https://github.com/hack-ink/subalfred/tags)
[![GitHub code lines](https://tokei.rs/b1/github/hack-ink/subalfred)](https://github.com/hack-ink/subalfred)
[![GitHub last commit](https://img.shields.io/github/last-commit/hack-ink/subalfred?color=red&style=plastic)](https://github.com/hack-ink/subalfred)

[![Subalfred](https://repobeats.axiom.co/api/embed/acdcdaf322ac3f7e821eb71a6985b14ec57e5c44.svg "Repobeats analytics image")](https://github.com/hack-ink/subalfred/pulse)

</div>

## Philosophies
- **Lighting Fast**
- **Less Dependencies**
- **No `unsafe {}`/`.expect()`/`.unwrap()`**
- **Easy to Use**
- **Strive for Excellence**

## Usage
```
subalfred 0.9.0-1bc3414-x86_64-unknown-linux-gnu
Xavier Lau <xavier@inv.cafe>
Your Substrate Alfred

USAGE:
    subalfred [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -h, --help                    Print help information
        --log <TARGET=LEVEL,*>    Set a custom logging filter. Also, work with the `RUST_LOG` environment variable [default: info]
    -V, --version                 Print version information

SUBCOMMANDS:
    check           Some checking tools are pretty useful for runtime development
    export-state    Export the chain state
    hash            Hash the hex with the specific hasher
    help            Print this message or the help of the given subcommand(s)
    key             Convert the public key/SS58 address from SS58 address/public key
    storage-key     Calculate the storage key for the storage prefix/item
    workspace       Workspace manager
```

## Components
### CLI
- [check](src/bin/subalfred/command/check)
  - [runtime](src/bin/subalfred/command/check/runtime)
  - [std-feature](src/bin/subalfred/command/check/std-feature)
- [workspace](src/bin/subalfred/command/workspace)
  - [update](src/bin/subalfred/command/update)
- [export-state](src/bin/subalfred/command/export-state.rs)
- [hash](src/bin/subalfred/command/hash.rs)
- [key](src/bin/subalfred/command/key.rs)
- [storage-key](src/bin/subalfred/command/storage-key.rs)

### Subalfred Core Libraries
- [cargo](src/subalfred/core/cargo)
- [check](src/subalfred/core/check)
- [error](src/subalfred/core/error)
- [http](src/subalfred/core/http)
- [jsonrpc](src/subalfred/core/jsonrpc)
- [key](src/subalfred/core/key)
- [node](src/subalfred/core/node)
- [ss58](src/subalfred/core/ss58)
- [substrate-client](src/subalfred/core/substrate-client)
- [system](src/subalfred/core/system)

### Substrate Minimal Libraries
- [subcryptor](substrate-minimal/subcryptor)
- [subgrandpa](substrate-minimal/subgrandpa)
- [subhasher](substrate-minimal/subhasher)
- [submetadatan](substrate-minimal/submetadatan)
- [subrpcer](substrate-minimal/subrpcer)
  - [impl](substrate-minimal/subrpcer/impl)
- [substorager](substrate-minimal/substorager)
- [subversion](substrate-minimal/subversion)
