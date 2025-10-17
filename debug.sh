#!/bin/bash

# This is needed when using Musl
# export RUSTFLAGS="-C target-feature=-crt-static"

export RUSTUP_HOME=$HOME/.rustup
export RUSTUP_TOOLCHAIN=nightly-x86_64-unknown-linux-gnu
export LD_LIBRARY_PATH=$(realpath ./target/debug/build/*/{out,out/lib} | tr '\n' ':')
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH$PWD/target/debug
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$PWD/target/debug/deps
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$RUSTUP_HOME/toolchains/$RUSTUP_TOOLCHAIN/lib/rustlib/x86_64-unknown-linux-gnu/lib
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$RUSTUP_HOME/toolchains/$RUSTUP_TOOLCHAIN/lib

cargo build --bin backtest_external
rust-gdb ./target/debug/backtest_external

