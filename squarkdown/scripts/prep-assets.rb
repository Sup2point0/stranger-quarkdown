require "fileutils"

require_relative "../config"
require_relative "../utils/log"


Globs = [
  "**/*.png",
  "**/*.jpg",
  "**/*.jpeg",
  "**/*.svg",
]


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

  files = []
  Globs.each do |glob|
    files += route.glob(glob)
  end

  log "located #{files.length} assets"

  files.each do |file|
    rel = file.relative_path_from(route)
    dest = Routes.site / "static" / rel

    if !dest.exist?
      dest.dirname.mkpath
    end

    FileUtils.cp(file, dest)
  end
end
