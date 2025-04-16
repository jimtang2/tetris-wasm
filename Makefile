.PHONY: install_rust install_npm test build_rust build

install_rust: 
	@curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y @rustup target add wasm32-unknown-unknown

install_npm: 
	@npm install next

test: 
	@cargo test

build_rust: 
	@wasm-pack build --target web --out-dir public/wasm/

build: build_rust 

dev:
	npm run dev