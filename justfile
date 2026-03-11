build:
	cargo tauri build

run:
	cargo tauri dev

[working-directory: "frontend"]
run-web:
	bun run dev

test:
	cargo test -- --no-capture

lint:
	@cargo clippy -- \
		--allow clippy::needless_return \
		--allow clippy::uninlined_format_args

fmt:
	@alejandra .
	@cargo fmt

