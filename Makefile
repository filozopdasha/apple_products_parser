run:
	cargo run parse devices.json yaml
	cargo run parse devices.json toml

fmt:
	cargo fmt

clippy:
	cargo clippy

test:
	cargo test

build:
	cargo build

