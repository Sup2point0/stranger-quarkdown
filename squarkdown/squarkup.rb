require_relative "types/site-data"
require_relative "types/file-data"

require_relative "core/find"
require_relative "core/process"
require_relative "core/render"
require_relative "core/export"


##
# Run squarkup on a repository.
def squarkup(repo_config:)
  # Squarkdown only runs if `sources` or `exclude` have been set.
  if repo_config["paths / sources"].nil? and repo_config["paths / exclude"].nil?
    log done: true
    return
  end

  log "squarking up..."
  site_data = SiteData.new

  bases = _find_file_bases_(repo_config:)
  files = _find_files_(repo_config:, site_data:)
  _export_files_(files, bases:, repo_config:, site_data:)

  # TODO export index pages
  # if index_files.length > 0
  #   log "exporting index pages..."
  # end

  _save_site_data_(repo_config:, site_data:)
end


private def _find_file_bases_(repo_config:)
  return {} if repo_config["bases / path"].nil?
  
  log "locating file bases..."
  bases = {}

  if repo_config["bases / page.svelte"]
    bases["bases / page.svelte"] = find_file_base("bases / page.svelte", repo_config:)
    if bases["bases / page.svelte"].nil?
      log error: "no base for #{BLUE}+page.svelte#{RED} found!"
    else
      log success: "found base for #{BLUE}+page.svelte#{CYAN}"
    end
  end

  if repo_config["bases / page.js"]
    bases["bases / page.js"] = find_file_base("bases / page.js", repo_config:)
    if bases["bases / page.js"].nil?
      log error: "no base for #{BLUE}+page.js#{RED} found!"
    else
      log success: "found base for #{BLUE}+page.js#{CYAN}"
    end
  end

  return bases
end


private def _find_files_(repo_config:, site_data:)
  log "locating files..."
  
  files = find_files(repo_config:)
  total = files.length

  if total.nil? or total == 0
    log error: "no files found!"
  else
    log success: "found #{total} files!"
  end

  site_data.meta[:file_count] = total

  return files
end


private def _export_files_(files, bases:, repo_config:, site_data:)
  log "exporting files..."

  total = files.length
  index_files = []

  files.each_with_index do |file, i|
    log "#{i+1}#{GREY} of #{total}: #{WHITE}#{file.parent.basename}#{GREY}/#{BLUE}#{file.basename}"

    begin
      ## process
      lines = file.readlines
      file_data = FileData.new(file, repo_config:)
      file_data = extract_data(lines:, data: file_data, repo_config:)
      next if file_data.nil?

      ## render
      content = lines.join("")
      render = render_file(content, data: file_data, repo_config:)

      ## export
      created = export_file(render, data: file_data, bases:, repo_config:)
      next unless created

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
      squark_error(e, repo_config:)
    
    end
  end
end


private def _save_site_data_(repo_config:, site_data:)
  log "saving site data..."

  site_data.meta[:page_count] = site_data.pages.length

  save_site_data(site_data.export_json, repo_config:)

  log success: "saved site data to #{BLUE}#{repo_config['out / site-data']}"
end
