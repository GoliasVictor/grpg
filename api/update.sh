#!/bin/sh
RUST_DIRECTORY=<REPLACE_WITH_RUST_DIRECTORY>
export KUZU_SHARED=0
export KUZU_INCLUDE_DIR=<REPLACE_WITH_INCLUDE_DIR>
export KUZU_LIBRARY_DIR=<REPLACE_WITH_LIBRARY_DIR>
export PATH=$PATH:$RUST_DIRECTORY

{
    service grpg stop
    while service grpg status > /dev/null 2>&1; do
        sleep 1
    done
    git pull
    cargo build --release
    sudo service grpg start
} > update_log-latest.txt 2>&1
