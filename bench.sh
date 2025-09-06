#!/usr/bin/env bash
set -e

cd benchmark/fast

echo "🚀 Rodando benchmarks..."
echo "-----------------------------------------"

if [ -f test_primos.c ]; then
  echo "[C]"
  gcc -O3 test_primos.c -o test_primos_c
  time ./test_primos_c
  echo "-----------------------------------------"
fi

if [ -f test_primos.cc ]; then
  echo "[C++]"
  g++ -O3 test_primos.cc -o test_primos_cc
  time ./test_primos_cc
  echo "-----------------------------------------"
fi

if [ -f test_primos.rs ]; then
  echo "[Rust]"
  rustc -O test_primos.rs -o test_primos_rs
  time ./test_primos_rs
  echo "-----------------------------------------"
fi

if [ -f test_primos.js ]; then
  echo "[Node.js]"
  time node test_primos.js
  echo "-----------------------------------------"
fi

if [ -f test_primos.py ]; then
  echo "[Python]"
  time python3 test_primos.py
  echo "-----------------------------------------"
fi

