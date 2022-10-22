# The State Command
Provides a series of useful state operations.

## Diff
Get the differences between the two given states.

In fact, it accepts two chain spec files. So, make sure JSON file contains the `object["genesis"]["raw"]["top]` field.

Note that, this is not a symmetric diff.
`a.diff(b)` may equals to `b.diff(a)`, but not always.

### Examples
```sh
subalfred state diff a.json b.json
```
```diff
-0xbd2a529379475088d3e29a918cd47872878d434d6125b40443fe11fd292d13a4:0x03000000
-0x4d45a146e2a002ba470f48b9ed9a3e23878d434d6125b40443fe11fd292d13a4:0x02000200
-0xcebf674407db61a30e8759ec5084b7764e7b9012096b41c4eb3aaf947f6ea429:0x0000
-0x2f85f1e1378cb2d7b83adbaf0b5869c24e7b9012096b41c4eb3aaf947f6ea429:0x0000
-0x4a0fb74e77bb0a3a5a6c24785f9805fe878d434d6125b40443fe11fd292d13a4:0x02000500
-0x2aa79c02f5980c623c5a8c28e06320c4878d434d6125b40443fe11fd292d13a4:0x02000200
```
```sh
subalfred state diff b.json a.json
```
```diff
+0xbd2a529379475088d3e29a918cd47872878d434d6125b40443fe11fd292d13a4:0x03000000
+0x4d45a146e2a002ba470f48b9ed9a3e23878d434d6125b40443fe11fd292d13a4:0x02000200
+0xcebf674407db61a30e8759ec5084b7764e7b9012096b41c4eb3aaf947f6ea429:0x0000
+0x2f85f1e1378cb2d7b83adbaf0b5869c24e7b9012096b41c4eb3aaf947f6ea429:0x0000
+0x4a0fb74e77bb0a3a5a6c24785f9805fe878d434d6125b40443fe11fd292d13a4:0x02000500
+0x2aa79c02f5980c623c5a8c28e06320c4878d434d6125b40443fe11fd292d13a4:0x02000200
```

## Export
Export the chain state from a live chain.

The result will be store at `<a>.export`.

### Examples
For convenience, I use the [Darwinia Network] to demonstrate.

