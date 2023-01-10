# Command `track-updates`
```
Track the updates.

This command require a `GITHUB_TOKEN` environment variable to be set. It will list all the commits between the `from` and `to` GitHub ref.

The output is in markdown format.

Usage: subalfred track-updates [OPTIONS] --from <VERSION> --to <VERSION> <OWNER/REPOSITORY>

Arguments:
  <OWNER/REPOSITORY>
          Target repository.

          e.g. paritytech/substrate

Options:
      --from <VERSION>
          Release starting from

      --to <VERSION>
          Release updating to

  -l, --log <TARGET=LEVEL,*>
          Set a custom log filter.

          This flag is also working with the `RUST_LOG` environment variable. If you use `RUST_LOG`
          simultaneously, this will append `RUST_LOG`'s value after the log.

          [default: info]

  -h, --help
          Print help information (use `-h` for a summary)
```

## Example
We take `paritytech/substrate` as an example. If you want to track the updates of `cumulus` just replace the `paritytech/substrate` with `paritytech/cumulus`.

The `GITHUB_TOKEN` API key must have the privilege of the target repository.

This command will list all the commits and their labels between `polkadot-v0.9.33` and `polkadot-v0.9.36`.
And in the future, there will be a GitHub action, which will check the Polkadot updates and create an issue for that.

Currently, its output is a markdown. You can copy and paste it to create a new issue manually.

