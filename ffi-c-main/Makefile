a.out: main.c ./target/release/libffi_c_main.a
	clang main.c -O3 -L ./target/release/ -l ffi_c_main

./target/release/libffi_c_main.a: FORCE
	cargo build --release

FORCE: ;
