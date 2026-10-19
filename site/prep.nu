def main [] {
	print "syncing squarkup schema..."

	cd ../squarkdown/src/config
	let $ver = (open schema.json | get version)
	print $"received version: ($ver)"

	cp schema.json $"../../../site/static/squarkup-schema/v($ver).json"
	cp schema.json $"../../../site/static/squarkup-schema/latest.json"
}
