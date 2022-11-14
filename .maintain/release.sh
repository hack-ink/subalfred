#!/bin/sh

cargo publish -p subcryptor
cargo publish -p subhasher
cargo publish -p submetadatan
cargo publish -p subrpcer
cargo publish -p subruntimer
# substorager depends on subhasher, sleep for 15s
sleep 15s
cargo publish -p substorager
cargo publish -p subversioner

cargo publish -p subalfred-util
# subalfred-core depends on subalfred-util, sleep for 15s
sleep 15s
cargo publish -p subalfred-core
cargo publish -p cmd-impl
# subalfred depends on cmd-impl, sleep for 15s
sleep 15s
cargo publish -p subalfred