Here is a most common way to use this command.
To build a fork-off chain:
```sh
# If you want to renew the consensus you need to provide a genesis to override the exported consensus state.
# Otherwise, you can not get the old validators online. And the new chain will be bricked.
darwinia build-spec --raw > darwinia-dev.json
# `--simple-governance` will simplify the the governance state for more detail check the `--help`.
# If some pallets state are too large, you could skip them with this flag: `--skip-pallets System,Staking,Scheduler`.
subalfred state export wss://rpc.darwinia.network --renew-consensus-with darwinia-dev.json --simple-governance --disable-default-bootnodes -lsubalfred::core::node,subalfred::core::substrate_client
```
```log
2022-08-27T17:30:15.077768Z TRACE subalfred::core::substrate_client: fetched 512 keys
2022-08-27T17:30:15.103602Z TRACE subalfred::core::substrate_client: fetched 1024 keys
...
✓ fully exported 152674 pairs, takes 7s
```
To run the fork-off chain:
```sh
darwinia --chain darwinia-dev.json.export --alice --tmp
```
```log
2022-08-25 15:31:45   _____                      _       _
2022-08-25 15:31:45  |  __ \                    (_)     (_)
2022-08-25 15:31:45  | |  | | __ _ _ ____      ___ _ __  _  __ _
2022-08-25 15:31:45  | |  | |/ _` | '__\ \ /\ / / | '_ \| |/ _` |
2022-08-25 15:31:45  | |__| | (_| | |   \ V  V /| | | | | | (_| |
2022-08-25 15:31:45  |_____/ \__,_|_|    \_/\_/ |_|_| |_|_|\__,_|
2022-08-25 15:31:45 Darwinia
2022-08-25 15:31:45 ✌️  version 0.12.3-2-unknown-x86_64-linux-gnu
2022-08-25 15:31:45 ❤️  by Darwinia Network <hello@darwinia.network>, 2018-2022
2022-08-25 15:31:45 📋 Chain specification: Darwinia Development Testnet-export
2022-08-25 15:31:45 🏷 Node name: Alice
2022-08-25 15:31:45 👤 Role: AUTHORITY
2022-08-25 15:31:45 💾 Database: RocksDb at /tmp/substrateVfPtI4/chains/darwinia_dev-export/db/full
2022-08-25 15:31:45 ⛓  Native runtime: Darwinia-1232 (Darwinia-0.tx0.au0)
2022-08-25 15:32:15 🔨 Initializing Genesis block/state (state: 0xfff2…5f26, header-hash: 0xa524…c2b5)
2022-08-25 15:32:15 👴 Loading GRANDPA authority set from genesis on what appears to be first startup.
2022-08-25 15:32:16 ⏱  Loaded block-time = 6s from block 0xa524d767208b0142b2e2515f072add8391eb8d46359592f385585b453034c2b5
2022-08-25 15:32:16 👶 Creating empty BABE epoch changes on what appears to be first startup.
2022-08-25 15:32:16 🏷 Local node identity is: 12D3KooWBZ5yN77nLrmca1E7Fu1DpgXR5Y9VK3xupwPsWQHDBcdQ
2022-08-25 15:32:16 DVM mapping worker starts syncing from 0
2022-08-25 15:32:16 📦 Highest known block at #0
2022-08-25 15:32:16 〽️ Prometheus exporter started at 127.0.0.1:9615
2022-08-25 15:32:16 Listening for new connections on 0.0.0.0:9944.
2022-08-25 15:32:16 👶 Starting BABE Authorship worker
2022-08-25 15:32:18 🙌 Starting consensus session on top of parent 0xa524d767208b0142b2e2515f072add8391eb8d46359592f385585b453034c2b5
2022-08-25 15:32:18 🎁 Prepared block for proposing at 1 [hash: 0x7b730198ec586071afc14498afa6c1a1f94353db0008e427758adb6353a05690; parent_hash: 0xa524…c2b5; extrinsics (1): [0x60ef…54c4]]
2022-08-25 15:32:18 🔖 Pre-sealed block for proposal at 1. Hash now 0x9a763a5d33bd5bbb7fb37cb5fab5c8ec4f7c05f9e6cc4c1710a6d5076973c3e9, previously 0x7b730198ec586071afc14498afa6c1a1f94353db0008e427758adb6353a05690.
2022-08-25 15:32:18 👶 New epoch 0 launching at block 0x9a76…c3e9 (block slot 276902123 >= start slot 276902123).
2022-08-25 15:32:18 👶 Next epoch starts at slot 276904523
2022-08-25 15:32:18 ✨ Imported #1 (0x9a76…c3e9)
2022-08-25 15:32:21 💤 Idle (0 peers), best: #1 (0x9a76…c3e9), finalized #0 (0xa524…c2b5), ⬇ 0 ⬆ 0
2022-08-25 15:32:24 🙌 Starting consensus session on top of parent 0x9a763a5d33bd5bbb7fb37cb5fab5c8ec4f7c05f9e6cc4c1710a6d5076973c3e9
2022-08-25 15:32:24 🎁 Prepared block for proposing at 2 [hash: 0xfe20a319a55f621fcb5c0a7a93679931a3e729a90836cac71ac47e23475db69a; parent_hash: 0x9a76…c3e9; extrinsics (1): [0xcb64…465b]]
2022-08-25 15:32:24 🔖 Pre-sealed block for proposal at 2. Hash now 0x60275c406bd17b9c2ea4fd25a459db4fd5e3d0b46e5f4134789927b8cbe4c86e, previously 0xfe20a319a55f621fcb5c0a7a93679931a3e729a90836cac71ac47e23475db69a.
2022-08-25 15:32:24 ✨ Imported #2 (0x6027…c86e)
2022-08-25 15:32:26 💤 Idle (0 peers), best: #2 (0x6027…c86e), finalized #0 (0xa524…c2b5), ⬇ 14 B/s ⬆ 0
2022-08-25 15:32:30 🙌 Starting consensus session on top of parent 0x60275c406bd17b9c2ea4fd25a459db4fd5e3d0b46e5f4134789927b8cbe4c86e
2022-08-25 15:32:30 🎁 Prepared block for proposing at 3 [hash: 0x47a99410fa4b2414e1a50198ecd6f7ba191e3089942f29a3144bd0f37d48af53; parent_hash: 0x6027…c86e; extrinsics (1): [0xa820…1274]]
2022-08-25 15:32:30 🔖 Pre-sealed block for proposal at 3. Hash now 0xf6e17b2a54364b18007d3bba082a4a68ddc04ad30d64cd6cab24b0b7c25f91a1, previously 0x47a99410fa4b2414e1a50198ecd6f7ba191e3089942f29a3144bd0f37d48af53.
2022-08-25 15:32:30 ✨ Imported #3 (0xf6e1…91a1)
2022-08-25 15:32:31 💤 Idle (0 peers), best: #3 (0xf6e1…91a1), finalized #0 (0xa524…c2b5), ⬇ 0 ⬆ 0
2022-08-25 15:32:36 🙌 Starting consensus session on top of parent 0xf6e17b2a54364b18007d3bba082a4a68ddc04ad30d64cd6cab24b0b7c25f91a1
2022-08-25 15:32:36 🎁 Prepared block for proposing at 4 [hash: 0x41065d03a825b9ab3a11281d5521c4e8d623804b28fab3931fc55af081d488bf; parent_hash: 0xf6e1…91a1; extrinsics (1): [0x5cfc…5083]]
2022-08-25 15:32:36 🔖 Pre-sealed block for proposal at 4. Hash now 0xfd60895534f8ac72e73aa387b87264455ae06e6e484a7a3f50e4b1700798b2f2, previously 0x41065d03a825b9ab3a11281d5521c4e8d623804b28fab3931fc55af081d488bf.
2022-08-25 15:32:36 ✨ Imported #4 (0xfd60…b2f2)
2022-08-25 15:32:36 💤 Idle (0 peers), best: #4 (0xfd60…b2f2), finalized #1 (0x9a76…c3e9), ⬇ 0 ⬆ 0
2022-08-25 15:32:41 💤 Idle (0 peers), best: #4 (0xfd60…b2f2), finalized #2 (0x6027…c86e), ⬇ 14 B/s ⬆ 0
```

