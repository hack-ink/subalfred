# Command `track-update`
```
Track the updates.

This command require a `GITHUB_TOKEN` environment variable to be set. It will list all the commits between the `from` and `to` GitHub ref.

The output is in markdown format.

Usage: subalfred track-update [OPTIONS] --from <VERSION> --to <VERSION> <OWNER/REPOSITORY>

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

This command will list all the commits and their labels between `polkadot-v0.9.32` and `polkadot-v0.9.33`.
And in the future, there will be a GitHub action, which will check the Polkadot updates and create an issue for that.

Currently, its output is a markdown. You can copy and paste it to create a new issue manually.

Moreover, there is an open [StackExchange question](https://substrate.stackexchange.com/questions/5884/how-to-play-with-the-substrate-labels/5903#5903).
When paritytech finishes the refactoring of the labels.
This command will provide a categorized update list. You can easily focus on specific parts.
```sh
export GITHUB_TOKEN=OMITTED
subalfred track-update paritytech/substrate --from polkadot-v0.9.32 --to polkadot-v0.9.33
# or
GITHUB_TOKEN=OMITTED subalfred track-update paritytech/substrate --from polkadot-v0.9.32 --to polkadot-v0.9.33
```
### Output in source blob style
```
- [BlockId removal: refactor: ProofProvider](https://github.com/paritytech/substrate/pull/12519) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [registrar: Avoid freebies in provide_judgement](https://github.com/paritytech/substrate/pull/12465) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D1-audited 👍`
- [EPM: allow duplicate submissions](https://github.com/paritytech/substrate/pull/12237) - `A0-pleasereview`, `B0-silent`, `E1-runtimemigration`, `C1-low 📌`, `D9-needsaudit 👮`
- [CI check against Rust feature bleed](https://github.com/paritytech/substrate/pull/12341) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [contracts: Decrease the interation count on slow benchmarks](https://github.com/paritytech/substrate/pull/12526) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D2-notlive 💤`
- [BlockId removal: refactor: Finalizer](https://github.com/paritytech/substrate/pull/12528) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [BlockId removal: refactor: BlockImportOperation+Bknd::finalize_block](https://github.com/paritytech/substrate/pull/12535) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Remove multiple DHTs support from `Discovery`](https://github.com/paritytech/substrate/pull/12524) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [CI: Enable debug assertions in Wasmer sandbox test](https://github.com/paritytech/substrate/pull/12540) - `A0-pleasereview`, `A8-mergeoncegreen`, `B0-silent`, `C1-low 📌`, `D2-notlive 💤`
- [Force base weights to be the minimum only when the intercept is negative](https://github.com/paritytech/substrate/pull/12482) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Add `DefensiveTruncateFrom`](https://github.com/paritytech/substrate/pull/12515) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D5-nicetohaveaudit ⚠️`
- [Refactor service tests in `sc-network`](https://github.com/paritytech/substrate/pull/12517) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Actually fix major sync detection](https://github.com/paritytech/substrate/pull/12114) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [BlockId removal: refactor: Backend::begin_state_operation](https://github.com/paritytech/substrate/pull/12541) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Use headers on weight templates](https://github.com/paritytech/substrate/pull/12546) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Make `NetworkService` callable for `ChainSync`](https://github.com/paritytech/substrate/pull/12542) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Use genesis hash- and fork ID-based protocol name for Kademlia](https://github.com/paritytech/substrate/pull/12545) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [contracts: Allow indeterministic instructions off-chain](https://github.com/paritytech/substrate/pull/12469) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D2-notlive 💤`
- [Normalize keystore type and its usage across tests](https://github.com/paritytech/substrate/pull/12553) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`
- [Update template pallet to latest enum syntax](https://github.com/paritytech/substrate/pull/12552) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [feat: generalize some functions in sp-trie](https://github.com/paritytech/substrate/pull/12376) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Make Multisig Pallet Bounded](https://github.com/paritytech/substrate/pull/12457) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D1-audited 👍`
- [Fix error during build: failed to run custom build command for sc-net…](https://github.com/paritytech/substrate/pull/12494) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Update `pallet-multisig` benches](https://github.com/paritytech/substrate/pull/12558) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D3-trivial 🧸`
- [[ci] cargo-check-benches against different base branches](https://github.com/paritytech/substrate/pull/12557) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`
- [Fix typo in `membership` docs](https://github.com/paritytech/substrate/pull/12571) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [replaced println with log Closes #12338](https://github.com/paritytech/substrate/pull/12348) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Aura: Adds some compatibility mode to support old chains](https://github.com/paritytech/substrate/pull/12492) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [bump ed25519-zebra; fixes `full_crypto` feature flag in `no_std`](https://github.com/paritytech/substrate/pull/12576) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Utility: add more tests for batch/batchAll/forceBatch](https://github.com/paritytech/substrate/pull/12506) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Treat near-zero intercept values as zero when calculating weights](https://github.com/paritytech/substrate/pull/12573) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [[Enhancement] Convert fast-unstake to use StakingInterface, decouplin…](https://github.com/paritytech/substrate/pull/12424) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [nomination-pools: allow pool-ids to be reused](https://github.com/paritytech/substrate/pull/12407) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D5-nicetohaveaudit ⚠️`
- [WIP: Replace `wasm-gc` with `wasm-opt`](https://github.com/paritytech/substrate/pull/12280) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Use `minimum_nominator_bond` instead of `nominator_bond`](https://github.com/paritytech/substrate/pull/12585) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [BlockId removal: refactor: Backend::append_justification](https://github.com/paritytech/substrate/pull/12551) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Gov2 Documentation Typos](https://github.com/paritytech/substrate/pull/12584) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Added test for Client::block](https://github.com/paritytech/substrate/pull/12590) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [client/beefy: fix incorrect BEEFY justifications import test](https://github.com/paritytech/substrate/pull/12593) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D2-notlive 💤`
- [BlockId removal: refactor: Backend::body](https://github.com/paritytech/substrate/pull/12587) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [fix: construct_runtime multiple features](https://github.com/paritytech/substrate/pull/12594) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D5-nicetohaveaudit ⚠️`
- [Fix fungible unbalanced trait](https://github.com/paritytech/substrate/pull/12569) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D1-audited 👍`
- [[ci] allow fail skip-if-draft job](https://github.com/paritytech/substrate/pull/12604) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`
- [BlockId removal: refactor: Backend::justifications](https://github.com/paritytech/substrate/pull/12602) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [use associated iterator types for InspectEnumerable](https://github.com/paritytech/substrate/pull/12389) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Add map and try_map methods to BoundedBTreeMap](https://github.com/paritytech/substrate/pull/12581) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [[contracts] stabilize four storage host functions](https://github.com/paritytech/substrate/pull/12611) - `A0-pleasereview`, `B3-apinoteworthy`, `C1-low 📌`, `D3-trivial 🧸`
- [Collective benchmark respects DefaultVote configuration](https://github.com/paritytech/substrate/pull/12612) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [BlockId removal: refactor: Backend::block_indexed_body](https://github.com/paritytech/substrate/pull/12609) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Introduce DefensiveMin and DefensiveMax](https://github.com/paritytech/substrate/pull/12554) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [pallet-sudo: add `CheckOnlySudoAccount` signed extension](https://github.com/paritytech/substrate/pull/12496) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D5-nicetohaveaudit ⚠️`
- [Move Throughput into `sc-sysinfo`](https://github.com/paritytech/substrate/pull/12368) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Bump `k256` from `0.10.4` to `0.11.4`](https://github.com/paritytech/substrate/pull/12085) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Guard some invalid node for proof decoding.](https://github.com/paritytech/substrate/pull/12417) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Bump regex from 1.5.5 to 1.6.0](https://github.com/paritytech/substrate/pull/12117) - `A2-insubstantial`, `A3-stale`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Make `--db` case insensitive again](https://github.com/paritytech/substrate/pull/12630) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [txpool: enactment state forced update](https://github.com/paritytech/substrate/pull/12632) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Add pallet dev mode](https://github.com/paritytech/substrate/pull/12536) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D3-trivial 🧸`
- [BlockId removal: &Hash to Hash](https://github.com/paritytech/substrate/pull/12626) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Do not update peer information if ancestor search is in progress](https://github.com/paritytech/substrate/pull/12631) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Pipeline with ci image with rust 1.65](https://github.com/paritytech/substrate/pull/12628) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [`sp_trie::Recorder`: Fix recording the same key for different tries](https://github.com/paritytech/substrate/pull/12636) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Fix UI tests](https://github.com/paritytech/substrate/pull/12642) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D2-notlive 💤`
- [`payment_queryInfo`: Make it work with `WeightV2`](https://github.com/paritytech/substrate/pull/12633) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [State-db refactoring](https://github.com/paritytech/substrate/pull/12239) - `A0-pleasereview`, `B0-silent`, `C3-medium 📣`
- [Machine metrics: remove duplicate units](https://github.com/paritytech/substrate/pull/12634) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Add batching to fast-unstake pallet](https://github.com/paritytech/substrate/pull/12394) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D9-needsaudit 👮`
- [[ci] Use ci-linux:production image in ci](https://github.com/paritytech/substrate/pull/12648) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`
- [New Weights for All Pallets](https://github.com/paritytech/substrate/pull/12325) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D3-trivial 🧸`
- [Remove partial key size limit from trie codec](https://github.com/paritytech/substrate/pull/12566) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Keep the same type of name](https://github.com/paritytech/substrate/pull/12616) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Do not finalize parent twice](https://github.com/paritytech/substrate/pull/12653) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [update paritydb and remove dev deps on rocksdb](https://github.com/paritytech/substrate/pull/12641) - `A3-inprogress`, `B5-clientnoteworthy`, `C1-low 📌`, `D2-notlive 💤`
- [Epoch-Changes tree pruning was lagging by one epoch](https://github.com/paritytech/substrate/pull/12567) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D2-notlive 💤`
- [Bound Election and Staking by MaxActiveValidators](https://github.com/paritytech/substrate/pull/12436) - `A0-pleasereview`, `B3-apinoteworthy`, `C1-low 📌`, `D3-trivial 🧸`
- [Update some dependencies to prune duplicated crates with different version](https://github.com/paritytech/substrate/pull/12560) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Consolidate and deduplicate MMR API methods](https://github.com/paritytech/substrate/pull/12530) - `A0-pleasereview`, `B3-apinoteworthy`, `C1-low 📌`, `D2-notlive 💤`, `D2-breaksapi`
- [Bump ss58-registry from 1.29.0 to 1.34.0](https://github.com/paritytech/substrate/pull/12659) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Add `CreateOrigin` to Assets Pallet](https://github.com/paritytech/substrate/pull/12586) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D1-audited 👍`
- [`sp-runtime`: make `parity-util-mem` dependency optional](https://github.com/paritytech/substrate/pull/12657) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [GrandpaJustification: Feature gate `Debug`](https://github.com/paritytech/substrate/pull/12664) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [More testing and fuzzing and docs for pools](https://github.com/paritytech/substrate/pull/12624) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D2-notlive 💤`
- [Remove `sp_tasks::spawn` API and related code + host functions](https://github.com/paritytech/substrate/pull/12639) - `A0-pleasereview`, `B3-apinoteworthy`, `B5-clientnoteworthy`, `C1-low 📌`, `E4-newhostfunctions`, `D3-trivial 🧸`
- [Backport "Contracts pallet: Bump Runtime API (#12677)" to 0.9.33 branch](https://github.com/paritytech/substrate/pull/12686)
```
### Output in rendered blob style
- [BlockId removal: refactor: ProofProvider](https://github.com/paritytech/substrate/pull/12519) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [registrar: Avoid freebies in provide_judgement](https://github.com/paritytech/substrate/pull/12465) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D1-audited 👍`
- [EPM: allow duplicate submissions](https://github.com/paritytech/substrate/pull/12237) - `A0-pleasereview`, `B0-silent`, `E1-runtimemigration`, `C1-low 📌`, `D9-needsaudit 👮`
- [CI check against Rust feature bleed](https://github.com/paritytech/substrate/pull/12341) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [contracts: Decrease the interation count on slow benchmarks](https://github.com/paritytech/substrate/pull/12526) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D2-notlive 💤`
- [BlockId removal: refactor: Finalizer](https://github.com/paritytech/substrate/pull/12528) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [BlockId removal: refactor: BlockImportOperation+Bknd::finalize_block](https://github.com/paritytech/substrate/pull/12535) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Remove multiple DHTs support from `Discovery`](https://github.com/paritytech/substrate/pull/12524) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [CI: Enable debug assertions in Wasmer sandbox test](https://github.com/paritytech/substrate/pull/12540) - `A0-pleasereview`, `A8-mergeoncegreen`, `B0-silent`, `C1-low 📌`, `D2-notlive 💤`
- [Force base weights to be the minimum only when the intercept is negative](https://github.com/paritytech/substrate/pull/12482) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Add `DefensiveTruncateFrom`](https://github.com/paritytech/substrate/pull/12515) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D5-nicetohaveaudit ⚠️`
- [Refactor service tests in `sc-network`](https://github.com/paritytech/substrate/pull/12517) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Actually fix major sync detection](https://github.com/paritytech/substrate/pull/12114) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [BlockId removal: refactor: Backend::begin_state_operation](https://github.com/paritytech/substrate/pull/12541) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Use headers on weight templates](https://github.com/paritytech/substrate/pull/12546) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Make `NetworkService` callable for `ChainSync`](https://github.com/paritytech/substrate/pull/12542) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Use genesis hash- and fork ID-based protocol name for Kademlia](https://github.com/paritytech/substrate/pull/12545) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [contracts: Allow indeterministic instructions off-chain](https://github.com/paritytech/substrate/pull/12469) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D2-notlive 💤`
- [Normalize keystore type and its usage across tests](https://github.com/paritytech/substrate/pull/12553) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`
- [Update template pallet to latest enum syntax](https://github.com/paritytech/substrate/pull/12552) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [feat: generalize some functions in sp-trie](https://github.com/paritytech/substrate/pull/12376) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Make Multisig Pallet Bounded](https://github.com/paritytech/substrate/pull/12457) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D1-audited 👍`
- [Fix error during build: failed to run custom build command for sc-net…](https://github.com/paritytech/substrate/pull/12494) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Update `pallet-multisig` benches](https://github.com/paritytech/substrate/pull/12558) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D3-trivial 🧸`
- [[ci] cargo-check-benches against different base branches](https://github.com/paritytech/substrate/pull/12557) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`
- [Fix typo in `membership` docs](https://github.com/paritytech/substrate/pull/12571) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [replaced println with log Closes #12338](https://github.com/paritytech/substrate/pull/12348) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Aura: Adds some compatibility mode to support old chains](https://github.com/paritytech/substrate/pull/12492) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [bump ed25519-zebra; fixes `full_crypto` feature flag in `no_std`](https://github.com/paritytech/substrate/pull/12576) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Utility: add more tests for batch/batchAll/forceBatch](https://github.com/paritytech/substrate/pull/12506) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Treat near-zero intercept values as zero when calculating weights](https://github.com/paritytech/substrate/pull/12573) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [[Enhancement] Convert fast-unstake to use StakingInterface, decouplin…](https://github.com/paritytech/substrate/pull/12424) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [nomination-pools: allow pool-ids to be reused](https://github.com/paritytech/substrate/pull/12407) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D5-nicetohaveaudit ⚠️`
- [WIP: Replace `wasm-gc` with `wasm-opt`](https://github.com/paritytech/substrate/pull/12280) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Use `minimum_nominator_bond` instead of `nominator_bond`](https://github.com/paritytech/substrate/pull/12585) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [BlockId removal: refactor: Backend::append_justification](https://github.com/paritytech/substrate/pull/12551) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Gov2 Documentation Typos](https://github.com/paritytech/substrate/pull/12584) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Added test for Client::block](https://github.com/paritytech/substrate/pull/12590) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [client/beefy: fix incorrect BEEFY justifications import test](https://github.com/paritytech/substrate/pull/12593) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D2-notlive 💤`
- [BlockId removal: refactor: Backend::body](https://github.com/paritytech/substrate/pull/12587) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [fix: construct_runtime multiple features](https://github.com/paritytech/substrate/pull/12594) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D5-nicetohaveaudit ⚠️`
- [Fix fungible unbalanced trait](https://github.com/paritytech/substrate/pull/12569) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D1-audited 👍`
- [[ci] allow fail skip-if-draft job](https://github.com/paritytech/substrate/pull/12604) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`
- [BlockId removal: refactor: Backend::justifications](https://github.com/paritytech/substrate/pull/12602) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [use associated iterator types for InspectEnumerable](https://github.com/paritytech/substrate/pull/12389) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Add map and try_map methods to BoundedBTreeMap](https://github.com/paritytech/substrate/pull/12581) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [[contracts] stabilize four storage host functions](https://github.com/paritytech/substrate/pull/12611) - `A0-pleasereview`, `B3-apinoteworthy`, `C1-low 📌`, `D3-trivial 🧸`
- [Collective benchmark respects DefaultVote configuration](https://github.com/paritytech/substrate/pull/12612) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [BlockId removal: refactor: Backend::block_indexed_body](https://github.com/paritytech/substrate/pull/12609) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Introduce DefensiveMin and DefensiveMax](https://github.com/paritytech/substrate/pull/12554) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [pallet-sudo: add `CheckOnlySudoAccount` signed extension](https://github.com/paritytech/substrate/pull/12496) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D5-nicetohaveaudit ⚠️`
- [Move Throughput into `sc-sysinfo`](https://github.com/paritytech/substrate/pull/12368) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Bump `k256` from `0.10.4` to `0.11.4`](https://github.com/paritytech/substrate/pull/12085) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Guard some invalid node for proof decoding.](https://github.com/paritytech/substrate/pull/12417) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Bump regex from 1.5.5 to 1.6.0](https://github.com/paritytech/substrate/pull/12117) - `A2-insubstantial`, `A3-stale`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Make `--db` case insensitive again](https://github.com/paritytech/substrate/pull/12630) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [txpool: enactment state forced update](https://github.com/paritytech/substrate/pull/12632) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Add pallet dev mode](https://github.com/paritytech/substrate/pull/12536) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D3-trivial 🧸`
- [BlockId removal: &Hash to Hash](https://github.com/paritytech/substrate/pull/12626) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Do not update peer information if ancestor search is in progress](https://github.com/paritytech/substrate/pull/12631) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Pipeline with ci image with rust 1.65](https://github.com/paritytech/substrate/pull/12628) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [`sp_trie::Recorder`: Fix recording the same key for different tries](https://github.com/paritytech/substrate/pull/12636) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Fix UI tests](https://github.com/paritytech/substrate/pull/12642) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D2-notlive 💤`
- [`payment_queryInfo`: Make it work with `WeightV2`](https://github.com/paritytech/substrate/pull/12633) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [State-db refactoring](https://github.com/paritytech/substrate/pull/12239) - `A0-pleasereview`, `B0-silent`, `C3-medium 📣`
- [Machine metrics: remove duplicate units](https://github.com/paritytech/substrate/pull/12634) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Add batching to fast-unstake pallet](https://github.com/paritytech/substrate/pull/12394) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D9-needsaudit 👮`
- [[ci] Use ci-linux:production image in ci](https://github.com/paritytech/substrate/pull/12648) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`
- [New Weights for All Pallets](https://github.com/paritytech/substrate/pull/12325) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D3-trivial 🧸`
- [Remove partial key size limit from trie codec](https://github.com/paritytech/substrate/pull/12566) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Keep the same type of name](https://github.com/paritytech/substrate/pull/12616) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Do not finalize parent twice](https://github.com/paritytech/substrate/pull/12653) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [update paritydb and remove dev deps on rocksdb](https://github.com/paritytech/substrate/pull/12641) - `A3-inprogress`, `B5-clientnoteworthy`, `C1-low 📌`, `D2-notlive 💤`
- [Epoch-Changes tree pruning was lagging by one epoch](https://github.com/paritytech/substrate/pull/12567) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D2-notlive 💤`
- [Bound Election and Staking by MaxActiveValidators](https://github.com/paritytech/substrate/pull/12436) - `A0-pleasereview`, `B3-apinoteworthy`, `C1-low 📌`, `D3-trivial 🧸`
- [Update some dependencies to prune duplicated crates with different version](https://github.com/paritytech/substrate/pull/12560) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Consolidate and deduplicate MMR API methods](https://github.com/paritytech/substrate/pull/12530) - `A0-pleasereview`, `B3-apinoteworthy`, `C1-low 📌`, `D2-notlive 💤`, `D2-breaksapi`
- [Bump ss58-registry from 1.29.0 to 1.34.0](https://github.com/paritytech/substrate/pull/12659) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [Add `CreateOrigin` to Assets Pallet](https://github.com/paritytech/substrate/pull/12586) - `A0-pleasereview`, `B7-runtimenoteworthy`, `C1-low 📌`, `D1-audited 👍`
- [`sp-runtime`: make `parity-util-mem` dependency optional](https://github.com/paritytech/substrate/pull/12657) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [GrandpaJustification: Feature gate `Debug`](https://github.com/paritytech/substrate/pull/12664) - `A2-insubstantial`, `B0-silent`, `C1-low 📌`, `D3-trivial 🧸`
- [More testing and fuzzing and docs for pools](https://github.com/paritytech/substrate/pull/12624) - `A0-pleasereview`, `B0-silent`, `C1-low 📌`, `D2-notlive 💤`
- [Remove `sp_tasks::spawn` API and related code + host functions](https://github.com/paritytech/substrate/pull/12639) - `A0-pleasereview`, `B3-apinoteworthy`, `B5-clientnoteworthy`, `C1-low 📌`, `E4-newhostfunctions`, `D3-trivial 🧸`
- [Backport "Contracts pallet: Bump Runtime API (#12677)" to 0.9.33 branch](https://github.com/paritytech/substrate/pull/12686)
