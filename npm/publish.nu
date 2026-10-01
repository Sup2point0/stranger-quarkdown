const n = ansi 'n'

const WHITE = $'($n)(ansi '#ffffff')'
const PINK  = $'($n)(ansi '#f190f1')'


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
	chmod +x bin/squarkdown-linux
}

def sync-version [] {
	print $"($PINK)\n› syncing version from Cargo.toml to package.json\n($WHITE)"

	let $ver = (open Cargo.toml | get package.version)
	print $"received version: ($ver)"

	cd ../npm

	let $regex_match = "\"version\": \"([^\"]+)\""
	let $regex_repl  = $"\"version\": \"($ver)\""

	let $before = open package.json -r
	let $after = $before | str replace --regex $regex_match $regex_repl
	$after | save package.json -f
}

def windows [] {
	print $"($PINK)\n› compiling Windows binary\n($WHITE)"
	cargo build --release --target-dir ../npm/target-temp/windows
}

def linux [] {
	print $"($PINK)\n› compiling Linux binary\n($WHITE)"
	wsl -- bash -lc "cargo build --release --target-dir ../npm/target-temp/linux"
}
