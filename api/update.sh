#!/bin/sh
RUST_DIRECTORY="$HOME/.cargo/bin"
export KUZU_SHARED=0
export KUZU_INCLUDE_DIR=$(realpath ./kuzu)
export KUZU_LIBRARY_DIR=$(realpath ./kuzu)
export PATH=$PATH:$RUST_DIRECTORY

service grpg stop
git pull
cargo build --release
sudo service grpg start