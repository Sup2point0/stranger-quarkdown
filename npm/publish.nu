def main [] {
	cp ../LICENCE .

	cd ../squarkdown
	cargo test

	sync-version
	windows
	linux

	cd ../npm
	mkdir bin
	cp target-temp/windows/release/squarkdown.exe bin/squarkdown-win.exe
	cp target-temp/linux/release/squarkdown bin/squarkdown-linux
}

def sync-version [] {
	echo "\n› syncing version from Cargo.toml to package.json\n"
	let $ver = (open Cargo.toml | get package.version)

	cd ../npm

	let $regex_match = "\"version\": \"([^\"]+)\""
	let $regex_repl  = "\"version\": \"$1\""

	let $before = open package.json -r
	let $after = $before | str replace --regex $regex_match $regex_repl
	$after | save package.json -f
}

def windows [] {
	echo "\n› compiling Windows binary\n"
	cargo build --release --target-dir ../npm/target-temp/windows
}

def linux [] {
	echo "\n› compiling Linux binary\n"
	wsl -- bash -lc "cargo build --release --target-dir ../npm/target-temp/linux"
}
