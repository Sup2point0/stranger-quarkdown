T_START = Time.now

require_relative "__init__"
log "#{CYAN}running Squarkdown v#{VERSION}"

require_relative "core/find"
begin
  repo_config = find_repo_config()
rescue => e
  log error: e
  exit 1
end
log success: "found #{BLUE}squarkup.json"

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

require_relative "squarkup"
squarkup(repo_config:)


T_END = Time.now

log done: true
log "#{GREY}finished in #{YELLOW}#{(1000 * (T_END - T_START)).round}#{WHITE} ms"
