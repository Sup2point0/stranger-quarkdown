module Squarkdown


## :: FileContent -> *FileData -> Hash FileContent -> *Routes -> *RepoConfig -> Bool
#
# Export a processed `.md` file to `.svx`, as well as its `+page.svelte` and `+page.js` if desired.
def self.export_file(content, file_data:, bases:, routes:, repo_config:)

  if content.empty?
    raise "received no content to render for #{file_data.path}"
  end

  out_dir = routes.site / repo_config.paths.dest / file_data.dest
  out_dir.mkpath()

  self._export_svx_(content, out_dir:, routes:, repo_config:)
  self._export_svelte_(out_dir:, bases:, routes:, repo_config:)
  self._export_js_(out_dir:, bases:, routes:, repo_config:)

end

## :: FileContent -> Pathname -> *Routes -> *RepoConfig -> ()
def self._export_svx_(content, out_dir:, routes:, repo_config:)

  filepath = out_dir / (repo_config.out.file_name + ".svx")
  self._export_component_(content, filepath:, routes:, repo_config:)

end

## :: Pathname -> Hash Content -> *Routes -> *RepoConfig -> ()
def self._export_svelte_(out_dir:, bases:, routes:, repo_config:)

  content = bases["page.svelte"]
  return if content.nil?

  filepath = out_dir / "+page.svelte"
  self._export_component_(content, filepath:, routes:, repo_config:)

end

## :: Pathname -> Hash Content -> *Routes -> *RepoConfig -> ()
def self._export_js_(out_dir:, bases:, routes:, repo_config:)

  content = bases["page.js"]
  return if content.nil?

  filepath = out_dir / "+page.js"

  self._export_component_(content, filepath:, routes:, repo_config:)
  
end

## :: FileContent -> Pathname -> *Routes -> *RepoConfig -> ()
def self._export_component_(content, filepath:, routes:, repo_config:)

  if filepath.exist?
    case repo_config.opts.on_file_exists
    when "kill"
      raise "destination file already exists: #{filepath}"
    
    when "skip"
      log error: "destination file already exists: #{BLUE}#{filepath}#{RED}, skipping..."
    
    when "overwrite"
      log "#{BLACK}overwriting file: #{filepath.relative_path_from(routes.repo)}"
      
      File.write(filepath, content)
    
    end
  end

end


## :: *SiteData -> *Routes -> *RepoConfig -> ()
def self.save_site_data(data, routes:, repo_config:)

  log "saving site data..."

  dest = routes.site / repo_config.out.site_data
  unless dest.dirname.exist?
    squark_error("#{WHITE}out / site-data #{RED}does not exist: #{BLUE}#{dest}", repo_config:)
  end

  File.write(dest, data)

  log success: "saved site data to #{BLUE}#{repo_config.out.site_data}"
end


end
