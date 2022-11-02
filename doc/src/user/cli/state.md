# Command `state`
```
A set of tools to process Substrate-like chain state

Usage: subalfred state [OPTIONS] <COMMAND>

Commands:
  diff
          Check the differences between the two states.
          Note:
          This is not a symmetric difference operation.
          `a.diff(b)` might equal `b.diff(a)`, but not always.
  export
          Export the chain state from the Substrate-like node through the WS RPC endpoint
  fork-off
          Fork-off the Substrate-like chain state
  insert
          Insert the key/value pair into the specific file
  override
          Override the chain spec a with b.
  help
          Print this message or the help of the given subcommand(s)

Options:
  -l, --log <TARGET=LEVEL,*>
          Set a custom log filter.

          This flag is also working with the `RUST_LOG` environment variable. If you use `RUST_LOG` simultaneously, this will append `RUST_LOG`'s value after the log.

          [default: info]

  -h, --help
          Print help information (use `-h` for a summary)
```

## Command `state diff`
```
Check the differences between the two states.
Note:
This is not a symmetric difference operation.
`a.diff(b)` might equal `b.diff(a)`, but not always.

Usage: subalfred state diff [OPTIONS] <PATH> <PATH>

Arguments:
  <PATH>
          Chain spec a's path

  <PATH>
          Chain spec b's path

Options:
  -l, --log <TARGET=LEVEL,*>
          Set a custom log filter.

          This flag is also working with the `RUST_LOG` environment variable. If you use `RUST_LOG` simultaneously, this will append `RUST_LOG`'s value after the log.

          [default: info]

  -h, --help
          Print help information (use `-h` for a summary)
```

### Example
```sh
subalfred state diff chain-spec-a.json chain-spec-b.json
```
```diff
-0xbd2a529379475088d3e29a918cd47872878d434d6125b40443fe11fd292d13a4:0x03000000
-0x4d45a146e2a002ba470f48b9ed9a3e23878d434d6125b40443fe11fd292d13a4:0x02000200
-0xcebf674407db61a30e8759ec5084b7764e7b9012096b41c4eb3aaf947f6ea429:0x0000
```
```sh
subalfred state diff chain-spec-b.json chain-spec-a.json
```
```diff
+0xbd2a529379475088d3e29a918cd47872878d434d6125b40443fe11fd292d13a4:0x03000000
+0x4d45a146e2a002ba470f48b9ed9a3e23878d434d6125b40443fe11fd292d13a4:0x02000200
+0xcebf674407db61a30e8759ec5084b7764e7b9012096b41c4eb3aaf947f6ea429:0x0000
```

## Command `state export`
```
Export the chain state from the Substrate-like node through the WS RPC endpoint.

The result will be stored at `<a>.export`.

Usage: subalfred state export [OPTIONS] <URI>

Arguments:
  <URI>
          Live chain's HTTP RPC endpoint

Options:
      --at <HASH/NUM>
          Export the data starting from this block

      --timeout <SECS>
          Timeout for the fetching

          [default: 10]

      --all
          Export all the data.

          So, it conflicts with any other filter option.

          Note:
          The default behaviour (without this option) is fetching according to metadata's pallet
          storage records, which means if there is any old storage prefix that can not be found in
          the current runtime's pallet storage names will be ignored.

      --skip-pallets <[PALLET]>
          Skip these pallets, while fetching.

          It's useful when you want to skip the 'large' pallets.

      --renew-consensus-with <PATH>
          Renew the consensus relate things of the chain.

          We need the dev chain specification to renew the consensus relates genesis. Otherwise, the
          fork-off chain won't produce block.

          It will:
          - Skip `["System", "Babe", "Authorship", "Session", "Grandpa", "Beefy"]` pallets, but keep
            the `System::Account` data. (in order to make the new chain runnable)
          - Change the id and impl name to `*-export`.
          - Clear the bootnodes.
          - Set the `Staking::ForceEra` to `ForceNone`. (in order to prevent the validator set from
            changing mid-test)

          Usually use this as below to get a runnable fork-off chain, and you can do whatever you
          want on it. Test new features, runtime upgrade, etc.

          ```sh
          xxx-node --export-state > xxx-export.json
          xxx-node --build-spec xxx-dev > xxx-dev.json
          subalfred state fork-off xxx-export.json --renew-consensus-with xxx.dev.json --simple-governance --disable-default-bootnodes
          xxx-node --chain xxx.json.fork-off --alice --tmp
          ```

          Note:
          `--alice` only works for which dev chain's genesis validator is `//Alice`, otherwise the
          new chain won't produce block. If your dev chain's genesis validator is `//Bob`, then
          running with `--bob`. But if your dev chain's genesis validator isn't any one of the
          well-known keys, then you should start the node with `--validator` and insert the key
          manually.

      --simple-governance
          Use `//Alice` to control the governance.

          It's useful when you want to test the runtime upgrade.

          It will:
          - Replace sudo key with `//Alice`, if the sudo pallet existed.
          - Replace phragmen election and council members with `//Alice`, if the collective pallet
            existed.
          - Replace technical membership and tech.comm members with `//Alice`, if the membership
            pallet existed.

      --disable-default-bootnodes
          Disable adding the default bootnodes to the specification.

  -l, --log <TARGET=LEVEL,*>
          Set a custom log filter.

          This flag is also working with the `RUST_LOG` environment variable. If you use `RUST_LOG` simultaneously, this will append `RUST_LOG`'s value after the log.

          [default: info]

  -h, --help
          Print help information (use `-h` for a summary)
