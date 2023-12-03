SHELL = /bin/bash
RUST_9CC = target/release/rust-9cc
SRC = src/*.rs
TEMP_SOURCE = tmp.s
TEMP_OBJECT = tmp
TEMP_DEBUG_OBJECT = tmpg
TEMP_FILES = $(TEMP_SOURCE) $(TEMP_OBJECT)
C_FUNCTIONS_FILE = functions_for_test.c
C_FUNCTIONS_OBJ = functions_for_test.o
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
test: $(C_FUNCTIONS_OBJ)
	$(call run_external_test,'cargo llvm-cov report --fail-under-functions 100 --ignore-filename-regex="(ast|infer_type).rs"')

# Run Integration tests and generate a lcov file
test-lcov: $(C_FUNCTIONS_OBJ)
	$(call run_external_test,"cargo llvm-cov report --lcov --output-path $(LCOV_FILE)")

# Run unit tests
utest: $(C_FUNCTIONS_OBJ)
	cargo llvm-cov --lcov --output-path $(LCOV_FILE)

$(C_FUNCTIONS_OBJ): $(C_FUNCTIONS_FILE)
	gcc -o $(C_FUNCTIONS_OBJ) -c $(C_FUNCTIONS_FILE)

clean:
	rm -f $(RUST_9CC) $(TEMP_FILES)

$(RUST_9CC): $(SRC)
	cargo build --release

dbg: $(TEMP_DEBUG_OBJECT)

$(TEMP_DEBUG_OBJECT): $(TEMP_SOURCE)
	gcc -o $(TEMP_DEBUG_OBJECT) -g $(TEMP_SOURCE)

.PHONY: all test clean
