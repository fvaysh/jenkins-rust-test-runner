#!/bin/bash
echo "Setting up environment..."
rustup update
cargo install cargo-tarpaulin
echo "Environment setup complete."
