WASI_SDK ?= /opt/wasi-sdk

build/component.wasm: build/module.wasm foo.wit build/wasi_snapshot_preview1.reactor.wasm
	@mkdir -p "$(@D)"
	wasm-tools component embed foo.wit build/module.wasm -o build/module-with-type.wasm
	wasm-tools component new --adapt build/wasi_snapshot_preview1.reactor.wasm build/module-with-type.wasm -o $@

build/module.wasm: foo.c foo-impl.c foo.h
	@mkdir -p "$(@D)"
	$(WASI_SDK)/bin/clang -Wall -Wextra -Werror foo.c foo-impl.c -o $@

build/wasi_snapshot_preview1.reactor.wasm:
	curl -L "https://github.com/bytecodealliance/wasmtime/releases/download/v10.0.1/wasi_snapshot_preview1.reactor.wasm" -o $@

.PHONY: clean
clean:
	rm -rf build
