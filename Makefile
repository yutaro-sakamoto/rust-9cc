all:
	cargo run -- 42 > test.s
	cc -o test test.s
	./test || true
