#!/bin/bash
set -e
echo "=== AIAL Build ==="
cd "$(dirname "$0")/aial-compiler"
cargo build --release 2>&1 | tail -1
echo "Compiler: OK"
cd ../aial-rt
cargo build --release 2>&1 | tail -1
echo "Runtime:  OK"
echo ""
echo "Done. Add to PATH:"
echo "  export PATH=\"$(pwd)/../aial-compiler/target/release:\$PATH\""
echo ""
echo "Usage:"
echo "  aial run file.aal        # interpreter"
echo "  aial build file.aal      # LLVM AOT -> aial_output.ll"
echo "  clang aial_output.ll -L $(pwd)/target/release -laial_rt -lm -lpthread -ldl -o binary"
