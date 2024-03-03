bl:
	@cargo build --lib --release
	@cp ./target/release/librustcbinding.* .
	@cbindgen --crate=rust-c-binding --lang c > header.h