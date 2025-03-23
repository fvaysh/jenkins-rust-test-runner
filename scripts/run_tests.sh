#!/bin/bash
echo "Running tests..."
cargo test -- --test-threads=1 | tee logs/test_results.log
