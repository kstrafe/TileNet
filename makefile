all:
	cargo doc
	cargo build --features dev
	bash -c 'if ! cargo test -- --nocapture > /tmp/$$$$cargout; then cat /tmp/$$$$cargout; rm /tmp/$$$$cargout; fi'

fmt:
	cargo fmt -- --write-mode diff

fmto:
	cargo fmt -- --write-mode overwrite
