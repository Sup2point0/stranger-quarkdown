## Injects the Squarkup JSON Schema into `docs/squarkup-config.md`.

require "json"

require_relative "../../squarkdown/config"
require_relative "../../squarkdown/utils/log"
require_relative "helpers/render-table"


log "syncing squarkup schema..."

version = ARGV[0]

route = Routes.root / "squarkdown/resources/squarkup-schema.json"
dest = Routes.site / "static/squarkup-schema"

dest.mkpath()
FileUtils.cp(route, dest / "latest.json")
FileUtils.cp(route, dest / "v#{version}.json")

log done: true
