#!/bin/sh
RUST_DIRECTORY=<REPLACE_WITH_RUST_DIRECTORY>
export KUZU_SHARED=0
export KUZU_INCLUDE_DIR=<REPLACE_WITH_INCLUDE_DIR>
export KUZU_LIBRARY_DIR=<REPLACE_WITH_LIBRARY_DIR>
export PATH=$PATH:$RUST_DIRECTORY

nohup sh -c '
    service grpg stop
    git pull
    cargo build --release
    sudo service grpg start
' > update_log-latest.txt 2>&1 &