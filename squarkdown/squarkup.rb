require_relative "config"
require_relative "types/site-data"
require_relative "types/file-data"
require_relative "core/find"
require_relative "core/process"
require_relative "core/render"
require_relative "core/export"
require_relative "utils/log"

require_relative "scripts/prep-assets" if ARGV.include? "assets"


log "squarking up..."


site_data = SiteData.new

repo_config = find_repo_config()
if repo_config.nil? or repo_config.length == 0
  log error: "squarkup config not found!"
  exit
else
  log success: "found squarkup config!"
end


if ARGV.include? "fonts"
  require_relative "scripts/prep-fonts"
  prep_fonts(repo_config:)
end

if ARGV.include? "assets"
  require_relative "scripts/prep-assets"
  prep_assets(repo_config:)
end

if ARGV.include? "scss"
  require_relative "scripts/prep-scss"
  prep_scss(repo_config:)
end


log "locating file base..."

base = {}

base["page.svelte"] = find_file_base("page.svelte", repo_config:)
if base["page.svelte"].nil?
  log error: "no base for +page.svelte found!"
else
  log success: "found base for +page.svelte!"
end

base["page.js"] = find_file_base("page.js", repo_config:)
if base["page.js"].nil?
  log error: "no base for +page.js found!"
else
  log success: "found base for +js.svelte!"
end


log "locating files..."

files = find_files(repo_config:)
total = files.length

if total.nil? or total == 0
  log error: "no files found!"
else
  log success: "found #{total} files!"
end


i = 1

unless base.nil?
  log "exporting files..."

  files.each do |file|
    log "#{i}#{Cols[:grey]} of #{total} – #{Cols[:blue]}#{file.basename}"

    begin
      lines = file.readlines
      file_data = extract_data(lines:, repo_config:)
      if file_data.nil?
        next
      end

      site_data.add_page(file_data)
      site_data.add_index(file_data.index)
      site_data.add_shard(file_data.shard)

      content = lines.join("")
      render = render_file(content, data: file_data, repo_config:)
      export_file(render, data: file_data, base: base, repo_config:)

    rescue => e
      # log error: "#{e.class}: #{e.message}"
      raise
    ensure
      i += 1
    end
  end
end


log done: true
