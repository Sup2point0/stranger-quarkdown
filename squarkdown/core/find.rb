require "json"
require "json-schema"

require_relative "../config"
require_relative "../utils/ansi"
require_relative "../utils/log"


Schema = JSON.parse(
  File.read(Routes.root / "squarkdown/resources/squarkup-schema.json")
)


def find_repo_config(from: Routes.repo)
  repo_config = load_default_repo_config()

  route = from / ".squarkdown/squarkup.json"
  content = File.read(route)
  data = JSON.parse(content)

  JSON::Validator.validate!(Schema, data)
  
  repo_config.merge!(data)
  Routes.site = Routes.repo / repo_config["paths / site"]

  return repo_config
end


def load_default_repo_config()
  data = Schema["properties"].map do |prop, data|
    [prop, data["default"]]
  end.to_h
  
  return data
end


## Find template for `+page.svelte`
def find_file_base(file, from: Routes.site, repo_config:)
  return nil if repo_config[file].nil?
  route = from / repo_config["bases / path"] / repo_config[file]
  content = File.read(route)
  return content
end


## Find .md files to squarkup
def find_files(from: nil, repo_config:)
  from = Routes.repo unless !from.nil?

  sources = repo_config["paths / sources"]
  if sources && !sources.empty?
    paths = sources.map do |path|
      if path == "." || path == "./"
        from.glob "*.md"
      else
        (from / path).glob "**/*.md"
      end
    end
    paths = paths.flatten
  else
    paths = from.glob("**/*.md", File::FNM_DOTMATCH)
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
