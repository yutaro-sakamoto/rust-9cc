RUST_9CC = target/release/rust-9cc
SRC = src/main.rs

all: $(RUST_9CC)

test: $(RUST_9CC)
	./test.sh

clean:
	rm -f $(RUST_9CC)

RUST_9CC: $(SRC)
	cargo build --release

.PHONY: all test clean