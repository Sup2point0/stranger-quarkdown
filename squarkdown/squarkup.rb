require_relative "config"
require_relative "types/site-data"
require_relative "types/file-data"
require_relative "utils/log"

require_relative "core/find"
require_relative "core/process"
require_relative "core/render"
require_relative "core/export"
require_relative "core/construct"


log "squarking up..."


site_data = SiteData.new

repo_config = find_repo_config()
if repo_config.nil? or repo_config.length == 0
  log error: "squarkup config not found!"
  exit
else
  log success: "found squarkup config!"
end


## scripts
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


## find
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
    log "#{i}#{Cols[:grey]} of #{total} – #{Cols[:white]}#{file.basename}"

    begin
      ## process
      lines = file.readlines
      file_data = FileData.new(file)
      file_data = extract_data(lines:, data: file_data, repo_config:)
      if file_data.nil?
        log error: "processing failed!"
        next
      end

      ## render
      content = lines.join("")
      render = render_file(content, data: file_data, repo_config:)

      ## export
      export_file(render, data: file_data, base: base, repo_config:)

      site_data.add_page(file_data)

      file_data.index.each do |index|
        site_data.add_index(index:, page: file_data.path)
      end
      file_data.shard.each do |shard|
        site_data.add_shard(shard:, page: file_data.path)
      end

    rescue => e
      # log error: "#{e.class}: #{e.message}"
      raise
    ensure
      i += 1
    end
  end
end


log "saving site data..."
save_site_data(site_data.export_json, repo_config:)
log success: "saved site data to #{Cols[:yellow]}#{repo_config['site-data']}"


log done: true
