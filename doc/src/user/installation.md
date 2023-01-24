# Installation
There are multiple ways to install the `subalfred` CLI tool.
Choose any one of the methods below that best suit your needs.

## Pre-built Binaries
1. Download it from the [GitHub Releases page].
2. Uncompress the downloaded file.
3. Rename it to whatever you like. (*optional*)
4. Give the execution privilege to it.

The compressing algorithm is [zstd].
For some systems, you might need to install [zstd] first.
For *Windows* users, you would like to rename the extracted content name to `subalfred.exe`.

Use `subalfred-aarch64-apple-darwin.zst` as an example.
- One line command:
	```sh
	curl -L https://github.com/hack-ink/subalfred/releases/latest/download/subalfred-aarch64-apple-darwin.zst | zstd -o subalfred -d && chmod u+x subalfred
	```
- Multi-lines command:
	```sh
	curl -LO https://github.com/hack-ink/subalfred/releases/latest/download/subalfred-aarch64-apple-darwin.zst
	zstd -o subalfred -d
	chmod u+x subalfred
	```

Available pre-built binaries list.
- `subalfred-aarch64-apple-darwin.zst`
- `subalfred-x86_64-apple-darwin.zst`
- `subalfred-x86_64-pc-windows-msvc.zst`
- `subalfred-x86_64-unknown-linux-gnu.zst`

[GitHub Releases page]: https://github.com/hack-ink/subalfred/releases
[zstd]: https://github.com/facebook/zstd/releases

## Build from Source
To build `subalfred` from source, you will first need to install [Rust].
Once you have installed [Rust], the following command can be used to build and install `subalfred`:

- Build from crates.io:
	```sh
	cargo install subalfred
	```
- Build from the latest main branch code:
	```sh
	cargo install subalfred --git https://github.com/hack-ink/subalfred
	```

This will automatically download `subalfred` from [crates.io]/[github.com], build it, and install it in Cargo's global binary directory (`~/.cargo/bin/` by default).

The version published to [crates.io] will ever so slightly be behind the version hosted on [github.com].
If you need the latest version you can build the git version of `subalfred` yourself.
Cargo makes this ***super easy***!

To uninstall, run the command `cargo uninstall subalfred`.

[Rust]: https://rustup.rs
[crates.io]: https://crates.io
[github.com]: https://github.com/hack-ink/subalfred
