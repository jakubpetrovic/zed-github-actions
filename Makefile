.PHONY: build test check clean

build:
	cargo build --target wasm32-wasip1 --release

test:
	cargo test

check:
	cargo fmt --check
	cargo clippy --target wasm32-wasip1 --all-targets -- -D warnings

clean:
	cargo clean
