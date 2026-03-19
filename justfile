build:
	cargo tauri build

run:
	cargo tauri dev

run_web:
	cargo run -p raesan_web

# [working-directory: "frontend"]
# run-web:
# 	bun run dev

test:
	cargo test -- --no-capture

lint:
	@cargo clippy -- \
		--allow clippy::needless_return \
		--allow clippy::uninlined_format_args

fmt:
	@alejandra .
	@cargo fmt

