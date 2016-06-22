all:
	cargo doc
	cargo fmt -- --write-mode overwrite
	cargo build
	cargo test -- --nocapture
