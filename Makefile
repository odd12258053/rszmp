fmt:
	cargo fmt

lint:
	cargo clippy -- -D warnings
	cargo fmt --check

test:
	cargo test --tests

bench:
	cargo
