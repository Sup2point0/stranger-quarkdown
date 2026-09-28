def main [] {
	cp ../LICENCE .
	
	cd ../squarkdown
	cargo test
	cargo build --release --target-dir ../npm/bin
}
