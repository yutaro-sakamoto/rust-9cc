RUST_9CC = target/release/rust-9cc
SRC = src/main.rs src/ast.rs
TEMP_FILES = tmp tmp.s

all: $(RUST_9CC)

test: $(RUST_9CC)
	./test.sh

clean:
	rm -f $(RUST_9CC) $(TEMP_FILES)

$(RUST_9CC): $(SRC)
	cargo build --release

.PHONY: all test clean
