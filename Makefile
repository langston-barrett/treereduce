SHELL := $(shell which bash)
.SHELLFLAGS := -ec

CARGO = cargo
FLAMEGRAPH = flamegraph
RUSTUP = rustup

CARGO_FLAGS =

RS = $(shell find . -name "*.rs")
TOML = $(shell find . -name "*.toml")

.PHONY: all
.DEFAULT: all
all: build fmt lint test

.PHONY: bench
bench:
	$(RUSTUP) run nightly cargo $(CARGO_FLAGS) bench

.PHONY: build
build:
	$(CARGO) $(CARGO_FLAGS) build --examples

# requires: apt-get install -y musl-tools
# requires: rustup target add x86_64-unknown-linux-musl
.PHONY: static
static:
	$(CARGO) build $(CARGO_FLAGS) \
	  --bin treereduce-c \
	  --bin treereduce-java \
	  --bin treereduce-javascript \
	  --bin treereduce-lua \
	  --bin treereduce-rust \
	  --bin treereduce-souffle \
	  --locked \
	  --release \
	  --target=x86_64-unknown-linux-musl

.PHONY: check
check:
	$(CARGO) check --examples $(CARGO_FLAGS)

.PHONY: doc
doc:
	$(MAKE) -C doc html

%.c.clang.svg: %.c $(RS)
	$(CARGO) build $(CARGO_FLAGS) --release
	$(FLAMEGRAPH) -o $@ -- target/release/treereduce-c -s $< ./scripts/clang.sh

%.c.true.svg: %.c $(RS)
	$(CARGO) build $(CARGO_FLAGS) --release
	$(FLAMEGRAPH) -o $@ -- target/release/treereduce-c -s $< true

%.c.false.svg: %.c $(RS)
	$(CARGO) build $(CARGO_FLAGS) --release
	$(FLAMEGRAPH) -o $@ -- target/release/treereduce-c --interesting-exit-code 1 -s $< false

.PHONY: entr
entr:
	ls Makefile $(RS) $(TOML) | \
	  entr -c -s "make -j fmt check lint && make build && make test"

.PHONY: fmt
fmt:
	$(CARGO) fmt $(CARGO_FLAGS)

.PHONY: lint
lint:
	$(CARGO) clippy $(CARGO_FLAGS) -- \
	  -D warnings \
	  -D clippy::unnecessary_wraps

.PHONY: unit
unit:
	$(CARGO) test $(CARGO_FLAGS)

.PHONY: test
test: unit
