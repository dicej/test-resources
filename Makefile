build/component.wasm: build/module.wasm resources.wit build/wasi_snapshot_preview1.reactor.wasm
	@mkdir -p "$(@D)"
	wasm-tools component embed resources.wit build/module.wasm -o build/module-with-type.wasm
	wasm-tools component new --adapt build/wasi_snapshot_preview1.reactor.wasm build/module-with-type.wasm -o $@

build/module.wasm: src/lib.rs Cargo.toml
	@mkdir -p "$(@D)"
	cargo build --release --target wasm32-wasi
	cp target/wasm32-wasi/release/test_resources.wasm $@

build/wasi_snapshot_preview1.reactor.wasm:
	@mkdir -p "$(@D)"
	curl -L "https://github.com/bytecodealliance/wasmtime/releases/download/v10.0.1/wasi_snapshot_preview1.reactor.wasm" -o $@

.PHONY: clean
clean:
	cargo clean
	rm -rf build
