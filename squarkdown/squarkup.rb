require_relative "config"
require_relative "core/find"
require_relative "core/process"
require_relative "core/render"
require_relative "core/export"
require_relative "utils/log"


log "squarking up..."

repo_config = find_repo_config(in: REPO)

files = find_files(repo_config:)
total = files.length
i = 1


files.each do |file|
  log "#{i} of #{total} – #{file.basename}"

  lines = file.readlines
  content = lines.join("")

  data = extract_data(lines:)
  render = render_file(content:)

  export_file(render, data:, repo_config:)
end


log done: true
