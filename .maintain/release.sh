#!/bin/sh

cargo publish --locked -p subcryptor
cargo publish --locked -p subhasher
cargo publish --locked -p submetadatan
cargo publish --locked -p subrpcer
cargo publish --locked -p subruntimer
cargo publish --locked -p subspector
# substorager depends on subhasher, sleep for 15s
sleep 15
cargo publish --locked -p substorager
cargo publish --locked -p subversioner

cargo publish --locked -p subalfred-util
# subalfred-core depends on subalfred-util, sleep for 15s
sleep 15
cargo publish --locked -p subalfred-core
cargo publish --locked -p cmd-impl
# subalfred depends on cmd-impl, sleep for 15s
sleep 15
cargo publish --locked -p subalfred
