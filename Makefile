CARGO := cargo
BIN := vmctl

.PHONY: all build build-debug run list start stop delete test fmt lint check install clean help

all: build

build:
	$(CARGO) build --release

build-debug:
	$(CARGO) build

run: build-debug
	$(CARGO) run --bin $(BIN) -- $(ARGS)

list: build-debug
	$(CARGO) run --bin $(BIN) -- list $(if $(PROVIDER),--provider $(PROVIDER),)

start: build-debug
	$(CARGO) run --bin $(BIN) -- start $(PROVIDER) $(ID)

stop: build-debug
	$(CARGO) run --bin $(BIN) -- stop $(PROVIDER) $(ID)

delete: build-debug
	$(CARGO) run --bin $(BIN) -- delete $(PROVIDER) $(ID)

test:
	$(CARGO) test --workspace

fmt:
	$(CARGO) fmt --all

lint:
	$(CARGO) clippy --workspace --all-targets -- -D warnings

check:
    fmt lint test

install: build
	$(CARGO) install --path vmctl

clean:
	$(CARGO) clean

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'
