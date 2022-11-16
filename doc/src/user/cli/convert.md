# Command `convert`
```
Data style converters

Usage: subalfred convert [OPTIONS] <COMMAND>

Commands:
  ascii2hex
          Convert ascii to hex
  bytes-style
          Convert bytes between several different styles
  bytes2hex
          Convert bytes to hex
  hex2bytes
          Convert hex to bytes
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

## Command `convert ascii2hex`
```
Convert ascii to hex

Usage: subalfred convert ascii2hex [OPTIONS] <ASCII DATA>

Arguments:
  <ASCII DATA>
          Ascii data input

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
subalfred convert ascii2hex AurevoirXavier
```
```
0x41757265766f6972586176696572
```

## Command `convert bytes-style`
```
Convert bytes between several different styles

Usage: subalfred convert bytes-style [OPTIONS] --from <BYTES STYLE> --to <BYTES STYLE> <BYTES>

Arguments:
  <BYTES>
          Bytes data input

Options:
      --from <BYTES STYLE>
          Origin style

          [possible values: byte-string-literal, vec-string]

      --to <BYTES STYLE>
          Target style

          [possible values: byte-string-literal, vec-string]

  -l, --log <TARGET=LEVEL,*>
          Set a custom log filter.

          This flag is also working with the `RUST_LOG` environment variable. If you use `RUST_LOG` simultaneously, this will append `RUST_LOG`'s value after the log.

          [default: info]

  -h, --help
          Print help information (use `-h` for a summary)
```


### Example
#### From Vec String to Byte String Literal
```sh
subalfred convert bytes-style --from vec-string --to byte-string-literal "[65, 117, 114, 101, 118, 111, 105, 114, 88, 97, 118, 105, 101, 114]"
```
```
AurevoirXavier
```
```sh
subalfred convert bytes-style --from vec-string --to byte-string-literal "[1, 1, 1, 1]"
```
```
\x01\x01\x01\x01
```

#### From Byte String Literal to Vec String
```sh
subalfred convert bytes-style --from byte-string-literal --to vec-string "AurevoirXavier"
```
```
[65, 117, 114, 101, 118, 111, 105, 114, 88, 97, 118, 105, 101, 114]
```
```sh
subalfred convert bytes-style --from byte-string-literal --to vec-string "\x01\x01\x01\x01"
```
```
[1, 1, 1, 1]
```

## Command `convert bytes2hex`
```
Convert bytes to hex

Usage: subalfred convert bytes2hex [OPTIONS] <BYTES>

Arguments:
  <BYTES>
          Bytes data input.

          e.g. `[0, 0, 0, 0]`

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
subalfred convert bytes2hex "[65, 117, 114, 101, 118, 111, 105, 114, 88, 97, 118, 105, 101, 114]"
```
```
0x41757265766f6972586176696572
```

## Command `convert hex2bytes`
```
Convert hex to bytes

Usage: subalfred convert hex2bytes [OPTIONS] <HEX>

Arguments:
  <HEX>
          Hex input.

          e.g. `0x00000000`

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
subalfred convert hex2bytes 0x41757265766f6972586176696572
```
```
[65, 117, 114, 101, 118, 111, 105, 114, 88, 97, 118, 105, 101, 114]
```
