require "date"

require_relative "../config"
require_relative "../utils/ansi"
require_relative "../utils/log"
require_relative "../utils/error"


def prep_scss(repo_config:)
  log "preprocessing SCSS..."

  begin
    try_prep_scss(repo_config:)
  rescue => e
    error(e, repo_config:)
  end
end


def try_prep_scss(repo_config:)
  partial = Pathname.new(repo_config["styles / path"])
  route = Routes.site / partial
  raise "styles path not found" unless route

  files = route.glob("**/~*.scss")
  total = files.length

  data = files.each_with_index.map do |file, i|
    log "#{i+1}#{GREY} of #{total}: #{WHITE}#{file.parent.basename}#{GREY}/#{BLUE}#{file.basename}"
    dest = file.relative_path_from(route)
    full = partial / dest
    "@use './#{full}' as *;"
  end

  data.sort! do |prot, deut|
    prot.length <=> deut.length
  end

  text = """/// SCSS Config
/// Last generated #{Date.today.strftime("%d %B %Y")}

const scssConfig = {
  prependData: `
    #{data.join("\n    ")}
  `
};
export default scssConfig;
"""

  File.write(Routes.site / "scss-config.js", text)
end
