all:
	cargo doc
	cargo build --features dev
	cargo test -- --nocapture

fmt:
	cargo fmt -- --write-mode diff

fmto:
	cargo fmt -- --write-mode overwrite
