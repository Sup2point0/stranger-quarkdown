require_relative "../config"
require_relative "../utils/log"


def prep_fonts(repo_config:)
  log "preprocessing fonts..."

  begin
    try_prep_fonts(repo_config:)
  rescue => e
    log error: e.to_s
  end
end


def prep_fonts(repo_config:)
  fonts = repo_config["fonts"]
  raise "no fonts configured" unless fonts

  data = fonts.map { |font| "family=" + font }
  text = data.join("&")
  repl = "css2?" + text + "&display=swap"

  path = REPO / repo_config["site"] / "src/app.html"
  pattern = /css2.*display=swap/
  
  existing = File.read(path)
  content = existing.sub(pattern, repl)
  File.write(path, content)
end
