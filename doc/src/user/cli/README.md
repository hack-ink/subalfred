# Command `subalfred`
```
Your Substrate Alfred.

Usage: subalfred [OPTIONS] <COMMAND>

Commands:
  check
          Substrate development checkers
  convert
          Data style converters
  get
          Substrate-link node getter
  hash
          Hash the hex with the specific hasher(hash algorithm)
  key
          Calculate the public key/SS58 address of the SS58 address/public key
  rpc
          Send a RPC request to the node's HTTP RPC endpoint.
  state
          A set of tools to process Substrate-like chain state
  storage-key
          Calculate the storage key of the storage item
  track-updates
          Track the updates
  workspace
          Workspace manager
  help
          Print this message or the help of the given subcommand(s)

Options:
  -l, --log <TARGET=LEVEL,*>
          Set a custom log filter.

          This flag is also working with the `RUST_LOG` environment variable. If you use `RUST_LOG`
          simultaneously, this will append `RUST_LOG`'s value after the log.

          [default: info]

  -h, --help
          Print help information (use `-h` for a summary)

  -V, --version
          Print version information
```
