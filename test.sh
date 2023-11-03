#!/bin/bash
RUST_9CC=target/release/rust-9cc
assert() {
    expected="$1"
    input="$2"

    ${RUST_9CC} "$input" > tmp.s
    cc -o tmp tmp.s
    ./tmp
    actual="$?"

    if [ "$actual" = "$expected" ]; then
      echo "$input => $actual"
    else
      echo "$input => $expected expected, but got $actual"
      exit 1
    fi
}

assert 17 '5+20-4*2'
assert 7 '6/2+2*(1+1)'

echo OK
