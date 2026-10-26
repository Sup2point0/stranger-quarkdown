def main [] {
	cd ../squarkdown
	cargo run

	print "syncing squarkup schema..."
	cd src/config
	let $ver = (open schema.json | get "squarkdown-version")
	print $"received version: ($ver)"

	cp schema.json $"../../../site/static/schemas/v($ver).json"
	cp schema.json $"../../../site/static/schemas/latest.json"
}
