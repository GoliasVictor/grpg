export KUZU_SHARED=1
export KUZU_INCLUDE_DIR=$(realpath ./target/kuzu)
export KUZU_LIBRARY_DIR=$(realpath ./target/kuzu)
export LD_LIBRARY_PATH=LD_LIBRARY_PATH:$(realpath ./target/kuzu)
