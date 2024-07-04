require_relative "../config"
require_relative "../utils/log"


def prep_scss(repo_config:)
  log "preprocessing assets..."

  begin
    try_prep_scss(repo_config:)
  rescue => e
    log error: e.to_s
  end
end


def try_prep_scss(repo_config:)
  partial = repo_config["styles"]
  route = Routes.site / partial
  raise "styles path not found" unless route

  files = route.glob("**/~*.scss")
  data = files.map do |sheet|
    dest = file.relative_path_from(route)
    "@use './#{partial}/#{dest}' as *;"
  end

  data = data.join("\n")

  text = """/// SCSS Config
/// Last generated #{}

const scssConfig = `
#{data.join("\n")}
`;
export default scssConfig;
"""

  File.write(Routes.site / "scss-config.js", text)
end
