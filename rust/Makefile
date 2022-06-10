build-debug:
	cd crates/wasm && \
	wasm-pack build -d ../../../wasm_dist --target web --debug .

build:
	cd crates/wasm && \
	wasm-pack build -d ../../../wasm_dist --target web --release .

test:
  cargo nextest run
