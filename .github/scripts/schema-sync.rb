## Injects the Squarkup JSON Schema into `docs/squarkup-config.md`.

require "json"

require_relative "../../squarkdown/utils/log"
require_relative "helpers/render-table"


def sync_schema(version)
  log "syncing squarkup schema..."

  route = Routes.root / "squarkdown/resources/squarkup-schema.json"
  dest = Routes.root / "site/static/squarkup-schema"

  dest.mkpath()
  FileUtils.cp(route, dest / "latest.json")
  FileUtils.cp(route, dest / "v#{version}.json") if version
end
