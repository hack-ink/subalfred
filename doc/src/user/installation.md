# Installation
There are multiple ways to install the `subalfred` CLI tool.
Choose any one of the methods below that best suit your needs.

## Pre-compiled Binaries
Executable binaries are available for download on the [GitHub Releases page][releases].
Download the binary for your platform (Windows, macOS, or Linux) and extract the archive.
The archive contains an `subalfred` executable which you can run to build your books.

The compressing algorithm is zstd.
For some old systems, you might need to install zstd lib first.
And for the macOS users, you need to install gnu-tar and replace the `tar` command with `gtar`.

Example:
```sh
# One line command
curl -L https://github.com/hack-ink/subalfred/releases/download/v0.9.0-rc6/subalfred-x86_64-unknown-linux-gnu.tar.zst | tar x -I pzstd
# Or
curl -LO https://github.com/hack-ink/subalfred/releases/download/v0.9.0-rc6/subalfred-x86_64-unknown-linux-gnu.tar.zst
tar xf subalfred-x86_64-unknown-linux-gnu.tar.zst -I pzstd
```

[releases]: https://github.com/hack-ink/subalfred/releases

## Build from Source
To build the `subalfred` executable from source, you will first need to install Rust and Cargo.
Once you have installed Rust, the following command can be used to build and install `subalfred`:
```sh
# Build from crates.io:
cargo install subalfred
# Build from the latest main branch code:
cargo install subalfred --git https://github.com/hack-ink/subalfred
```

This will automatically download `subalfred` from [crates.io]/[github.com], build it, and install it in Cargo's global binary directory (`~/.cargo/bin/` by default).

The version published to crates.io will ever so slightly be behind the version hosted on GitHub.
If you need the latest version you can build the git version of `subalfred` yourself.
Cargo makes this ***super easy***!

To uninstall, run the command `cargo uninstall subalfred`.

[crates.io]: https://crates.io
[github.com]: https://github.com/hack-ink/subalfred
