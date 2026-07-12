require "pathname"
require "json"
require "json-schema"

require_relative "../utils/ansi"
require_relative "../utils/log"


SquarkupSchema = JSON.parse(
  File.read(
    Routes.root / "squarkdown/resources/squarkup-schema.json"
  )
)


##
# Look under `from` for a `.squarkdown/squarkup.json`, and produce a complete repo config, with absent values filled in by defaults.
#
# This also sets the global `Routes.site` with the value provided by `squarkup.json`.
def find_repo_config(from: Routes.repo)
  repo_config = load_default_repo_config()

  route = from / ".squarkdown/squarkup.json"
  raise "could not find #{CYAN}squarkup.json" if !route.exist?

  content = File.read(route)

  data = JSON.parse(content)

  JSON::Validator.validate!(SquarkupSchema, data)
  
  repo_config.merge!(data)
  Routes.site = Routes.repo / repo_config["paths / site"]

  return repo_config
end


def load_default_repo_config()
  data = SquarkupSchema["properties"].map do |prop, data|
    [prop, data["default"]]
  end.to_h
  
  return data
end


##
# Find template for `base_type` (`+page.svelte` or `+page.js`)
def find_file_base(base_type, from: Routes.site, repo_config:)
  filepath = repo_config[base_type]
  return nil if filepath.nil? or filepath.empty?

  route = from / repo_config["bases / path"] / filepath
  raise "no base for #{BLUE}#{base_type}#{RED} found!" unless route.exist?

  out = File.read(route)
  log success: "found base for #{BLUE}#{base_type}#{CYAN}"

  return out
end


##
# Recursively search for `.md` files to squarkup
def find_files(from: nil, repo_config:)
  from = Routes.repo unless !from.nil?

  sources = repo_config["paths / sources"]

  if !sources.nil? && !sources.empty?
    paths = sources.map do |path|
      if path == "." || path == "./"
        from.glob "*.md", File::FNM_DOTMATCH
      else
        (from / path).glob "**/*.md", File::FNM_DOTMATCH
      end
    end
    paths = paths.flatten
  else
    paths = from.glob "**/*.md", File::FNM_DOTMATCH
  end

  exclude = repo_config["paths / exclude"]

  if exclude
    paths.filter! do |path|
      (exclude.map { |pattern|
        path.realpath.to_s.match(pattern)
      }).none?
    end
  end

  return paths
end
