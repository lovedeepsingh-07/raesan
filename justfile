export PUBLIC_APP_ENV := "development"
export PUBLIC_API_URL := "http://localhost:8080"

build:
	cargo tauri build

run:
	PUBLIC_APP_PLATFORM="native" cargo tauri dev

run_web:
	cargo run -p raesan_web

run_frontend:
	PUBLIC_APP_PLATFORM="web" yarn run dev
build_frontend:
	PUBLIC_APP_PLATFORM="web" yarn run build

test:
	@cargo test -p web_scraper -- --no-capture

lint:
	@cargo clippy -- \
		--allow clippy::needless_return \
		--allow clippy::uninlined_format_args

fmt:
	@alejandra .
	@cargo fmt

build_nix package:
	@mkdir -p build
	@nix build .#{{package}}
	@cp -f -R result/bin/* build/
	@cp -f -R ./test.db build/
