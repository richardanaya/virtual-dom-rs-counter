build:
	cargo +nightly build --target wasm32-unknown-unknown
	wasm-bindgen target/wasm32-unknown-unknown/debug/dom.wasm --out-dir .
setup:
	npm install
serve:
	npm run serve
