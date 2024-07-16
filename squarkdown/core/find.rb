require "json"
require "json-schema"

require_relative "../config"
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
  Routes.set_site(Routes.repo / repo_config["site"])

  return repo_config
end


def load_default_repo_config()
  data = Schema["properties"].map do |prop, data|
    [prop, data["default"]]
  end.to_h
  
  return data
end


def find_file_base(file, from: Routes.site, repo_config:)
  route = from / repo_config["bases"] / repo_config[file]
  content = File.read(route)
  return content
end


def find_files(from: nil, repo_config:)
  from = Routes.repo unless !from.nil?

  if source = repo_config["sources"]
    paths = source.map {|path| (from / path).glob "**/*.md" }.flatten
  else
    paths = from.glob "**/*.md"
  end

  exclude = repo_config["exclude"]

  if exclude
    paths.filter! do |path|
      (exclude.map { |pattern|
        path.realpath.to_s.match(pattern)
      }).none?
    end
  end

  return paths
end
