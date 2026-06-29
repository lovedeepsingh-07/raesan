run:
	doppler run -- cargo run -p raesan_web

build_docker:
	nix build .#web_docker --print-build-logs
	sudo docker load < result
	id=$(sudo docker create raesan:latest) && \
			sudo docker cp ./raesan.db $id:/raesan.db && \
			sudo docker commit $id raesan && \
			sudo docker rm $id

run_docker:
	sudo docker run -p 8080:8080 \
			-e FRONTEND_URL="https://raesan.enthalapy.com" \
			-e PUBLIC_APP_ENV="production" \
			raesan:latest

build_frontend:
	yarn run build
[working-directory: "frontend"]
run_frontend:
	doppler run -- yarn run dev

test:
	@cargo test -p raesan -- --no-capture

lint:
	@cargo clippy -- \
		--allow clippy::needless_return \
		--allow clippy::uninlined_format_args

fmt:
	@alejandra .
	@cargo fmt
