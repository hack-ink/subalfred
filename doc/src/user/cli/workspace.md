# Command `workspace`
```
Workspace manager

Usage: subalfred workspace [OPTIONS] <COMMAND>

Commands:
  update
          Update the workspace member versions
  update-deps
          Update the workspace dependency versions.
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

## Command `workspace update`
```
Update the workspace member versions

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

          This flag is also working with the `RUST_LOG` environment variable. If you use `RUST_LOG`
          simultaneously, this will append `RUST_LOG`'s value after the log.

          [default: info]

  -h, --help
          Print help information (use `-h` for a summary)
```
### Example
```sh
git clone https://github.com/substrate-developer-hub/substrate-parachain-template.git /tmp/subalfred-example/substrate-parachain-template
cd /tmp/subalfred-example/substrate-parachain-template
subalfred workspace update 1.0.0
```

## Command `workspace update-deps`
```
Update the Substrate-related repositories' dependency versions.

To use this command, you must make sure your dependencies were anchored at a branch.
This is a general rule of the Polkadot ecosystem.

We use the regex pattern matching here.
So, `git` field must be set before the `branch` field.

It might look like this:
\```toml
frame-system = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v.0.0.0" }
\```

Usage: subalfred workspace update-deps [OPTIONS] --targets <REPOSITORY,*> <VERSION>

Arguments:
  <VERSION>
          Target version.

          e.g. `0.0.0` will generate `release-v0.0.0` and `polkadot-v0.0.0`

Options:
      --manifest-path <PATH>
          Root `Cargo.toml`'s path.

          If `Cargo.toml` wasn't given, Subalfred will search it under the given path.

          [default: ./Cargo.toml]

      --targets <REPOSITORY,*>
          Targets.

          e.g. cumulus,polkadot,substrate

  -l, --log <TARGET=LEVEL,*>
          Set a custom log filter.

          This flag is also working with the `RUST_LOG` environment variable. If you use `RUST_LOG`
          simultaneously, this will append `RUST_LOG`'s value after the log.

          [default: info]

  -h, --help
          Print help information (use `-h` for a summary)
```
### Episode 1
In parachain development. We have some dependency groups. `cumulus`,`polkadot` and `substrate`.
And you could add your specific dependency group. For example `frontier`.

For example, when we want to update the version from `polkadot-v0.9.30` to `polkadot-v0.9.31`. We need to handle those dependency versions.
And here is a general rule. All those repositories have and must have a branch named `polkadot-v0.9.31`. For `polkadot` it's `release-v0.9.31`.

So, with this, you can update those versions with a one-line command.

Any ideas/improvements are welcome. We can draft an `Episode 2` together!
### Example
We take `substrate-parachain-template` as an example.

This command will update all the dependencies that I described above.

And there is nothing I can show. Try the example commands below and check the file differences.
You'll find all the dependencies were updated correctly.
```sh
git clone https://github.com/substrate-developer-hub/substrate-parachain-template.git /tmp/subalfred-example/substrate-parachain-template
cd /tmp/subalfred-example/substrate-parachain-template
subalfred workspace update-deps 0.9.31 --targets cumulus,polkadot,substrate
```
