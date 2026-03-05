build:
	@mkdir -p build/static
	@bunx @tailwindcss/cli -i ./tailwind.css -o build/static/tailwind.css
	@cargo build
	@cp -r ./target/debug/raesan ./build/
	@cp -r ./scripts/ ./build/static/

[working-directory: "build"]
run *args: build
	@./raesan {{args}}

lint:
	@cargo clippy -- \
		--allow clippy::needless_return \
		--allow clippy::uninlined_format_args

fmt:
	@alejandra .
	@cargo fmt

test:
	cargo test -- --no-capture
