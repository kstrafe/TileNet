all:
	cargo doc
	cargo build
	cargo test -- --nocapture
