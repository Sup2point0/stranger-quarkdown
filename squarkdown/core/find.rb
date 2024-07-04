require "json"

require_relative "../config"


def find_repo_config(from: Routes.repo, _testing: false)
  repo_config = load_default_repo_config()

  begin
    route = from / ".squarkdown/squarkup.json"
    content = File.read(route)
    data = JSON.parse(content)
    repo_config.merge!(data)
  rescue
    raise if _testing
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
  if from.nil?
    if source = repo_config["source"]
      if source.is_a?(Array)
        paths = (Routes.repo / source).map { |path| path.glob "**/*.md" }.flatten
      else
        paths = (Routes.repo / source).glob "**/*.md"
      end
    else
      paths = Routes.repo.glob "**/*.md"
    end
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
