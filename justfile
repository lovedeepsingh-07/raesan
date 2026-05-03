build:
	cargo tauri build

run:
	doppler run -- cargo tauri dev

run_web:
	doppler run -- cargo run -p raesan_web

run_web_docker:
	nix build .#web_docker --print-build-logs
	sudo docker load < result
	id=$(sudo docker create raesan_web:latest) && \
			sudo docker cp ./raesan.db $id:/raesan.db && \
			sudo docker commit $id raesan_web:latest && \
			sudo docker rm $id
	sudo docker run -p 8080:8080 \
			-e FRONTEND_URL="https://raesan.pages.dev" \
			-e PUBLIC_APP_ENV="production" \
			raesan_web:latest

[working-directory: "frontend"]
run_frontend:
	doppler run -- yarn run dev
build_frontend:
	yarn run build

test:
	@cargo test -p web_scraper -- --no-capture

lint:
	@cargo clippy -- \
		--allow clippy::needless_return \
		--allow clippy::uninlined_format_args

fmt:
	@alejandra .
	@cargo fmt
