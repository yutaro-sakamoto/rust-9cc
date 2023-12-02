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

assert_fail_compile() {
    input="$1"

    ${RUST_9CC} "$input" > tmp.s
    if [ "$?" = "0" ]; then
      echo "should fail to compile, but succeeded"
      exit 1
    fi
    true
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
assert_program 3 'int a;a=1;a+2;'
assert_program 3 'int a;a=1;int b;b=2;a+b;'
assert_program 9 'int aa;aa=1;int bbb;bbb=2;int cccc;cccc=aa+bbb;(aa+bbb)*cccc;'
assert_program 9 'int aa; aa = 1; int bbb; bbb = 2; int cccc; cccc= aa + bbb; (aa + bbb) * cccc;'

# test return statements
assert_program 3 'return 1 + 2;'
assert_program 3 'int a; a = 1; int b; b = 2; return a + b;'

# test if statements
assert_program 3 'int a; a = 1; int b; b = 2; if (b == 2) {a = a + 2;} a;'
assert_program 3 'int a; a = 1; int b; b = 2; if (b == 2) {a = a + 2;} else { a = a + 3; } a;'
assert_program 4 'int a; a = 1; int b; b = 2; if (b != 2) {a = a + 2;} else { a = a + 3; } a;'

# test statement blocks
assert_program 4 'int a; a = 1; int b; b = 2; if (b == 2) {a = a + 2; a = a + 1; } else { a = a + 3; } a;'
assert_program 5 'int a; a = 1; int b; b = 2; if (b != 2) {a = a + 2;} else { a = a + 3;  a = a + 1; } a;'

# test while statements
assert_program 10 'int a; a = 0; while (a < 10) { a = a + 1; } a;'

# test for statements
assert_program 10 'int a; a = 0; int i; for (i = 0; i < 10; i = i + 1) { a = a + 1; } a;'
assert_program 10 'int a; a = 0; int i; i = 0; for (; i < 10; i = i + 1) { a = a + 1; } a;'
assert_program 5 'int i; for (i = 5; ; i = i + 1) { break; } i;'
assert_program 10 'int a; a = 0; int i; for (i = 0; i < 10; ) { a = a + 1; i = i + 1; } a;'
assert_program 10 'int a; a = 2; int i; i = 5; while(i < 10) { if(i == 7) { a = 10; } i = i + 1; } a;'

# test break statements
assert_program 5 'int a; a = 0; while (a < 10) { if(a == 5) {break;} a = a + 1; } a;'
assert_program 6 'int i; for (i = 0; i < 10; i = i + 1) { if(i >= 6) {break;} } i;'

# test call
assert_program 3 'three();' $C_FUNCTION_OBJ
assert_program 2 'sub(5, 3);' $C_FUNCTION_OBJ
assert_program 102 'int a; a = sub(5, 3); int b; b = avg3(100, 50, 150); a + b;' $C_FUNCTION_OBJ
assert_program 21 'sum6(1,2,3,4,5,6);' $C_FUNCTION_OBJ

# test function definitions
assert_program 3 'int add(int x, int y) { x + y; } add(1, 2);'
assert_program 4 'int sub(int x, int y) { x - y; } sub(5, 1);'
assert_program 31 '
int add3(int x, int y, int z) { x + y + z; }
int sub(int x, int y) { x - y;}
add3(1, 20, sub(100, 90));
'
assert_program 5 '
int min3(int x, int y, int z) 
{
  if (x < y) {
    if (x < z) {
      return x;
    } else {
      return z;
    }
  } else {
    if (y < z) {
      return y;
    } else {
      return z;
    }
  }
}
int add(int x, int y) { x + y; }
min3(add(5,1), add(2,4), add(3,2));
'

# test pointer
assert_program 123 'int a; a = 123; int b; b = &a; *b;'
assert_program 3 'int x; x = 3; int y; y = 5; int z; z = &y + 8; *z;'

# test undefined variable
assert_fail_compile 'a = 1; a;'
assert_fail_compile 'a;'
assert_fail_compile '&a;'

# test assign statement using pointers
assert_program 143 'int x; int* y; y = &x; *y = 143; x;'
assert_program 143 'int x; int * y; int** z; y = &x; z = &y; **z = 143; x;'
assert_program 143 'int x; int * y; int** z; int *** w; y = &x; z = &y; w = &z; ***w = 143; x;'

echo OK
