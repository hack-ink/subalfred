# Command `check`
```
Substrate development checkers

Usage: subalfred check [OPTIONS] <COMMAND>

Commands:
  runtime
          Compare the local node's runtime version with the live's one
  features
          Check if the crates' features are enabled correctly
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
```

## Command `check runtime`
```
Compare the local node's runtime version with the live's one

Usage: subalfred check runtime [OPTIONS] --executable <PATH> --chain <CHAIN> --live <URI> --property <PROPERTY>

Options:
      --executable <PATH>
          Node executable's path

      --chain <CHAIN>
          Pass this name to `--chain` to launch the local chain

      --live <URI>
          Live chain's HTTP RPC endpoint

      --property <PROPERTY>
          Target property

          [possible values: storage, version]

  -l, --log <TARGET=LEVEL,*>
          Set a custom log filter.

          This flag is also working with the `RUST_LOG` environment variable. If you use `RUST_LOG`
          simultaneously, this will append `RUST_LOG`'s value after the log.

          [default: info]

  -h, --help
          Print help information (use `-h` for a summary)
```

### Example
We assume node-template's live chain is Polkadot here.

#### ENV preparation
```sh
git clone https://github.com/substrate-developer-hub/substrate-node-template.git /tmp/subalfred-example/substrate-node-template
cd /tmp/subalfred-example/substrate-node-template
cargo build
```

#### Command `check runtime --property storage`
```sh
subalfred check runtime --chain dev --executable target/debug/node-template --live https://rpc.polkadot.io --property storage
```
```diff
- Pallet: "Auctions"
+ Pallet: "Aura"
- Pallet: "Authorship"
- Pallet: "Babe"
- Pallet: "Bounties"
- Pallet: "ChildBounties"
- Pallet: "Claims"
- Pallet: "Configuration"
- Pallet: "Council"
- Pallet: "Crowdloan"
- Pallet: "Democracy"
- Pallet: "Dmp"
- Pallet: "ElectionProviderMultiPhase"
- Pallet: "Hrmp"
- Pallet: "Identity"
- Pallet: "ImOnline"
- Pallet: "Indices"
- Pallet: "Initializer"
- Pallet: "Multisig"
- Pallet: "NominationPools"
- Pallet: "Offences"
- Pallet: "ParaInclusion"
- Pallet: "ParaInherent"
- Pallet: "ParaScheduler"
- Pallet: "ParaSessionInfo"
- Pallet: "Paras"
- Pallet: "ParasDisputes"
- Pallet: "ParasShared"
- Pallet: "PhragmenElection"
- Pallet: "Preimage"
- Pallet: "Proxy"
+ Pallet: "RandomnessCollectiveFlip"
- Pallet: "Registrar"
- Pallet: "Scheduler"
- Pallet: "Session"
- Pallet: "Slots"
- Pallet: "Staking"
+ Pallet: "Sudo"
- Pallet: "TechnicalCommittee"
- Pallet: "TechnicalMembership"
+ Pallet: "TemplateModule"
- Pallet: "Tips"
- Pallet: "Treasury"
- Pallet: "Ump"
- Pallet: "Vesting"
- Pallet: "VoterList"
- Pallet: "XcmPallet"

Pallet Balances
+ Entry: StorageEntryMetadata { name: "Locks", modifier: Default, ty: Map { hashers: [Blake2_128Concat], key: UntrackedSymbol { id: 0, marker: PhantomData }, value: UntrackedSymbol { id: 94, marker: PhantomData } }, default: [0], docs: [" Any liquidity locks on some account balances.", " NOTE: Should only be accessed when setting, changing and freeing a lock."] }
- Entry: StorageEntryMetadata { name: "Locks", modifier: Default, ty: Map { hashers: [Blake2_128Concat], key: UntrackedSymbol { id: 0, marker: PhantomData }, value: UntrackedSymbol { id: 467, marker: PhantomData } }, default: [0], docs: [" Any liquidity locks on some account balances.", " NOTE: Should only be accessed when setting, changing and freeing a lock."] }
+ Entry: StorageEntryMetadata { name: "Reserves", modifier: Default, ty: Map { hashers: [Blake2_128Concat], key: UntrackedSymbol { id: 0, marker: PhantomData }, value: UntrackedSymbol { id: 98, marker: PhantomData } }, default: [0], docs: [" Named reserves on some account balances."] }
- Entry: StorageEntryMetadata { name: "Reserves", modifier: Default, ty: Map { hashers: [Blake2_128Concat], key: UntrackedSymbol { id: 0, marker: PhantomData }, value: UntrackedSymbol { id: 471, marker: PhantomData } }, default: [0], docs: [" Named reserves on some account balances."] }

Pallet Grandpa
+ Entry: StorageEntryMetadata { name: "PendingChange", modifier: Optional, ty: Plain(UntrackedSymbol { id: 77, marker: PhantomData }), default: [0], docs: [" Pending change: (signaled at, scheduled change)."] }
- Entry: StorageEntryMetadata { name: "PendingChange", modifier: Optional, ty: Plain(UntrackedSymbol { id: 513, marker: PhantomData }), default: [0], docs: [" Pending change: (signaled at, scheduled change)."] }

Pallet System
+ Entry: StorageEntryMetadata { name: "BlockWeight", modifier: Default, ty: Plain(UntrackedSymbol { id: 7, marker: PhantomData }), default: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], docs: [" The current weight for the block."] }
- Entry: StorageEntryMetadata { name: "BlockWeight", modifier: Default, ty: Plain(UntrackedSymbol { id: 7, marker: PhantomData }), default: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], docs: [" The current weight for the block."] }
+ Entry: StorageEntryMetadata { name: "Events", modifier: Default, ty: Plain(UntrackedSymbol { id: 16, marker: PhantomData }), default: [0], docs: [" Events deposited for the current block.", "", " NOTE: The item is unbound and should therefore never be read on chain.", " It could otherwise inflate the PoV size of a block.", "", " Events have a large in-memory size. Box the events to not go out-of-memory", " just in case someone still reads them from within the runtime."] }
- Entry: StorageEntryMetadata { name: "Events", modifier: Default, ty: Plain(UntrackedSymbol { id: 16, marker: PhantomData }), default: [0], docs: [" Events deposited for the current block.", "", " NOTE: The item is unbound and should therefore never be read on chain.", " It could otherwise inflate the PoV size of a block.", "", " Events have a large in-memory size. Box the events to not go out-of-memory", " just in case someone still reads them from within the runtime."] }
```

