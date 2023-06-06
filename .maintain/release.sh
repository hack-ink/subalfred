#!/bin/sh
cargo publish --locked -p subalfred-util
# subalfred-core depends on subalfred-util, sleep for 30s
sleep 30
cargo publish --locked -p subalfred-core
cargo publish --locked -p cmd-impl
# subalfred depends on cmd-impl, sleep for 30s
sleep 30
cargo publish --locked -p subalfred
