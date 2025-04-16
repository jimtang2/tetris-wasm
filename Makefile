.PHONY: install_rust install_npm test build_rust build

install: install_rust install_npm

install_rust: 
	@curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y 

install_npm: 
	@npm install 

test: 
	@cargo test

build_rust: 
	@wasm-pack build --target web --out-dir public/wasm/

build: build_rust 

dev: test build_rust
	npm run dev

