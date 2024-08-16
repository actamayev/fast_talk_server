#!/bin/bash

# Run Clippy on all targets and treat warnings as errors
cargo clippy --all -- -D warnings
if [ $? -ne 0 ]; then
  echo "Clippy checks failed!"
  exit 1
fi

# Run cargo check on all targets
cargo check --all-targets
if [ $? -ne 0 ]; then
  echo "Type checks failed!"
  exit 1
fi

echo "Validation passed!"
