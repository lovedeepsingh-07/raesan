export PUBLIC_APP_ENV := "development"
export PUBLIC_FRONTEND_URL := "https://raesan.pages.dev"
export PUBLIC_API_URL := "http://localhost:8080"

build:
	cargo tauri build

run:
	PUBLIC_APP_PLATFORM="native" cargo tauri dev

run_web:
	cargo run -p raesan_web

[working-directory: "frontend"]
run_frontend:
	PUBLIC_APP_PLATFORM="web" bun run dev

test:
	cargo test -- --no-capture

lint:
	@cargo clippy -- \
		--allow clippy::needless_return \
		--allow clippy::uninlined_format_args

fmt:
	@alejandra .
	@cargo fmt

