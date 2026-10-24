const MAX_DEPTH = 3
const FILE_COUNT = 1000
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
	let $bench_sources = [(pwd), "bench-sources"] | path join
	create-source-dir-from 0 $bench_sources
}

def create-source-dir-from [depth: int, parent: path] {

	if $depth >= $MAX_DEPTH { return }

	for folder_name in $FOLDER_NAMES {
		let $subfolder = [$parent, $folder_name] | path join
		# rm $folder
		print $"creating: ($subfolder)"
		mkdir $subfolder

		for i in 0..$FILE_COUNT {
			let $filepath = [$subfolder, $"page-($i).md"] | path join
			print $filepath

			# $"# Page #($i)
			# <!-- #SQUARK live! -->
			# " | save $filepath
		}

		create-source-dir-from ($depth + 1) $subfolder
	}
}
