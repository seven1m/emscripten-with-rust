emscripten-test.js:
	cargo build --target wasm32-unknown-emscripten
	cp target/wasm32-unknown-emscripten/debug/emscripten-test.js .
	cp target/wasm32-unknown-emscripten/debug/deps/*.wasm .
	cp target/wasm32-unknown-emscripten/debug/deps/*.wasm.map .

clean:
	rm -f emscripten-test.js

build: emscripten-test.js