With the `--simple-governance` flag, you could perform a runtime upgrade test though sudo or council locally.

It's a really useful testing tool.

## Fork-Off
This is very similar to the `export` command. But this command can build a fork-off chain even offline.

The result will be store at `<a>.fork-off`.

Sometimes, you have already running a node yourself.
So, you don't want to fetch the state from a live chain, which is slow.
Then you could use this command.

### Examples
For convenience, I use the [Darwinia Network] to demonstrate.

Here is a most common way to use this command.
To build a fork-off chain:
```sh
darwinia build-spec --raw > darwinia-dev.json
# Export the state from your node.
darwinia export-state -d <PATH TO DATA> --chain <CHAIN> <BLOCK NUMBER> > darwinia-node-export.json
subalfred state fork-off darwinia-node-export.json --renew-consensus-with darwinia-dev.json --simple-governance --disable-default-bootnodes
```
To run the fork-off chain:
```sh
darwinia --chain darwinia-node-export.json.fork-off --alice --tmp
```

But there is an issue that we build the fork-off chain in this way.
The block can not be finalized. After some searching I found some well-known keys were dumped.
There some GRANDPA round state old data. Check this [question](https://substrate.stackexchange.com/questions/4359/where-the-grandpa-round-state-stored-at). I'll fix this once I get the well-known keys.

[Darwinia Network]: https://github.com/darwinia-network

## Override
Override state a with b.

In fact, it accepts two chain spec files. So, make sure JSON file contains the `object["genesis"]["raw"]["top]` field.

The result will be store at `<a>.override`.

### Examples
```sh
subalfred state override a.json b.json
# You could check this command's output. It should be empty.
subalfred state diff a.json.override b.json
```
