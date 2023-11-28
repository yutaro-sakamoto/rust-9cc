#!/bin/bash
RUST_9CC=target/debug/rust-9cc
C_FUNCTION_OBJ=functions_for_test.o

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
    link="$3"

    ${RUST_9CC} "$input" > tmp.s
    cc -o tmp tmp.s $link
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
assert_program 4 'a = 1; b = 2; if (b == 2) {a = a + 2; a = a + 1; } else { a = a + 3; } a;'
assert_program 5 'a = 1; b = 2; if (b != 2) {a = a + 2;} else { a = a + 3;  a = a + 1; } a;'

# test while statements
assert_program 10 'a = 0; while (a < 10) { a = a + 1; } a;'

# test for statements
assert_program 10 'a = 0; for (i = 0; i < 10; i = i + 1) { a = a + 1; } a;'
assert_program 10 'a = 0; i = 0; for (; i < 10; i = i + 1) { a = a + 1; } a;'
assert_program 5 'for (i = 5; ; i = i + 1) { break; } i;'
assert_program 10 'a = 0; for (i = 0; i < 10; ) { a = a + 1; i = i + 1; } a;'
assert_program 10 'a = 2; i = 5; while(i < 10) { if(i == 7) { a = 10; } i = i + 1; } a;'

# test break statements
assert_program 5 'a = 0; while (a < 10) { if(a == 5) {break;} a = a + 1; } a;'
assert_program 6 'for (i = 0; i < 10; i = i + 1) { if(i >= 6) {break;} } i;'

# test call
assert_program 2 'sub(5, 3);' $C_FUNCTION_OBJ
assert_program 102 'a = sub(5, 3); b = avg3(100, 50, 150); a + b;' $C_FUNCTION_OBJ
assert_program 21 'sum6(1,2,3,4,5,6);' $C_FUNCTION_OBJ

# test function definitions
assert_program 123 'fn add(x, y) { x + y; } 123;'
echo OK
