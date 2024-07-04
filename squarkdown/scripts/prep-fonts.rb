require_relative "config"


def prep_fonts(repo_config:)
  fonts = repo_config["fonts"]
  return unless fonts

  data = fonts.map { |font| "family=" + fonts }
  text = data.join("&")
  repl = "css2?" + text + "&display=swap"

  pattern = /css2.*display=swap/
  existing = File.read(REPO / repo_config["site"] / "src/app.html")
  existing.sub!(pattern, repl)
end
