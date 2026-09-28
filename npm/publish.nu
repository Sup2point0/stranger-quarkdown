def main [] {
	cp ../LICENCE .
	
	cd ../squarkdown
	cargo test
	cargo build --release --target-dir ../npm/target-temp

	cd ../npm
	mkdir bin
	cp target-temp/release/squarkdown.exe bin
}
