def main [] {
	cp ../LICENCE .

	cd ../squarkdown
	cargo test
	
	windows
	
	linux

	cd ../npm
	mkdir bin
	cp target-temp/windows/release/squarkdown.exe bin/squarkdown-win.exe
	cp target-temp/linux/release/squarkdown bin/squarkdown-linux.exe
}

def windows [] {
	echo "\n› compiling Windows binary\n"
	cargo build --release --target-dir ../npm/target-temp/windows
}

def linux [] {
	echo "\n› compiling Linux binary\n"
	wsl -- bash -lc "cargo build --release --target-dir ../npm/target-temp/linux"
}
