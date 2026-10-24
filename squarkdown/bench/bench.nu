const BENCH_FOLDER = path self | path dirname
const BENCH_SOURCES = [$BENCH_FOLDER, "bench-sources"] | path join
const BENCH_FILE = [$BENCH_SOURCES, "fixed", "page.md"] | path join

const MAX_DEPTH = 3
const FILE_COUNT = 100
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
	print "done!"

	# cargo run
}

def create-source-dirs [] {

	create-source-dir-from 0 $BENCH_SOURCES
}

def create-source-dir-from [depth: int, parent: path] {

	if $depth >= $MAX_DEPTH { return }

	for folder_name in $FOLDER_NAMES {
		let $subfolder = [$parent, $folder_name] | path join
		print $"rebuilding: ($subfolder)"
		check-safe-to-delete $subfolder
		# rm -rf $subfolder
		mkdir $subfolder

		0..$FILE_COUNT | par-each {|i|
			let $filepath = [$subfolder, $"page-($i).md"] | path join
			cp $BENCH_FILE $filepath
		}

		create-source-dir-from ($depth + 1) $subfolder
	}
}

def check-safe-to-delete [target: path] {
	try {
		$target | path relative-to $BENCH_FOLDER
	} catch {
		error make {
			msg: $"refusing to delete path outside bench/ folder: ($target)"
		}
	}
}
