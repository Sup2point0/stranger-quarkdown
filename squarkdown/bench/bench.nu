const BENCH_FOLDER = path self | path dirname
const BENCH_SOURCES = [$BENCH_FOLDER, "bench-sources"] | path join
const BENCH_FILE = [$BENCH_SOURCES, "fixed", "page.md"] | path join

const MAX_DEPTH = 3
const FILE_COUNT = 42
const FOLDER_NAMES = [
	"prot",
	"deut",
	"trit",
	"quat",
	"quint",
	"hexa",
]


def main [] {
	print "creating source directories..."
	create-source-dirs
	print $"done!"

	cargo run --release -- --quiet
}

def create-source-dirs [] {

	let generated = [$BENCH_SOURCES, "dyna"] | path join
	rm -rf $generated
	create-source-dir-from 0 $generated
}

def create-source-dir-from [depth: int, parent: path] {

	if $depth >= $MAX_DEPTH { return }

	for folder_name in $FOLDER_NAMES {
		let $subfolder = [$parent, $folder_name] | path join
		print $"rebuilding: ($subfolder)"
		mkdir $subfolder

		1..$FILE_COUNT | par-each {|i|
			let $filepath = [$subfolder, $"page-($i).md"] | path join
			cp $BENCH_FILE $filepath
		}

		create-source-dir-from ($depth + 1) $subfolder
	}
}
