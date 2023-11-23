#!/bin/bash
RUST_9CC=target/debug/rust-9cc
assert() {
    expected="$1"
    command="${@:2}"
    if [ "$command" = "" ]; then
      echo "command is empty"
      exit 1
    fi
    ${command}
    if [ "$?" = "$expected" ]; then
      echo "$command => $expected"
    else
      echo "$command => $expected expected, but got $?"
      exit 1
    fi
}

assert_program() {
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

# test the format of command line arguments
assert 1 ${RUST_9CC}
assert 1 ${RUST_9CC} 'first argument' 'second argument'

# test the invalid program format
assert_program 127 '$y-0'

# test basic arithmetics
assert_program 17 '5+20-4*2;'
assert_program 7 '6/2+2*(1+1);'
assert_program 7 '-5+20-4*2;'
assert_program 1 '6/(-2)+2*(1+1);'
assert_program 1 '6/-2+2*(1+1);'

# comparison test

assert_program 1 '1==1;'
assert_program 0 '1==2;'

assert_program 1 '1!=2;'
assert_program 0 '1!=1;'

assert_program 1 '-1<2;'
assert_program 0 '-1<-2;'
assert_program 0 '-1<-1;'

assert_program 1 '2>-1;'
assert_program 0 '-2>-1;'
assert_program 0 '-1>-1;'

assert_program 1 '-1<=2;'
assert_program 0 '-1<=-2;'
assert_program 1 '-1<=-1;'

assert_program 1 '2>=-1;'
assert_program 0 '-2>=-1;'
assert_program 1 '-1>=-1;'

# test the variable
assert_program 3 'a=1;a+2;'
assert_program 3 'a=1;b=2;a+b;'
assert_program 9 'aa=1;bbb=2;cccc=aa+bbb;(aa+bbb)*cccc;'
assert_program 9 'aa = 1; bbb = 2; cccc= aa + bbb; (aa + bbb) * cccc;'

# test return statements
assert_program 3 'return 1 + 2;'
assert_program 3 'a = 1; b = 2; return a + b;'

# test if statements
assert_program 3 'a = 1; b = 2; if (b == 2) {a = a + 2;} a;'
assert_program 3 'a = 1; b = 2; if (b == 2) {a = a + 2;} else { a = a + 3; } a;'
assert_program 4 'a = 1; b = 2; if (b != 2) {a = a + 2;} else { a = a + 3; } a;'

# test statement blocks
assert_program 3 '{a = 1; b = 2; a + b;}'
assert_program 4 'a = 1; b = 2; if (b == 2) {a = a + 2; a = a + 1; } else { a = a + 3; } a;'
assert_program 5 'a = 1; b = 2; if (b != 2) {a = a + 2;} else { a = a + 3;  a = a + 1; } a;'

echo OK
