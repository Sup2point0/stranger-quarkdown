GOOGLE_FONTS_URL = "https://fonts.googleapis.com/"


## Handle preprocessing fonts by writing the Google Fonts query to `app.html`.
def prep_fonts(repo_config:)
  log "preprocessing fonts..."

  begin
    try_prep_fonts(repo_config:)
  rescue => e
    squark_error(e, repo_config:)
  end
end


def try_prep_fonts(repo_config:)
  query = build_query(repo_config:)
  write(query, repo_config:) if query
end


## Build the Google Fonts query params, by reading the fonts in `squarkup.json``.
def build_query(repo_config:)
  fonts = repo_config["fonts / queries"]
  raise "no fonts configured" unless fonts

  if fonts.empty?
    nil
  else
    data = fonts.map { |font| "family=" + font }
    "css2?" + data.join("&") + "&display=swap"
  end
end

## Write the Google Fonts `query` to `app.html`.
def write(query, repo_config:)
  log "Routes.site = #{Routes.site}"
  path = Routes.site / "src/app.html"
  # log "path = #{path}"
  pattern = /css2.*display=swap/
  
  app_html = File.read(path)

  if app_html.match pattern
    content = app_html.sub(pattern, query)
  else
    content = app_html.sub(
      /( *)(<\/head>)/,
      "\n\\1  <link rel=\"stylesheet\" href=\"#{GOOGLE_FONTS_URL}#{query}\" />\n\\1\\2"
    )
  end

  File.write(path, content)
end
