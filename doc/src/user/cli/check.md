# The Check Command
Currently, there are two subcommands available.

## Runtime
As the name says, the checks are on the runtime level.

There are three shared options:
- `--chain <NAME>`The chain name, which will supply to the executable.
- `--executable <PATH>` The executable path.
- `--live <URI>` The live chain's RPC HTTP endpoint.

And there are two properties to check, `storage` and `version`.

### Examples
For convenience, I use the [Pangolin Network](https://github.com/darwinia-network/darwinia-common) to demonstrate.

- `pangolin-dev` is the dev chain spec, which means its genesis is built from the latest code.
  If you are on Polkadot, then use `polkadot-dev`.
- `./drml` is the path to my local pangolin node executable.
- `https://pangolin-rpc.darwinia.network` is the Pangoro live chain's RPC HTTP endpoint.
  Note that, I use the Pangoro here. Just because I'm doing a demonstration.
  Compare with two different chains' runtime storage/version will get a lot of output.

#### Check Runtime Storage
```sh
subalfred check runtime --chain pangolin-dev --executable ./drml --live https://pangoro-rpc.darwinia.network --property storage
```
```diff
+ Pallet: "Bounties"
- Pallet: "BridgePangolinGrandpa"
- Pallet: "BridgePangolinMessages"
+ Pallet: "BridgePangolinParachainMessages"
+ Pallet: "BridgePangoroGrandpa"
+ Pallet: "BridgePangoroMessages"
+ Pallet: "BridgeRococoGrandpa"
+ Pallet: "BridgeRococoParachains"
+ Pallet: "Council"
+ Pallet: "DarwiniaEthereumRelay"
+ Pallet: "Democracy"
+ Pallet: "EcdsaRelayAuthority"
+ Pallet: "EthereumBacking"
+ Pallet: "HeaderMmr"
+ Pallet: "Identity"
+ Pallet: "Instance1DarwiniaRelayerGame"
+ Pallet: "KtonTreasury"
+ Pallet: "Multisig"
- Pallet: "PangolinFeeMarket"
+ Pallet: "PangolinParachainFeeMarket"
+ Pallet: "PangoroFeeMarket"
+ Pallet: "PhragmenElection"
+ Pallet: "Proxy"
+ Pallet: "Recovery"
+ Pallet: "Society"
- Pallet: "Substrate2SubstrateBacking"
+ Pallet: "Substrate2SubstrateIssuing"
+ Pallet: "TechnicalCommittee"
+ Pallet: "TechnicalMembership"
+ Pallet: "Tips"
+ Pallet: "ToPangolinParachainBacking"
+ Pallet: "Vesting"

Pallet ElectionProviderMultiPhase
+ Entry: StorageEntryMetadata { name: "SignedSubmissionsMap", modifier: Default, ty: Map { hashers: [Twox64Concat], key: UntrackedSymbol { id: 4, marker: PhantomData }, value: UntrackedSymbol { id: 227, marker: PhantomData } }, default: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], docs: [" Unchecked, signed solutions.", "", " Together with `SubmissionIndices`, this stores a bounded set of `SignedSubmissions` while", " allowing us to keep only a single one in memory at a time.", "", " Twox note: the key of the map is an auto-incrementing index which users cannot inspect or", " affect; we shouldn't need a cryptographically secure hasher."] }
- Entry: StorageEntryMetadata { name: "SignedSubmissionsMap", modifier: Default, ty: Map { hashers: [Twox64Concat], key: UntrackedSymbol { id: 4, marker: PhantomData }, value: UntrackedSymbol { id: 186, marker: PhantomData } }, default: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], docs: [" Unchecked, signed solutions.", "", " Together with `SubmissionIndices`, this stores a bounded set of `SignedSubmissions` while", " allowing us to keep only a single one in memory at a time.", "", " Twox note: the key of the map is an auto-incrementing index which users cannot inspect or", " affect; we shouldn't need a cryptographically secure hasher."] }

Pallet Scheduler
+ Entry: StorageEntryMetadata { name: "Agenda", modifier: Default, ty: Map { hashers: [Twox64Concat], key: UntrackedSymbol { id: 4, marker: PhantomData }, value: UntrackedSymbol { id: 609, marker: PhantomData } }, default: [0], docs: [" Items to be executed, indexed by the block number that they should be executed on."] }
- Entry: StorageEntryMetadata { name: "Agenda", modifier: Default, ty: Map { hashers: [Twox64Concat], key: UntrackedSymbol { id: 4, marker: PhantomData }, value: UntrackedSymbol { id: 378, marker: PhantomData } }, default: [0], docs: [" Items to be executed, indexed by the block number that they should be executed on."] }

Pallet Session
+ Entry: StorageEntryMetadata { name: "NextKeys", modifier: Optional, ty: Map { hashers: [Twox64Concat], key: UntrackedSymbol { id: 0, marker: PhantomData }, value: UntrackedSymbol { id: 347, marker: PhantomData } }, default: [0], docs: [" The next session keys for a validator."] }
- Entry: StorageEntryMetadata { name: "NextKeys", modifier: Optional, ty: Map { hashers: [Twox64Concat], key: UntrackedSymbol { id: 0, marker: PhantomData }, value: UntrackedSymbol { id: 282, marker: PhantomData } }, default: [0], docs: [" The next session keys for a validator."] }
+ Entry: StorageEntryMetadata { name: "QueuedKeys", modifier: Default, ty: Plain(UntrackedSymbol { id: 345, marker: PhantomData }), default: [0], docs: [" The queued keys for the next session. When the next session begins, these keys", " will be used to determine the validator's session keys."] }
- Entry: StorageEntryMetadata { name: "QueuedKeys", modifier: Default, ty: Plain(UntrackedSymbol { id: 280, marker: PhantomData }), default: [0], docs: [" The queued keys for the next session. When the next session begins, these keys", " will be used to determine the validator's session keys."] }

Pallet System
+ Entry: StorageEntryMetadata { name: "Events", modifier: Default, ty: Plain(UntrackedSymbol { id: 15, marker: PhantomData }), default: [0], docs: [" Events deposited for the current block.", "", " NOTE: This storage item is explicitly unbounded since it is never intended to be read", " from within the runtime."] }
- Entry: StorageEntryMetadata { name: "Events", modifier: Default, ty: Plain(UntrackedSymbol { id: 15, marker: PhantomData }), default: [0], docs: [" Events deposited for the current block.", "", " NOTE: This storage item is explicitly unbounded since it is never intended to be read", " from within the runtime."] }
```

#### Check Runtime Version
```sh
subalfred check runtime --chain pangolin-dev --executable ./drml --live https://pangoro-rpc.darwinia.network --property version
```
```diff
RuntimeVersion {
-   spec_name: "Pangoro",
+   spec_name: "Pangolin",
-   impl_name: "Pangoro",
+   impl_name: "Pangolin",
    authoring_version: 0,
    spec_version: 29020,
    impl_version: 0,
    transaction_version: 0,
}
```

## Features
You could check if all your runtime features' were enabled correctly in one command.

### Episode 1
As we know Substrate has two runtime ENVs, native and WASM.

If a runtime dependency is not pure no-std, we need to write:
```toml
[features]
std = ["xxx/std"]

[dependencies]
xxx = { version = "0.1.0", default-features = false }
```

Sometimes, we might forget to write add the `xxx/std`.

Recently, I found someone have the same [requirement](https://github.com/paritytech/substrate/pull/11715).

So, I decide to make this public.

### Episode 2
As time passed, more and more features were added to Substrate.

We have `std`, `runtime-benchmarks` and `try-runtime` now.

It's hard to check if all features are enabled correctly.

### Examples
> Testing commit [paritytech/polkadot@`0fd106c`](https://github.com/paritytech/polkadot/commit/0fd106c04e5f57f6342f8e000d471d0f819f7b61)
```sh
git clone https://github.com/paritytech/polkadot /tmp/paritytech/polkadot
subalfred check features --manifest-path /tmp/paritytech/polkadot/runtime/polkadot -ltrace
```
```
checking: /tmp/paritytech/polkadot/runtime/polkadot/Cargo.toml
2022-10-22T06:10:58.626261Z TRACE subalfred::core::check::features: check std takes 0.000408458 secs
2022-10-22T06:10:58.626636Z TRACE subalfred::core::check::features: check runtime-benchmarks takes 0.000177708 secs
2022-10-22T06:10:58.626764Z TRACE subalfred::core::check::features: check try-runtime takes 0.0001255 secs
`std` of `frame-benchmarking` was omitted
`std` of `pallet-election-provider-support-benchmarking` was omitted
`std` of `polkadot-primitives` was omitted
`std` of `polkadot-runtime-common` was omitted
`std` of `polkadot-runtime-parachains` was omitted
`std` of `serde_json` was omitted
`std` of `sp-authority-discovery` was omitted
`std` of `sp-block-builder` was omitted
`std` of `sp-consensus-babe` was omitted
`std` of `sp-inherents` was omitted
`std` of `sp-io` was omitted
`std` of `sp-offchain` was omitted
`std` of `sp-tracing` was omitted
`std` of `sp-transaction-pool` was omitted
`std` of `sp-trie` was omitted
`runtime-benchmarks` of `polkadot-primitives` was omitted
`runtime-benchmarks` of `polkadot-runtime-common` was omitted
`runtime-benchmarks` of `polkadot-runtime-parachains` was omitted
`runtime-benchmarks` of `sp-staking` was omitted
`runtime-benchmarks` of `xcm-executor` was omitted
`try-runtime` of `frame-support` was omitted
`try-runtime` of `polkadot-runtime-common` was omitted
`try-runtime` of `polkadot-runtime-parachains` was omitted
```

## CI
Moreover, we can add the checks into your project CI.

I've already add these to the [Darwinia CI](https://github.com/darwinia-network/darwinia/blob/v0.12.3/.github/workflows/ci.yml).
And here is a real world [example](https://github.com/darwinia-network/darwinia/pull/940#issuecomment-1226917895).
