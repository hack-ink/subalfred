# Command `storage-key`
```
Calculate the storage key of the storage item

Usage: subalfred storage-key [OPTIONS] --pallet <PALLET> --item <ITEM>

Options:
      --pallet <PALLET>
          Prefix of the storage

      --item <ITEM>
          Name of the storage item

  -l, --log <TARGET=LEVEL,*>
          Set a custom log filter.

          This flag is also working with the `RUST_LOG` environment variable. If you use `RUST_LOG` simultaneously, this will append `RUST_LOG`'s value after the log.

          [default: info]

  -h, --help
          Print help information (use `-h` for a summary)
```

## Example
```sh
subalfred storage-key --pallet System --item Account
```
```
0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9
```

Actually, it equals to `twox128(pallet) + twox128(item)`.
```sh
subalfred convert ascii2hex System
```
```
0x53797374656d
```
```sh
subalfred hash --hasher twox128 0x53797374656d
```
```
0x26aa394eea5630e07c48ae0c9558cef7
```
```sh
subalfred convert ascii2hex Number
```
```
0x4e756d626572
```
```sh
subalfred hash --hasher twox128 0x4e756d626572
```
```
0x02a5c1b19ab7a04f536c519aca4983ac
```
```
0x26aa394eea5630e07c48ae0c9558cef7 + 0x02a5c1b19ab7a04f536c519aca4983ac
0x26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac
```
