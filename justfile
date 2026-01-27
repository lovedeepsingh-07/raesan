build:
	@mkdir -p build/static
	@bunx @tailwindcss/cli -i ./tailwind.css -o build/static/tailwind.css
	@cargo build
	@cp -r ./target/debug/raesan ./build/
	@cp -r ./scripts/ ./build/static/

[working-directory: "build"]
run *args: build
	@./raesan {{args}}
