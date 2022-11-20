build:
	cargo build --release
test-http:
	cargo run -- tests/httptest.json
test-ws:
	cargo run -- tests/httptest.json
