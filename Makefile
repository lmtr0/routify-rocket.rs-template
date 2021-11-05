run:
	pnpm dev & cargo run 

build:
	cargo build --release
	pnpm build
	
	rm -rf dist/
	mkdir -p dist/
	mv public dist/public
	mv target/release/server dist/server

	podman build -t mycoolserver .