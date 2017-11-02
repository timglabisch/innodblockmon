build-linux-on-osx:
	docker run -v "$(CURDIR)":/volume -w /volume -t clux/muslrust cargo build --target=x86_64-unknown-linux-musl --release

build: build-linux-on-osx

run:
	RUST_BACKTRACE=1 cargo run -- --user=root  --host="127.0.0.1" --port=3306

