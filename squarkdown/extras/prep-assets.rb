module Extras

require "fileutils"


## :: *Routes -> *RepoConfig -> ()
def self.prep_assets(routes:, repo_config:)

  log "preprocessing assets..."

  begin
    self.try_prep_assets(routes:, repo_config:)
  rescue => e
    squark_error(e, repo_config:)
  end
end


private 

## :: *Routes -> *RepoConfig -> ()
def self.try_prep_assets(routes:, repo_config:)
  assets_dir = routes.repo / repo_config.assets.path
  unless assets_dir.exist?
    raise "#{WHITE}assets / path #{RED}directory does not exist: #{BLUE}#{assets_dir}"
  end

  site_assets_dir = routes.repo / repo_config.assets.site_assets
  unless site_assets_dir.exist?
    # NOTE: Non-critical, let user know and fallback
    log error: "#{WHITE}assets / site-assets #{RED}directory does not exist: #{BLUE}#{site_assets_dir}"
  end

  extensions = repo_config.assets.extensions

  files = assets_dir.glob("**/*.{#{extensions.join(',')}}", File::FNM_DOTMATCH)
  total = files.length

  if total == 0
    # NOTE: Non-critical, let user know and no-op
    log error: "no assets found in #{BLUE}#{assets_dir}, skipping asset preprocessing"
    return
  else
    log success: "found #{total} assets in #{BLUE}#{assets_dir}"
  end

  dest_dir = routes.site / "static"
  log "copying assets to #{BLUE}#{dest_dir}#{YELLOW}..."

  files.each_with_index do |file, i|
    if (i % 10 == 0) or (i+1 == total)
      log "#{i+1}#{GREY} of #{total}..."
    end
    
    filepath = file.relative_path_from(assets_dir)

    # FIXME could be cleaner
    site_rel = nil
    unless site_assets_dir.nil?
      if file.parent.relative_path_from(site_assets_dir) == Pathname(".") 
        site_rel = file.relative_path_from(site_assets_dir)
      end
    end
  
    dest = dest_dir / (site_rel or filepath)

    dest.dirname.mkpath() unless dest.exist?

    FileUtils.cp(file, dest)
  end

  log success: "preprocessed assets!"
end


end