Moreover, there is an open [StackExchange question](https://substrate.stackexchange.com/questions/5884/how-to-play-with-the-substrate-labels/5903#5903). When paritytech finishes the refactoring of the labels.

Also, there are some discussion about this command [here](https://github.com/w3f/Grant-Milestone-Delivery/pull/629).

This command will provide a categorized update list. You can easily focus on specific parts.
```sh
export GITHUB_TOKEN=OMITTED
subalfred track-updates paritytech/substrate --from polkadot-v0.9.33 --to polkadot-v0.9.36
# or
GITHUB_TOKEN=OMITTED subalfred track-updates paritytech/substrate --from polkadot-v0.9.33 --to polkadot-v0.9.36
```

### Track updates
```
repository: paritytech/substrate
commits   : 112
command   : subalfred track-updates paritytech/substrate --from polkadot-v0.9.33 --to polkadot-v0.9.36
```
> https://github.com/paritytech/substrate/compare/polkadot-v0.9.33...polkadot-v0.9.36
### All
- [Contracts pallet: Bump Runtime API](https://github.com/paritytech/substrate/pull/12677) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D2-notlive 💤`
- [Fix typo in MultiSignature docs](https://github.com/paritytech/substrate/pull/12680) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Move `WeightCounter` to `sp-weights`](https://github.com/paritytech/substrate/pull/12603) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D1-audited 👍`
- [Allow other pallets to check asset ids.](https://github.com/paritytech/substrate/pull/12666) - `A0-pleasereview`, `B3-apinoteworthy`, `C1-low 📌`, `D3-trivial 🧸`
- [derive type info for some grandpa types](https://github.com/paritytech/substrate/pull/12683) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Safe TreeRoute constructor](https://github.com/paritytech/substrate/pull/12691) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [New `root_testing` pallet](https://github.com/paritytech/substrate/pull/12451) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `E6-transactionversion`, `D3-trivial 🧸`
- [[ci] Add DAG for build-rustdoc and check-dependent-project](https://github.com/paritytech/substrate/pull/12687) - `A3-inprogress`, `B0-silent`, `C1-low 📌`
- [Collective: Benchmark with greater `MaxProposals`](https://github.com/paritytech/substrate/pull/12454) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [[ci] fix buildah for publishing docker](https://github.com/paritytech/substrate/pull/12703) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`
- [Make public is_passing and ReferendumStatus](https://github.com/paritytech/substrate/pull/12667) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Asset Pallet: Support repeated destroys to safely destroy large assets](https://github.com/paritytech/substrate/pull/12310) - `A0-pleasereview`, `B7-runtimenoteworthy`, `E1-runtimemigration`, `C1-low 📌`, `D1-audited 👍`
- [`seal_reentrant_count` returns contract reentrant count](https://github.com/paritytech/substrate/pull/12695) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D2-notlive 💤`
- [Assets Pallet: reintroduce fungibles::Destroy trait ](https://github.com/paritytech/substrate/pull/12708) - `A0-pleasereview`, `B3-apinoteworthy`, `C1-low 📌`, `D3-trivial 🧸`
- [release `sp-core 7.0.0` and `sp-runtime 7.0.0`](https://github.com/paritytech/substrate/pull/12599) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Release `sp-keyring` and `pallet-contracts-primitives` `7.0.0`](https://github.com/paritytech/substrate/pull/12716) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Fix `cargo check` for `pallet-contracts-proc-macro`](https://github.com/paritytech/substrate/pull/12706) - `B5-clientnoteworthy`, `C1-low 📌`, `D3-trivial 🧸`
- [[ci] Improve pipeline stopper](https://github.com/paritytech/substrate/pull/12717) - `A3-inprogress`, `B0-silent`, `C1-low 📌`
- [sc-chainspec: Switch to `assimilate_storage`](https://github.com/paritytech/substrate/pull/12720) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [[Cleanup] Remove an obsolete event from fast-unstake](https://github.com/paritytech/substrate/pull/12725) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [[Fix] Deposit for fast-unstake has to be define as pallet::constant](https://github.com/paritytech/substrate/pull/12729) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Add event testing example to pallet template](https://github.com/paritytech/substrate/pull/12722) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Remove the `wasmtime` feature flag](https://github.com/paritytech/substrate/pull/12684) - `A0-pleasereview`, `B5-clientnoteworthy`, `C1-low 📌`, `D3-trivial 🧸`
- [Fix the light client protocol protobuf schema](https://github.com/paritytech/substrate/pull/12732) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`
- [Update template to remove clippy warnings](https://github.com/paritytech/substrate/pull/12670) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Check all crates](https://github.com/paritytech/substrate/pull/12709) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [client/beefy: persist voter state](https://github.com/paritytech/substrate/pull/12712) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D2-notlive 💤`
- [[Fix] Get target count from TargetList instead of storage](https://github.com/paritytech/substrate/pull/12748) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Move block/state/warpc sync requests/responses to `ChainSync`](https://github.com/paritytech/substrate/pull/12739) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`
- [perf: generate_initial_session_keys: load runtime only if its relevant](https://github.com/paritytech/substrate/pull/12651) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Prevent epochs pruning while finalizing blocks on epoch 0](https://github.com/paritytech/substrate/pull/12758) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`
- [return error instead of expect in `feasibility_check`](https://github.com/paritytech/substrate/pull/12745) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [BEEFY: optimize voter event loop for fewer 'active' wakeups](https://github.com/paritytech/substrate/pull/12760) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D2-notlive 💤`
- [Sort crates before splitting them into groups (+ some improvements)](https://github.com/paritytech/substrate/pull/12755) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D2-notlive 💤`
- [contracts: Replace `sp-sandbox` and `wasmi-validation` by newest wasmi](https://github.com/paritytech/substrate/pull/12501) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D2-notlive 💤`
- [Update DefaultNoBound derive macro](https://github.com/paritytech/substrate/pull/12723) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Fix rustdoc](https://github.com/paritytech/substrate/pull/12777) - `A2-insubstantial`, `A8-mergeoncegreen`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Allow Alliance Fellows to Give Up Voting Rights](https://github.com/paritytech/substrate/pull/12730) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D5-nicetohaveaudit ⚠️`
- [Add total nb to trie migration rpc](https://github.com/paritytech/substrate/pull/12770) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D2-notlive 💤`
- [add EnsureWithSuccess](https://github.com/paritytech/substrate/pull/12775) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Explicitly unset RUSTC_WRAPPER=sccache environment variable](https://github.com/paritytech/substrate/pull/12771) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`
- [contracts: Don't put unstable functions in special module](https://github.com/paritytech/substrate/pull/12781) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [ed25519_verify: Support using dalek for historical blocks](https://github.com/paritytech/substrate/pull/12661) - `A0-pleasereview`, `B5-clientnoteworthy`, `C1-low 📌`, `D3-trivial 🧸`
- [client/beefy: fix on-demand justifications sync for old blocks](https://github.com/paritytech/substrate/pull/12767) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D2-notlive 💤`
- [Remove Default, HasCompact, and TypeInfo trait bounds on AssetId](https://github.com/paritytech/substrate/pull/12740) - `A0-pleasereview`, `B3-apinoteworthy`, `C1-low 📌`, `E6-transactionversion`, `D3-trivial 🧸`
- [pallet-mmr: move offchain logic to client-side gadget](https://github.com/paritytech/substrate/pull/12753) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D2-notlive 💤`
- [Require rust-features check](https://github.com/paritytech/substrate/pull/12796) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [MMR: move RPC code from frame/ to client/](https://github.com/paritytech/substrate/pull/12805) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D2-notlive 💤`
- [chore: remove unused traits for wasm interface](https://github.com/paritytech/substrate/pull/12792) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [sc-transaction-handler: Fix potential crashes on exit](https://github.com/paritytech/substrate/pull/12807) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Don't announce blocks in `sync_to_tip_when_we_sync_together_with_multiple_peers`](https://github.com/paritytech/substrate/pull/12783) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [contracts: Replace cargo feature `unstable-interface` with config](https://github.com/paritytech/substrate/pull/12787) - `A0-pleasereview`, `B3-apinoteworthy`, `C1-low 📌`, `D3-trivial 🧸`
- [Bounties use SpendOrigin](https://github.com/paritytech/substrate/pull/12808) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D3-trivial 🧸`
- [Reduce provisioner work](https://github.com/paritytech/substrate/pull/12749) - `A0-pleasereview`, `B3-apinoteworthy`, `C1-low 📌`, `D3-trivial 🧸`, `D2-breaksapi`
- [Fix quantization in referenda alarm](https://github.com/paritytech/substrate/pull/12815) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Add `Weightless` benchmark bailing](https://github.com/paritytech/substrate/pull/12829) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D3-trivial 🧸`
- [API for registering inactive funds](https://github.com/paritytech/substrate/pull/12813) - `A0-pleasereview`, `B0-silent`, `E1-runtimemigration`, `C1-low 📌`, `D3-trivial 🧸`
- [Tweak to active total migrations](https://github.com/paritytech/substrate/pull/12832) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D2-notlive 💤`
- [frame-executive: Reject invalid inherents in the executive](https://github.com/paritytech/substrate/pull/12365) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D1-audited 👍`
- [Upgrade tokio to 1.22.0 and replace async-std with tokio](https://github.com/paritytech/substrate/pull/12646) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Make Gov2 Curve::threshold fn public](https://github.com/paritytech/substrate/pull/12814) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Non-Interactive Staking](https://github.com/paritytech/substrate/pull/12610) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D2-notlive 💤`
- [pallet-balances: Fix inactive funds migration](https://github.com/paritytech/substrate/pull/12840) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [client/beefy: add some bounds on enqueued votes](https://github.com/paritytech/substrate/pull/12562) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D2-notlive 💤`
- [OpenGov: Abstentions](https://github.com/paritytech/substrate/pull/12842) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D3-trivial 🧸`
- [Add `with_weight` extrinsic](https://github.com/paritytech/substrate/pull/12848) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D9-needsaudit 👮`
- [[contracts] Add per local weight for function call](https://github.com/paritytech/substrate/pull/12806) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [contracts: Add `instantiation_nonce` API](https://github.com/paritytech/substrate/pull/12800) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D9-needsaudit 👮`
- [Rename some crates for publishing to crates.io](https://github.com/paritytech/substrate/pull/12837) - `A2-insubstantial`, `B3-apinoteworthy`, `C1-low 📌`, `D3-trivial 🧸`
- [Whitelist pallet preimage provider upgrade](https://github.com/paritytech/substrate/pull/12834) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Remove `mem_info` and references to `parity-util-mem`](https://github.com/paritytech/substrate/pull/12795) - `A0-pleasereview`, `B5-clientnoteworthy`, `C1-low 📌`, `D3-trivial 🧸`
- [frame-support: Introduce `EnsureOriginOrHigherPrivilege`](https://github.com/paritytech/substrate/pull/12844) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Mmr persist state](https://github.com/paritytech/substrate/pull/12822) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D2-notlive 💤`
- [Refund referendum submission deposit](https://github.com/paritytech/substrate/pull/12788) - `B3-apinoteworthy`, `C1-low 📌`, `D3-trivial 🧸`
- [Remove sandboxing host function interface](https://github.com/paritytech/substrate/pull/12852) - `A0-pleasereview`, `B5-clientnoteworthy`, `C1-low 📌`, `E4-newhostfunctions`, `D3-trivial 🧸`
- [Referenda benchmark assert fix](https://github.com/paritytech/substrate/pull/12866) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Implement crate publishing on CI](https://github.com/paritytech/substrate/pull/12768) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [zombienet: warp-sync integration test added](https://github.com/paritytech/substrate/pull/12675) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Move LockableCurrency trait to fungibles::Lockable and deprecate LockableCurrency](https://github.com/paritytech/substrate/pull/12798) - `A0-pleasereview`, `B3-apinoteworthy`, `C1-low 📌`, `D5-nicetohaveaudit ⚠️`
- [[pallet-assets] add asset_exists(id: AssetId) function](https://github.com/paritytech/substrate/pull/12782) - `A0-pleasereview`, `B3-apinoteworthy`, `C1-low 📌`, `D3-trivial 🧸`
- [Introduce sensible weight constants](https://github.com/paritytech/substrate/pull/12868) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Checkout to the branch HEAD explicitly in `build-linux-substrate`](https://github.com/paritytech/substrate/pull/12876) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`
- [cli: Improve pruning documentation](https://github.com/paritytech/substrate/pull/12819) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Revert "Move LockableCurrency trait to fungibles::Lockable and deprecate LockableCurrency"](https://github.com/paritytech/substrate/pull/12882) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Don't indefinitely block on shutting down Tokio](https://github.com/paritytech/substrate/pull/12885) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [General Message Queue Pallet](https://github.com/paritytech/substrate/pull/12485) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D2-notlive 💤`
- [zombienet timings adjusted](https://github.com/paritytech/substrate/pull/12890) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Move import queue out of `sc-network`](https://github.com/paritytech/substrate/pull/12764) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`
- [Trace response payload in default `jsonrpsee` middleware](https://github.com/paritytech/substrate/pull/12886) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Ensure that we inform all tasks to stop before starting the 60 seconds shutdown](https://github.com/paritytech/substrate/pull/12897) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Safe desired targets call](https://github.com/paritytech/substrate/pull/12826) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Fix typo](https://github.com/paritytech/substrate/pull/12900) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [ValidateUnsigned: Improve docs.](https://github.com/paritytech/substrate/pull/12870) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [rpc server with HTTP/WS on the same socket](https://github.com/paritytech/substrate/pull/12663) - `A0-pleasereview`, `B5-clientnoteworthy`, `C1-low 📌`, `D3-trivial 🧸`
- [`pallet-message-queue`: Fix license](https://github.com/paritytech/substrate/pull/12895) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Use explicit call indices](https://github.com/paritytech/substrate/pull/12891) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Fix state-db race](https://github.com/paritytech/substrate/pull/12902) - `A0-pleasereview`, `B0-silent`, `C7-high ❗️`, `D3-trivial 🧸`
- [Remove implicit approval chilling upon slash.](https://github.com/paritytech/substrate/pull/12420) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D1-audited 👍`
- [pallet bounties calls docs fix](https://github.com/paritytech/substrate/pull/12909) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [pallet-contracts migration pre-upgrade fix for v8](https://github.com/paritytech/substrate/pull/12905) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Use custom environment for publishing crates](https://github.com/paritytech/substrate/pull/12912) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D2-notlive 💤`
- [[contracts] Add debug buffer limit + enforcement](https://github.com/paritytech/substrate/pull/12845) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Fixup some wrong dependencies](https://github.com/paritytech/substrate/pull/12899) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [add numerator and denominator to Rational128 Debug impl](https://github.com/paritytech/substrate/pull/12914) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Fix state-db pinning](https://github.com/paritytech/substrate/pull/12927) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`
- [[ci] add job switcher](https://github.com/paritytech/substrate/pull/12922) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`
- [Use LOG_TARGET in consensus related crates](https://github.com/paritytech/substrate/pull/12875) - `A0-pleasereview`, `B0-silent`, `B5-clientnoteworthy`, `C1-low 📌`, `D3-trivial 🧸`
- [Staking: store last `min-active-bond` on-chain](https://github.com/paritytech/substrate/pull/12889) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D3-trivial 🧸`
- [Try-runtime Revamp and Facelift](https://github.com/paritytech/substrate/pull/12537) - `A0-pleasereview`, `B3-apinoteworthy`, `C1-low 📌`, `D5-nicetohaveaudit ⚠️`
- [Automatic `withdraw_unbonded` upon `unbond` ](https://github.com/paritytech/substrate/pull/12582) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D1-audited 👍`
- [documentation: add BEEFY 'spec'](https://github.com/paritytech/substrate/pull/12920) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D2-notlive 💤`
- [Warn on missing `pallet::call_index`](https://github.com/paritytech/substrate/pull/12894) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D3-trivial 🧸`
### Watched labels
- #### B3-apinoteworthy
	- [Allow other pallets to check asset ids.](https://github.com/paritytech/substrate/pull/12666)
	- [Assets Pallet: reintroduce fungibles::Destroy trait ](https://github.com/paritytech/substrate/pull/12708)
	- [Remove Default, HasCompact, and TypeInfo trait bounds on AssetId](https://github.com/paritytech/substrate/pull/12740)
	- [contracts: Replace cargo feature `unstable-interface` with config](https://github.com/paritytech/substrate/pull/12787)
	- [Reduce provisioner work](https://github.com/paritytech/substrate/pull/12749)
	- [Rename some crates for publishing to crates.io](https://github.com/paritytech/substrate/pull/12837)
	- [Refund referendum submission deposit](https://github.com/paritytech/substrate/pull/12788)
	- [Move LockableCurrency trait to fungibles::Lockable and deprecate LockableCurrency](https://github.com/paritytech/substrate/pull/12798)
	- [[pallet-assets] add asset_exists(id: AssetId) function](https://github.com/paritytech/substrate/pull/12782)
	- [Try-runtime Revamp and Facelift](https://github.com/paritytech/substrate/pull/12537)
- #### B5-clientnoteworthy
	- [Fix `cargo check` for `pallet-contracts-proc-macro`](https://github.com/paritytech/substrate/pull/12706)
	- [Remove the `wasmtime` feature flag](https://github.com/paritytech/substrate/pull/12684)
	- [ed25519_verify: Support using dalek for historical blocks](https://github.com/paritytech/substrate/pull/12661)
	- [Remove `mem_info` and references to `parity-util-mem`](https://github.com/paritytech/substrate/pull/12795)
	- [Remove sandboxing host function interface](https://github.com/paritytech/substrate/pull/12852)
	- [rpc server with HTTP/WS on the same socket](https://github.com/paritytech/substrate/pull/12663)
	- [Use LOG_TARGET in consensus related crates](https://github.com/paritytech/substrate/pull/12875)
- #### B7-runtimenoteworthy
	- [Contracts pallet: Bump Runtime API](https://github.com/paritytech/substrate/pull/12677)
	- [Asset Pallet: Support repeated destroys to safely destroy large assets](https://github.com/paritytech/substrate/pull/12310)
	- [`seal_reentrant_count` returns contract reentrant count](https://github.com/paritytech/substrate/pull/12695)
	- [contracts: Replace `sp-sandbox` and `wasmi-validation` by newest wasmi](https://github.com/paritytech/substrate/pull/12501)
	- [Allow Alliance Fellows to Give Up Voting Rights](https://github.com/paritytech/substrate/pull/12730)
	- [Bounties use SpendOrigin](https://github.com/paritytech/substrate/pull/12808)
	- [Add `Weightless` benchmark bailing](https://github.com/paritytech/substrate/pull/12829)
	- [Non-Interactive Staking](https://github.com/paritytech/substrate/pull/12610)
	- [OpenGov: Abstentions](https://github.com/paritytech/substrate/pull/12842)
	- [Add `with_weight` extrinsic](https://github.com/paritytech/substrate/pull/12848)
	- [contracts: Add `instantiation_nonce` API](https://github.com/paritytech/substrate/pull/12800)
	- [General Message Queue Pallet](https://github.com/paritytech/substrate/pull/12485)
	- [Remove implicit approval chilling upon slash.](https://github.com/paritytech/substrate/pull/12420)
	- [Staking: store last `min-active-bond` on-chain](https://github.com/paritytech/substrate/pull/12889)
	- [Automatic `withdraw_unbonded` upon `unbond` ](https://github.com/paritytech/substrate/pull/12582)
	- [Warn on missing `pallet::call_index`](https://github.com/paritytech/substrate/pull/12894)
- #### C7-high ❗️
	- [Fix state-db race](https://github.com/paritytech/substrate/pull/12902)
- #### E1-runtimemigration
	- [Asset Pallet: Support repeated destroys to safely destroy large assets](https://github.com/paritytech/substrate/pull/12310)
	- [API for registering inactive funds](https://github.com/paritytech/substrate/pull/12813)
- #### E4-newhostfunctions
	- [Remove sandboxing host function interface](https://github.com/paritytech/substrate/pull/12852)
- #### E6-transactionversion
	- [New `root_testing` pallet](https://github.com/paritytech/substrate/pull/12451)
	- [Remove Default, HasCompact, and TypeInfo trait bounds on AssetId](https://github.com/paritytech/substrate/pull/12740)
