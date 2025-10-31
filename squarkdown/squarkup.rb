T_START = Time.now

require_relative "../squark.version"
require_relative "utils/ansi"
require_relative "utils/log"
require_relative "utils/error"
log "#{CYAN}running Squarkdown v#{VERSION}"

require_relative "config"
require_relative "types/site-data"
require_relative "types/file-data"

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
  log error: "could not find #{CYAN}squarkup.json#{RED}"
  exit
else
  log success: "found #{BLUE}squarkup.json#{CYAN}"
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

if repo_config["paths / sources"].nil? and repo_config["paths / exclude"].nil?
  log done: true
  exit(0)
end


## find file bases
log "locating file base..."

base = {}

if repo_config["bases / path"].nil?
  log error: "no base path set!"

else
  base["bases / page.svelte"] = find_file_base("bases / page.svelte", repo_config:)
  if base["bases / page.svelte"].nil?
    log error: "no base for #{BLUE}+page.svelte#{RED} found"
  else
    log success: "found base for #{BLUE}+page.svelte#{CYAN}"
  end

  base["bases / page.js"] = find_file_base("bases / page.js", repo_config:)
  if base["bases / page.js"].nil?
    log error: "no base for #{BLUE}+page.js#{RED} found"
  else
    log success: "found base for #{BLUE}+page.js#{CYAN}"
  end

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

site_data.meta[:file_count] = total


## export articles
log "exporting files..."

index_files = []

files.each_with_index do |file, i|
  log "#{i+1}#{GREY} of #{total}: #{WHITE}#{file.parent.basename}#{GREY}/#{BLUE}#{file.basename}"

  begin
    ## process
    lines = file.readlines
    file_data = FileData.new(file, repo_config:)
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

    if file_data.flags.include?("index")
      index_files.append(file_data.index)
      
      file_data.index.each do |index|
        site_data.create_index(index:, page: file_data.dest)
      end

      next
    end

    ## index + tag
    file_data.index.each do |index|
      site_data.update_index(index:, page: file_data.path)
    end
    file_data.tags.each do |tags|
      site_data.update_tags(tags:, page: file_data.path)
    end

  rescue => e
    error(e)
  
  end
end


# TODO export index pages
if index_files.length > 0
  log "exporting index pages..."
end


## save
log "saving site data..."

site_data.meta[:page_count] = site_data.pages.length

save_site_data(site_data.export_json, repo_config:)
log success: "saved site data to #{BLUE}#{repo_config['out / site-data']}"


## finish
T_END = Time.now

log done: true
log "#{GREY}finished in #{YELLOW}#{(1000 * (T_END - T_START)).round}#{WHITE} ms"
