## Injects the Squarkup JSON Schema into `docs/squarkup-config.md`.

require "json"

require_relative "../../squarkdown/utils/log"
require_relative "../helpers/render-table"


log "generating docs for squarkup schema..."

## Load
content = File.read(Root / "squarkdown/resources/squarkup-schema.json")
schema = JSON.parse(content)

## Render
content = schema["properties"].map do |field, props| (
 render_table(field:, props:)
end

## Inject
pattern = /<!-- #SQUARK inject? -->.*?<!-- #SQUARK inject. -->/m,
repl = """<!-- #SQUARK live! -->
#{content}
<!-- #SQUARK inject. -->"""

route = Root / "docs/squarkup-config.md"

existing = File.read(route)
text = existing.sub!(pattern, repl)

## Save
File.write(route, text)
