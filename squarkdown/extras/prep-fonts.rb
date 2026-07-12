module Extras

GOOGLE_FONTS_URL = "https://fonts.googleapis.com/"


## :: &Routes -> &RepoConfig -> Result () (Error | IO)
#
# Handle preprocessing fonts by writing the Google Fonts query to `src/app.html`.
def self.prep_fonts(routes:, repo_config:)

  log "preprocessing fonts..."

  begin
    query = self.build_query(routes:, repo_config:)

    if query.nil?
      # NOTE: Non-critical, let user know and no-op
      log error: "no Google Fonts queries provided, skipping fonts preprocessing"
      return
    end

    self.write_query(query, routes:, repo_config:)

    log success: "preprocessed fonts!"
  
  rescue => e
    squark_error(e, repo_config:)
  
  end
end


private

## :: &Routes -> &RepoConfig -> Result String Error
#
# Build the Google Fonts query, by extracting the params from `repo-config`.
def self.build_query(routes:, repo_config:)

  fonts = repo_config.fonts.queries
  raise "no fonts configured, skipping fonts preprocessing" unless fonts
  return nil if fonts.empty?
  
  params = fonts.map { |font| "family=" + font.gsub(" ", "+") }
  query = "css2?" + params.join("&") + "&display=swap"

  return query
end


## :: String -> &Routes -> &RepoConfig -> Result () Error
# Write the Google Fonts `query` to `app.html`.
def self.write_query(query, routes:, repo_config:)

  routes.check_site_resolved()

  path = routes.site / "src/app.html"
  raise "failed to find directory: #{BLUE}#{PATH}" unless path.exist?
  
  app_html = File.read(path)
  raise "#{BLUE}app.html #{RED}appears to be empty, skipping fonts preprocessing" if app_html.nil?

  pattern = /css2.*display=swap/

  if app_html.match(pattern)
    content = app_html.sub(pattern, query)
  else
    content = app_html.sub(
      /( *)(<\/head>)/,
      "\n\\1  <link rel=\"stylesheet\" href=\"#{GOOGLE_FONTS_URL}#{query}\" />\n\\1\\2"
    )
  end

    log "writing fonts query to #{BLUE}#{path}#{YELLOW}..."
  File.write(path, content)
end


end
