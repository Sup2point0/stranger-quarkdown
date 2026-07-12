T_START = Time.now

SILENT = ARGV.include? "silent"


## == Startup ==
require_relative "utils/ansi"
require_relative "utils/error"
require_relative "utils/log"


require_relative "../squark.version"
log "#{CYAN}running Squarkdown v#{VERSION}"


## == Routes ==
log "finding routes..."
require_relative "core/load-routes"

begin
  routes = Load.load_routes internal: (ARGV.include? "root")
rescue => e
  log error: e
  raise e
end

log success: "resolved routes!"


## == Config ==
log "finding #{BLUE}squarkup.json#{YELLOW}..."
require_relative "core/load-config"

begin
  repo_config = Load.load_repo_config!(routes:)
rescue => e
  log error: e
  raise e
end

log success: "found #{BLUE}squarkup.json"


## == Further Features ==
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


## == Squarkup ==
require_relative "squarkup"
squarkup(repo_config:)


## == Finish ==
T_END = Time.now

log done: true
log "#{GREY}finished in #{YELLOW}#{(1000 * (T_END - T_START)).round}#{WHITE} ms"
