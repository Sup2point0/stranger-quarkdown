require "json"

require_relative "../config"
require_relative "../utils/log"


def find_repo_config(from: Routes.repo, _testing: false)
  repo_config = load_default_repo_config()

  begin
    route = from / ".squarkdown/squarkup.json"
    content = File.read(route)
    data = JSON.parse(content)
    repo_config.merge!(data)
  rescue => e
    if _testing
      raise
    else
      log error: e.message
    end
  end

  Routes.set_site(Routes.repo / repo_config["site"])

  return repo_config
end


def load_default_repo_config()
  content = File.read(Routes.root / "squarkdown/resources/repo-config-schema.json")
  schema = JSON.parse(content)
  
  data = schema["properties"].map do |prop, data|
    [prop, data["default"]]
  end.to_h
  
  return data
end


def find_files(from: nil, repo_config:)
  from = Routes.repo unless !from.nil?

  if source = repo_config["source"]
    if source.is_a?(Array)
      paths = source.map {|path| (from / path).glob "**/*.md" }.flatten
    else
      paths = (from / source).glob "**/*.md"
    end
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
