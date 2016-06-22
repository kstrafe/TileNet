all:
	cargo doc
	cargo build
	bash -c 'if ! cargo test -- --nocapture > /tmp/cargout; then cat /tmp/cargout; fi'

fmt:
	cargo fmt -- --write-mode diff
