#!/bin/bash

CARGO_ROOT=$(dirname "$0")

if [ "$CARGO_ROOT" != "$PWD" ]; then
    pushd "$CARGO_ROOT" || exit 1
fi

mode=$1

case ${mode,,} in
    release|r)
        cargo build --release && \
            sudo setcap cap_net_raw,cap_net_admin=eip "$CARGO_ROOT/target/release/pkit"
        ;;
    debug|d)
        cargo build && \
            sudo setcap cap_net_raw,cap_net_admin=eip "$CARGO_ROOT/target/debug/pkit"
        ;;
    *) echo unknow parameter "$1" && exit 1 ;;
esac

popd || exit 1
