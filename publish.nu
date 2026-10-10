const n = ansi 'n'

const WHITE = $'($n)(ansi '#ffffff')'
const PINK  = $'($n)(ansi '#f190f1')'
const FADE  = $'($n)(ansi '#2070c1')'


def main [] {
	test-all
	publish-cargo
	publish-npm
}

## Run all tests before publish
def test-all [] {
	cd squarkdown/tests/test-project
	npm test
	cd ../..
	cargo run
}

## Publish to crates.io
def publish-cargo [] {
	print $"($PINK)\n› publishing to crates.io...\n($WHITE)"
	underline

	cd squarkdown
	cargo publish --allow-dirty
	cd ..
}

## Compile binaries and publish to NPM registry
def publish-npm [] {
	print $"($PINK)\n› publishing to NPM...\n($WHITE)"
	underline

	cd npm

	cp ../LICENCE .

	cd ../squarkdown
	sync-version
	compile-windows
	compile-linux

	cd ../npm
	mkdir bin
	cp target-temp/windows/release/squarkdown.exe bin/squarkdown-win.exe
	cp target-temp/linux/release/squarkdown bin/squarkdown-linux

	npm publish

	cd ..
}

def sync-version [] {
	print $"($PINK)\n› syncing version from Cargo.toml to package.json\n($WHITE)"
	underline

	let $ver = (open Cargo.toml | get package.version)
	print $"received version: ($ver)"

	cd ../npm

	let $regex_match = "\"version\": \"([^\"]+)\""
	let $regex_repl  = $"\"version\": \"($ver)\""

	let $before = open package.json -r
	let $after = $before | str replace --regex $regex_match $regex_repl
	$after | save package.json -f
}

def compile-windows [] {
	print $"($PINK)\n› compiling Windows binary\n($WHITE)"
	underline
	cargo build --release --target-dir ../npm/target-temp/windows
}

def compile-linux [] {
	print $"($PINK)\n› compiling Linux binary\n($WHITE)"
	underline
	wsl -- bash -lc "cargo build --release --target-dir ../npm/target-temp/linux"
}

def underline [] {
	print $"($FADE)─────────────────────────────\n($WHITE)"
}
