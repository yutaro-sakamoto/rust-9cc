SHELL = /bin/bash
RUST_9CC = target/release/rust-9cc
SRC = src/main.rs src/ast.rs src/assembly.rs
TEMP_FILES = tmp tmp.s
LCOV_FILE = lcov.info

all: $(RUST_9CC)

# Run Integration tests
test:
	source <(cargo llvm-cov show-env --export-prefix) &&\
	cargo llvm-cov clean --workspace &&\
	cargo build &&\
	./test.sh &&\
	cargo llvm-cov report --lcov --output-path $(LCOV_FILE)

# Run unit tests
utest:
	cargo llvm-cov --lcov --output-path $(LCOV_FILE)

clean:
	rm -f $(RUST_9CC) $(TEMP_FILES)

$(RUST_9CC): $(SRC)
	cargo build --release

.PHONY: all test clean
