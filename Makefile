fmt:
	cargo fmt
	uv run --directory esri_json_py/ ruff format
check:
	cargo clippy --all-targets --all-features && \
	uv run --directory esri_json_py/ ruff check
test:
	cargo nextest run --all-targets --all-features && \
	uv run --directory esri_json_py/ pytest && \
	cargo test --doc
doc:
	cargo doc --no-deps --open && \
	cargo watch -s 'cargo doc --no-deps'