```

### Example
#### ENV preparation
```sh
git clone https://github.com/substrate-developer-hub/substrate-node-template.git /tmp/subalfred-example/substrate-node-template
cd /tmp/subalfred-example/substrate-node-template
cargo build
target/debug/node-template build-spec --dev --raw > chain-spec.json
target/debug/node-template --chain chain-spec.json --alice --tmp
```
Open [PolkadotApps account page](https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9944#/accounts).
Check **Dave**'s balances is **0**. Then transfer **10 UNIT** from **Alice** to **Dave**.

#### Start example
```sh
# Open a new terminal.
cd /tmp/subalfred-example/substrate-node-template
subalfred state export ws://127.0.0.1:9944 -lsubalfred_core::state,subalfred_core::substrate_client
```
The exported state is locate at `/tmp/subalfred-example/substrate-node-template/default-chain-spec.json.export`.

## Command `state fork-off`
```
Fork-off the Substrate-like chain state.

The result will be stored at `<a>.fork-off`.

Usage: subalfred state fork-off [OPTIONS] <PATH>

Arguments:
  <PATH>
          Target chain spec file's path

Options:
      --renew-consensus-with <PATH>
          Renew the consensus relate things of the chain.

          We need the dev chain specification to renew the consensus relates genesis. Otherwise, the
          fork-off chain won't produce block.

          It will:
          - Skip `["System", "Babe", "Authorship", "Session", "Grandpa", "Beefy"]` pallets, but keep
            the `System::Account` data. (in order to make the new chain runnable)
          - Change the id and impl name to `*-export`.
          - Clear the bootnodes.
          - Set the `Staking::ForceEra` to `ForceNone`. (in order to prevent the validator set from
            changing mid-test)

          Usually use this as below to get a runnable fork-off chain, and you can do whatever you
          want on it. Test new features, runtime upgrade, etc.

          ```sh
          xxx-node --export-state > xxx-export.json
          xxx-node --build-spec xxx-dev > xxx-dev.json
          subalfred state fork-off xxx-export.json --renew-consensus-with xxx.dev.json --simple-governance --disable-default-bootnodes
          xxx-node --chain xxx.json.fork-off --alice --tmp
          ```

          Note:
          `--alice` only works for which dev chain's genesis validator is `//Alice`, otherwise the
          new chain won't produce block. If your dev chain's genesis validator is `//Bob`, then
          running with `--bob`. But if your dev chain's genesis validator isn't any one of the
          well-known keys, then you should start the node with `--validator` and insert the key
          manually.

      --simple-governance
          Use `//Alice` to control the governance.

          It's useful when you want to test the runtime upgrade.

          It will:
          - Replace sudo key with `//Alice`, if the sudo pallet existed.
          - Replace phragmen election and council members with `//Alice`, if the collective pallet
            existed.
          - Replace technical membership and tech.comm members with `//Alice`, if the membership
            pallet existed.

      --disable-default-bootnodes
          Disable adding the default bootnodes to the specification.

  -l, --log <TARGET=LEVEL,*>
          Set a custom log filter.

          This flag is also working with the `RUST_LOG` environment variable. If you use `RUST_LOG` simultaneously, this will append `RUST_LOG`'s value after the log.

          [default: info]

  -h, --help
          Print help information (use `-h` for a summary)
```

### Example
#### ENV preparation
Finish the [Command `state export`](#command-state-export)'s example first.

#### Start example
```sh
cd /tmp/subalfred-example/substrate-node-template
subalfred state fork-off default-chain-spec.json.export --renew-consensus-with chain-spec.json
target/debug/node-template --chain default-chain-spec.json.export.fork-off --alice --tmp
```
Check block number from the console, it should be started from **0**.
It means this is a new chain.

Open [PolkadotApps account page](https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9944#/accounts).
Check **Dave**'s balances, it should be **10**.
It means this is a new chain with the exported/fork-off(ed) data.

## Command `state override`
```
Override the chain spec a with b.

The result will be stored at `<a>.override`.

Usage: subalfred state override [OPTIONS] <PATH> <PATH>

Arguments:
  <PATH>
          Chain spec a's path

  <PATH>
          Chain spec b's path

Options:
  -l, --log <TARGET=LEVEL,*>
          Set a custom log filter.

          This flag is also working with the `RUST_LOG` environment variable. If you use `RUST_LOG` simultaneously, this will append `RUST_LOG`'s value after the log.

          [default: info]

  -h, --help
          Print help information (use `-h` for a summary)
```

### Example
```sh
subalfred state override a.json b.json
# You could use `diff` command to check the result, its output should be empty.
subalfred state diff a.json.override b.json
```

## Command `state insert`
```
Insert the key/value pair into the specific file.

If the key already exists, it will be overwritten.

# Examples ```sh # Calculate the WASM code key. subalfred convert ascii2hex ':code' # "0x3a636f6465" # Override the WASM code. subalfred state insert chain-spec.json --key 0x3a636f6465 --with-file runtime.compact.compressed.wasm ```

Usage: subalfred state insert [OPTIONS] --key <HEX> <--value <HEX>|--with-file <PATH>> <PATH>

Arguments:
  <PATH>
          Target state file's path

Options:
      --key <HEX>
          Storage key

      --value <HEX>
          Storage value

      --with-file <PATH>
          Storage value file

  -l, --log <TARGET=LEVEL,*>
          Set a custom log filter.

          This flag is also working with the `RUST_LOG` environment variable. If you use `RUST_LOG` simultaneously, this will append `RUST_LOG`'s value after the log.

          [default: info]

  -h, --help
          Print help information (use `-h` for a summary)
```

### Example
```sh
# Update WASM code of a chain spec file.
subalfred state insert chain-spec.json --key 0x3a636f6465 --with-file runtime.compact.compressed.wasm
```
