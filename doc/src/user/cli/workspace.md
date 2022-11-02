# Command `workspace`
```
Workspace manager

Usage: subalfred workspace [OPTIONS] <COMMAND>

Commands:
  update
          Update the workspace members' crate version
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

## Command `workspace update`
```
Update the workspace members' crate version

Usage: subalfred workspace update [OPTIONS] <VERSION>

Arguments:
  <VERSION>
          Target version

Options:
      --manifest-path <PATH>
          Root `Cargo.toml`'s path.

          If `Cargo.toml` wasn't given, Subalfred will search it under the given path.

          [default: ./Cargo.toml]

  -l, --log <TARGET=LEVEL,*>
          Set a custom log filter.

          This flag is also working with the `RUST_LOG` environment variable. If you use `RUST_LOG` simultaneously, this will append `RUST_LOG`'s value after the log.

          [default: info]

  -h, --help
          Print help information (use `-h` for a summary)
```

### Example
```sh
subalfred workspace update 1.0.0
```
