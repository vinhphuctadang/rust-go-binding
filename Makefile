bl:
	@cargo build --lib --release
	@cp ./target/release/librustcbinding.dylib .
	@cbindgen --crate=rust-c-binding --lang c > header.h