# Command `get`
```
Substrate-link node getter

Usage: subalfred get [OPTIONS] <COMMAND>

Commands:
  runtime-upgrade-block
          Get the runtime upgrade block
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

## Command `get runtime-upgrade-block`
```
Get the runtime upgrade block.

Using the dichotomy algorithm to find it out. This operation will fail if the runtime version does not existed.

Usage: subalfred get runtime-upgrade-block [OPTIONS] --uri <URI> <VERSION>

Arguments:
  <VERSION>
          At this runtime version

Options:
      --uri <URI>
          Node's WS RPC endpoint

          [default: ws://localhost:9944]

  -l, --log <TARGET=LEVEL,*>
          Set a custom log filter.

          This flag is also working with the `RUST_LOG` environment variable. If you use `RUST_LOG` simultaneously, this will append `RUST_LOG`'s value after the log.

          [default: info]

  -h, --help
          Print help information (use `-h` for a summary)
```

### Episode 1
The original question was from [stackexchange](https://substrate.stackexchange.com/questions/3861/how-do-you-tell-which-block-number-the-last-runtime-upgrade-happened-on-a-chain).

Yep, I think that is an interesting question.
So, I implemented this method.
It uses dichotomy algorithm, the whole process takes:

$$\log_{2} BlockHeight\times Requests$$

### Example
```sh
subalfred get runtime-upgrade-block 9100 --uri wss://rpc.polkadot.io -lsubalfred_core::node
```
```log
2022-08-27T13:58:07.684200Z TRACE subalfred_core::node: (0, 11788056) -> 9050
2022-08-27T13:58:09.469688Z TRACE subalfred_core::node: (5894028, 11788056) -> 9151
2022-08-27T13:58:10.874493Z TRACE subalfred_core::node: (5894028, 8841042) -> 9110
2022-08-27T13:58:11.852626Z TRACE subalfred_core::node: (5894028, 7367535) -> 9080
2022-08-27T13:58:12.916960Z TRACE subalfred_core::node: (6630782, 7367535) -> 9090
2022-08-27T13:58:13.126836Z TRACE subalfred_core::node: (6999158, 7367535) -> 9090
2022-08-27T13:58:14.591213Z TRACE subalfred_core::node: (7183346, 7367535) -> 9110
2022-08-27T13:58:14.800144Z TRACE subalfred_core::node: (7183346, 7275440) -> 9110
2022-08-27T13:58:15.921005Z TRACE subalfred_core::node: (7183346, 7229393) -> 9090
2022-08-27T13:58:18.036717Z TRACE subalfred_core::node: (7206370, 7229393) -> 9090
2022-08-27T13:58:18.993673Z TRACE subalfred_core::node: (7217881, 7229393) -> 9100
2022-08-27T13:58:19.208425Z TRACE subalfred_core::node: (7217881, 7223637) -> 9100
2022-08-27T13:58:19.415621Z TRACE subalfred_core::node: (7217881, 7220759) -> 9100
2022-08-27T13:58:19.624220Z TRACE subalfred_core::node: (7217881, 7219320) -> 9100
2022-08-27T13:58:19.828821Z TRACE subalfred_core::node: (7217881, 7218601) -> 9100
2022-08-27T13:58:20.035127Z TRACE subalfred_core::node: (7217881, 7218241) -> 9100
2022-08-27T13:58:20.240336Z TRACE subalfred_core::node: (7217881, 7218061) -> 9100
2022-08-27T13:58:20.444889Z TRACE subalfred_core::node: (7217881, 7217971) -> 9100
2022-08-27T13:58:21.594894Z TRACE subalfred_core::node: (7217881, 7217926) -> 9090
2022-08-27T13:58:24.625042Z TRACE subalfred_core::node: (7217904, 7217926) -> 9100
2022-08-27T13:58:24.824345Z TRACE subalfred_core::node: (7217904, 7217915) -> 9100
2022-08-27T13:58:25.030852Z TRACE subalfred_core::node: (7217904, 7217910) -> 9100
2022-08-27T13:58:26.058492Z TRACE subalfred_core::node: (7217904, 7217907) -> 9090
2022-08-27T13:58:26.275470Z TRACE subalfred_core::node: (7217906, 7217907) -> 9090
7217907 0x8f10de9e6dcf190dccc90f464a8aa4448c9b080746d8e905bb0e4841fef80fdd
```
