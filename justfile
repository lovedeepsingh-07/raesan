build:
	@mkdir -p build/static
	@npx @tailwindcss/cli -i ./tailwind.css -o build/static/tailwind.css
	@cargo build
	@cp -r ./target/debug/raesan ./build/

[working-directory: "build"]
run: build
	@./raesan
