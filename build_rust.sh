#!/usr/bin/env bash

if [[ ! $1 ]]; then
    echo "No root defined of Rust project"
    exit 1;
fi
RUST_ROOT=$1

if [[ ! $2 ]]; then
    echo "No tag defined for the docker build"
    exit 1;
fi
DOCKER_TAG=$2

echo "Changing working directory to ${RUST_ROOT}"
cd $RUST_ROOT

# run the unit tests
docker run -it --rm -v ${RUST_ROOT}:/source -w /source scorpil/rust:1.17 bash -c "cargo test --release && cargo clean && cargo build --release"
# Compile the source code
#docker run -it --rm -v ${RUST_ROOT}:/source -w /source scorpil/rust:1.17
# Build the docker contain that contains the new binary
echo "Building docker file with tag ${DOCKER_TAG}"
docker build . --tag $DOCKER_TAG