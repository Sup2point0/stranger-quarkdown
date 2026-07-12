module Squarkup

require_relative "types/__include__"
require_relative "utils/__include__"
require_relative "core/__include__"


## :: *Routes -> *RepoConfig -> ()
# 
# Run squarkup on a repository.
def self.squarkup(routes:, repo_config:)

  if repo_config.paths.sources.nil? and repo_config.paths.exclude.nil?
    log error: "no #{WHITE}paths / sources #{RED}or #{WHITE}paths / exclude #{RED}set in #{BLUE}squarkup.json"
    log hint: "Squarkdown needs at least 1 of these set to run"
    log done: true
    return
  end

  log "squarking up..."

  site_data = SiteData.new

  bases = self.find_file_bases(routes:, repo_config:)
  files = self.find_files(routes:, repo_config:)
  
  site_data.meta[:file_count] = files.length

  self.export_files(files, bases:, routes:, repo_config:, site_data:)
  
  site_data.meta[:page_count] = site_data.pages.length

  # TODO export index pages
  # if index_files.length > 0
  #   log "exporting index pages..."
  # end

  save_site_data(site_data.export_json, routes:, repo_config:)
end


private

## :: *Routes -> *RepoConfig -> Hash Content
def self.find_file_bases(routes:, repo_config:)

  return {} if repo_config.bases.path.nil?
  
  log "locating file bases..."

  return {
    "page.svelte" => Squarkdown.find_base_for(:@page_svelte, routes:, repo_config:),
    "page.js"     => Squarkdown.find_base_for(:@page_js, routes:, repo_config:),
  }
end


## :: *Routes -> *RepoConfig -> [Pathname]
def self.find_files(routes:, repo_config:)

  log "locating files..."
  
  files = Squarkdown.find_files_to_squarkup(routes:, repo_config:)
  total = files.length

  if total == 0
    log error: "no files found!"
  else
    log success: "found #{total} files!"
  end

  return files
end


## :: [Pathname] -> *Routes -> *RepoConfig -> *mut SiteData -> Hash String -> ()
def self.export_files(files, bases:, routes:, repo_config:, site_data:)
  log "exporting files..."

  total = files.length
  index_files = []

  files.each_with_index do |file, i|
    log "#{i+1}#{GREY} of #{total}: #{WHITE}#{file.parent.basename}#{GREY}/#{BLUE}#{file.basename}"

    begin
      ## process
      lines = file.readlines
      file_data = FileData.new(file, routes:, repo_config:)
      file_data = extract_data(lines:, data: file_data, repo_config:)
      next if file_data.nil?

      ## render
      content = lines.join("")
      render = render_file(content, data: file_data, repo_config:)

      ## export
      created = export_file(render, data: file_data, bases:, routes:, repo_config:)
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
      file_data.tags.each do |tag|
        site_data.update_tags(tag:, page: file_data.path)
      end

    rescue => e
      squark_error(e, repo_config:)
    
    end
  end
end


end
