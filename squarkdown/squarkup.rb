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


## find repo config
repo_config = find_repo_config()
if repo_config.nil? or repo_config.length == 0
  log error: "could not find #{Cols[:cyan]}squarkup.json#{Cols[:red]}"
  exit
else
  log success: "found #{Cols[:blue]}squarkup.json#{Cols[:cyan]}"
end


## execute scripts
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


## find file bases
log "locating file base..."

base = {}

base["page.svelte"] = find_file_base("page.svelte", repo_config:)
if base["page.svelte"].nil?
  log error: "no base for #{Cols[:blue]}+page.svelte found#{Cols[:red]}"
else
  log success: "found base for #{Cols[:blue]}+page.svelte#{Cols[:cyan]}"
end

base["page.js"] = find_file_base("page.js", repo_config:)
if base["page.js"].nil?
  log error: "no base for #{Cols[:blue]}+page.js found#{Cols[:red]}"
else
  log success: "found base for #{Cols[:blue]}+page.js#{Cols[:cyan]}"
end


## find files
log "locating files..."

files = find_files(repo_config:)
total = files.length

if total.nil? or total == 0
  log error: "no files found!"
else
  log success: "found #{total} files!"
end


## export articles
log "exporting files..."

i = 1
index_files = []

files.each do |file|
  log "#{i}#{Cols[:grey]} of #{total}: #{Cols[:white]}#{file.parent.basename}#{Cols[:grey]}/#{Cols[:blue]}#{file.basename}"

  begin
    ## process
    lines = file.readlines
    file_data = FileData.new(file)
    file_data = extract_data(lines:, data: file_data, repo_config:)
    
    if file_data.nil?
      next
    end

    ## render
    content = lines.join("")
    render = render_file(content, data: file_data, repo_config:)

    ## export
    export_file(render, data: file_data, base: base, repo_config:)

    site_data.add_page(file_data)

    if file_data.isIndex
      index_files.append(file_data.index)
      
      file_data.index.each do |index|
        site_data.new_index(index: page: file_data.path)
      end

      next
    end

    file_data.index.each do |index|
      site_data.add_index(index:, page: file_data.path)
    end
    file_data.shard.each do |shard|
      site_data.add_shard(shard:, page: file_data.path)
    end

  rescue => e
    if repo_config["on-error"] == "kill"
      raise
    else
      log error: "#{e.class}: #{e.message}"
    end
  ensure
    i += 1
  end
end


## export index pages
if index_files.length > 0
  log "exporting index pages..."

  i = 1
  total = index_files.length

  index_files.each do |file, file_data|
    log "#{i}#{Cols[:grey]} of #{total}: #{Cols[:white]}#{file.parent.basename}#{Cols[:grey]}/#{Cols[:blue]}#{file.basename}"
  end
end


## save
log "saving site data..."

save_site_data(site_data.export_json, repo_config:)
log success: "saved site data to #{Cols[:blue]}#{repo_config['site-data']}"


log done: true
