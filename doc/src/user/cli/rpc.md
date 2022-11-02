# Command `rpc`
```
Send a RPC request to the node's HTTP RPC endpoint.

Example: Get the Polkadot's block zero's hash: ``` # Normal output subalfred rpc https://rpc.polkadot.io --method chain_getBlockHash --params '[[0,1,2]]' # Beautiful output subalfred rpc https://rpc.polkadot.io --method chain_getBlockHash --params '[[0,1,2]]' | jq ```

Usage: subalfred rpc [OPTIONS] --method <METHOD> <URI>

Arguments:
  <URI>
          Node's HTTP RPC endpoint

          [default: http://localhost:9933]

Options:
      --method <METHOD>
          JSONRPC method name

      --params <[PARAMETER]>
          JSONRPC parameters

  -l, --log <TARGET=LEVEL,*>
          Set a custom log filter.

          This flag is also working with the `RUST_LOG` environment variable. If you use `RUST_LOG` simultaneously, this will append `RUST_LOG`'s value after the log.

          [default: info]

  -h, --help
          Print help information (use `-h` for a summary)
```

## Example
If you don't have the `jq` command, then just ignore that.
I only use it to beautify the output here.
```sh
subalfred rpc https://rpc.polkadot.io --method chain_getBlockHash --params '[[0,1,2]]' | jq
```
```json
{
  "id": 0,
  "jsonrpc": "2.0",
  "result": [
    "0x91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3",
    "0xc0096358534ec8d21d01d34b836eed476a1c343f8724fa2153dc0725ad797a90",
    "0x409d0bfe677594d7558101d574633d5808a6fc373cbd964ef236f00941f290ee"
  ]
}
```
