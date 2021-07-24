all:
	@rustfmt --check src/*
	@cargo clippy
	@cargo test --lib
