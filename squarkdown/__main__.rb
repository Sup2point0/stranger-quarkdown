T_START = Time.now

SILENT = ARGV.include? "--silent"


## == Startup ==
require_relative "utils/__include__"

require_relative "../squark.version"
log "#{CYAN}running Squarkdown v#{VERSION}"


## == Routes ==
log "resolving routes..."
require_relative "core/load-routes"

begin
  routes = Load.load_routes(internal: ARGV.include?("--root"))
rescue => e
  log error: e
  raise e
end

log success: "resolved routes!"


## == Config ==
log "configuring..."
require_relative "core/load-config"

begin
  repo_config = Load.load_repo_config!(routes:)
rescue => e
  log error: e
  raise e
end

log success: "all setup done, ready to squarkup!"


## == Extras ==
if ARGV.include? "fonts"  then require_relative "extras/prep-fonts";  Extras.prep_fonts(routes:, repo_config:) end
if ARGV.include? "assets" then require_relative "extras/prep-assets"; Extras.prep_assets(routes:, repo_config:) end
if ARGV.include? "scss"   then require_relative "extras/prep-scss";   Extras.prep_scss(routes:, repo_config:) end


## == Squarkup ==
require_relative "squarkup"
Squarkup.squarkup(routes:, repo_config:)


## == Finish ==
T_END = Time.now

log done: true
log "#{GREY}finished in #{YELLOW}#{(1000 * (T_END - T_START)).round}#{WHITE} ms"