#### Command `check runtime --property version`
```sh
subalfred check runtime --chain dev --executable target/debug/substrate-node-template --live https://rpc.polkadot.io --property version
```
```diff
RuntimeVersion {
-   spec_name: "polkadot",
+   spec_name: "node-template",
-   impl_name: "parity-polkadot",
+   impl_name: "node-template",
-   authoring_version: 0,
+   authoring_version: 1,
-   spec_version: 9291,
+   spec_version: 100,
-   impl_version: 0,
+   impl_version: 1,
-   transaction_version: 14,
+   transaction_version: 1,
-   state_version: 0,
+   state_version: 1,
}
```

## Features
```
Check if the crates' features are enabled correctly

Usage: subalfred check features [OPTIONS] [PATH]

Arguments:
  [PATH]
          Root `Cargo.toml`'s path.

          If `Cargo.toml` wasn't given, Subalfred will search it under the given path.

          [default: ./Cargo.toml]

Options:
  -l, --log <TARGET=LEVEL,*>
          Set a custom log filter.

          This flag is also working with the `RUST_LOG` environment variable. If you use `RUST_LOG` simultaneously, this will append `RUST_LOG`'s
          value after the log.

          [default: info]

  -h, --help
          Print help information (use `-h` for a summary)
```

### Episode 1
As we know Substrate has two runtime ENVs, native and WASM.
If a runtime dependency is not pure no-std, we need to write:
```toml
[features]
std = ["pallet/std"]

[dependencies]
pallet = { version = "0.1.0", default-features = false }
```

Sometimes, we might forget to write add the `pallet/std`.
Recently, I found someone have the same [requirement](https://github.com/paritytech/substrate/pull/11715).
So, I decide to make this tool.

### Episode 2
As time passed, more and more features were added to Substrate.
We have `std`, `runtime-benchmarks` and `try-runtime` now.
It's hard to check if all features are enabled correctly.

### Example
```sh
git clone https://github.com/paritytech/polkadot /tmp/subalfred-example/polkadot
cd /tmp/subalfred-example/polkadot
git checkout 0fd106c04e5f57f6342f8e000d471d0f819f7b61
subalfred check features runtime/polkadot -lsubalfred
```
```
checking: runtime/polkadot/Cargo.toml
2022-11-02T17:50:17.766228Z TRACE subalfred_core::check::features: check::features::try-runtime takes 0.000155583 secs
2022-11-02T17:50:17.766669Z TRACE subalfred_core::check::features: check::features::std takes 0.000417541 secs
2022-11-02T17:50:17.766846Z TRACE subalfred_core::check::features: check::features::runtime-benchmarks takes 0.000169125 secs
incomplete `try-runtime` of `frame-support`
incomplete `try-runtime` of `runtime-parachains`
incomplete `std` of `frame-benchmarking`
incomplete `std` of `frame-system-benchmarking`
incomplete `std` of `pallet-election-provider-support-benchmarking`
incomplete `std` of `pallet-nomination-pools-benchmarking`
incomplete `std` of `pallet-offences-benchmarking`
incomplete `std` of `pallet-session-benchmarking`
incomplete `std` of `runtime-parachains`
incomplete `std` of `sp-io`
incomplete `runtime-benchmarks` of `primitives`
incomplete `runtime-benchmarks` of `sp-staking`
incomplete `runtime-benchmarks` of `xcm-executor`
```

## CI
Moreover, we can add the checks into your project CI.

I've already add these to the [Darwinia CI](https://github.com/darwinia-network/darwinia/blob/v0.12.3/.github/workflows/ci.yml).
And here is the result [example](https://github.com/darwinia-network/darwinia/pull/940#issuecomment-1226917895).
