require_relative "core/find"
require_relative "core/process"
require_relative "core/render"
require_relative "core/export"
require_relative "utils/log"


log "squarking up..."

repo_config = find_repo_config()

files = find_files(repo_config:)
total = files.length
i = 1

files.each do |file|
  log "#{1} of #{total} – #{file.basename}"

  lines = file.readlines
  content = lines.join("")

  data = extract_data(lines:)
  render = render_file(content:)

  export_file(content, data:, repo_config:)
end
