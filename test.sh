#!/bin/bash

# Build the Solana program first
echo "Building Solana program..."
cargo build-sbf

# Run tests with the correct environment variable
echo "Running tests..."
SBF_OUT_DIR="$(pwd)/target/deploy" cargo test