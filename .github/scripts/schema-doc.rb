## Injects the Squarkup JSON Schema into `docs/squarkup-config.md`.

require "json"

require_relative "../../squarkdown/config"
require_relative "../../squarkdown/utils/log"
require_relative "helpers/render-table"


log "documenting squarkup schema..."

## Load
content = File.read(Routes.root / "squarkdown/resources/squarkup-schema.json")

schema = JSON.parse(content)

## Render
rows = schema["properties"].map do |field, props|
  render_table(field:, props:)
end
content = rows.join("\n")

## Inject
pattern = /<!-- #SQUARK inject\? -->.*?<!-- #SQUARK inject. -->/m
repl = """<!-- #SQUARK inject? -->
| Field | Type | Values | Default | Description |
| :---- | :--- | :----- | :------ | :---------- |
#{content}
<!-- #SQUARK inject. -->"""

route = Routes.root / "docs/reference/squarkup-json.md"
existing = File.read(route)
text = existing.sub(pattern, repl)

## Save
File.write(route, text)


log done: true
