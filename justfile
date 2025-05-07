
# build web version and put it out directory
web_build:
	cargo build --target wasm32-unknown-unknown --release
	wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/slavic_castles.wasm
	cp -u wasm/* out/
	cp -R -u assets out/
	echo "castles.mevlyshkin.com" >> out/CNAME
	ls -R out

# validate the code
check:
	cargo fmt --all -- --check
	cargo clippy -- -D warnings

# self host web version
web_host:
	lwa_simple_server out

# run desktop version with bevy file-watcher feature on
hot_reload:
	cargo run --features "bevy/file-watcher"
	
# installs used cli tools
prepare:
	cargo install lwa_simple_server
	cargo install wasm-bindgen-cli
