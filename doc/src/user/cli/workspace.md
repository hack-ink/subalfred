# The Workspace Command
This is a release helper.

Currently, there is only one command.

## Update
Update the workspace members version to the given one.
If `--manifest-path` is missing, it will read the current folder's `Cargo.toml` as the root manifest.

### Examples
```sh
subalfred workspace update v0.9.1
```
