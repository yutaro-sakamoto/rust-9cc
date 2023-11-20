#!/bin/bash
RUST_9CC=target/debug/rust-9cc
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

assert 17 '5+20-4*2;'
assert 7 '6/2+2*(1+1);'
assert 7 '-5+20-4*2;'
assert 1 '6/(-2)+2*(1+1);'
assert 1 '6/-2+2*(1+1);'

# comparison test

assert 1 '1==1;'
assert 0 '1==2;'

assert 1 '1!=2;'
assert 0 '1!=1;'

assert 1 '-1<2;'
assert 0 '-1<-2;'
assert 0 '-1<-1;'

assert 1 '2>-1;'
assert 0 '-2>-1;'
assert 0 '-1>-1;'

assert 1 '-1<=2;'
assert 0 '-1<=-2;'
assert 1 '-1<=-1;'

assert 1 '2>=-1;'
assert 0 '-2>=-1;'
assert 1 '-1>=-1;'

assert 3 'a=1;a+2;'
assert 3 'a=1;b=2;a+b;'
assert 9 'aa=1;bbb=2;cccc=aa+bbb;(aa+bbb)*cccc;'
assert 9 'aa = 1; bbb = 2; cccc= aa + bbb; (aa + bbb) * cccc;'

echo OK
