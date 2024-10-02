require "fileutils"

require_relative "../config"
require_relative "../utils/log"


def prep_assets(repo_config:)
  log "preprocessing assets..."

  begin
    try_prep_assets(repo_config:)
  rescue => e
    log error: e.to_s
  end
end


def try_prep_assets(repo_config:)
  route = Routes.repo / repo_config["assets"]
  raise "asset path not found" unless route

  if repo_config["site-assets"]
    site_route = Routes.repo / repo_config["site-assets"]
  else
    site_route = nil
  end

  files = route.glob(
    "**/*.{png,jpg,jpeg,svg}",
    File::FNM_DOTMATCH
  )
  i = 0
  total = files.length

  log success: "found #{total} assets in #{Cols[:blue]}#{route.relative_path_from(Routes.root)}"
  log "transferring assets..."

  files.each do |file|
    i += 1
    if i % 10 == 0
      log "#{i}#{Cols[:grey]} of #{total}..."
    end
    rel = file.relative_path_from(route)

    site_rel = nil
    if !site_route.nil?
      if file.parent.relative_path_from(site_route) == Pathname(".") 
        site_rel = file.relative_path_from(site_route)
      end
    end
    
    dest = Routes.site / "static" / (site_rel or rel)

    if !dest.exist?
      dest.dirname.mkpath()
    end

    FileUtils.cp(file, dest)
  end

  log success: "transferred all assets!"
end
