## Injects the Squarkup JSON Schema into `docs/squarkup-config.md`.

require "json"

require_relative "../../squarkdown/config"
require_relative "../../squarkdown/utils/log"
require_relative "helpers/render-table"


log "syncing squarkup schema..."

route = Routes.root / "squarkdown/resources/squarkup-schema.json"
dest = Routes.site / "static/squarkup-schema/latest.json"
FileUtils.cp(route, dest)

log done: true
