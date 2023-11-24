SHELL = /bin/bash
RUST_9CC = target/release/rust-9cc
SRC = src/main.rs src/ast.rs src/assembly.rs src/gen_code.rs src/virtual_machine.rs
TEMP_FILES = tmp tmp.s
LCOV_FILE = lcov.info

all: $(RUST_9CC)

define run_external_test
	source <(cargo llvm-cov show-env --export-prefix) &&\
	cargo llvm-cov clean --workspace &&\
	cargo build &&\
	./test.sh &&\
	$(SHELL) <(echo $(1))
endef

# Run Integration tests
test:
	$(call run_external_test,'cargo llvm-cov report --fail-under-regions 100')

# Run Integration tests and generate a lcov file
test-lcov:
	$(call run_external_test,"cargo llvm-cov report --lcov --output-path $(LCOV_FILE)")

# Run unit tests
utest:
	cargo llvm-cov --lcov --output-path $(LCOV_FILE)

clean:
	rm -f $(RUST_9CC) $(TEMP_FILES)

$(RUST_9CC): $(SRC)
	cargo build --release

.PHONY: all test clean
