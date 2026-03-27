.PHONY: build test check clean

build:
	cargo build --target wasm32-wasip1 --release

test:
	cargo test

check:
	cargo clippy --target wasm32-wasip1 -- -D warnings

clean:
	cargo clean
