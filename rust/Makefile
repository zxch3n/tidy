build:
	cd crates/wasm && \
	wasm-pack build -d ../../../wasm_dist --target web --release .

build-debug:
	cd crates/wasm && \
	wasm-pack build -d ../../../wasm_dist --target web --debug .

test:
	cargo nextest run

clean:
	cargo clean

bench-wasm:
	cd crates/wasm && \
	wasm-pack test --firefox --headless --release
