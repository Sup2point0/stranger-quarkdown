require "json"

require_relative "../config"


def find_repo_config(from: REPO, _testing: false)
  config = load_default_repo_config()

  begin
    route = from / ".squarkdown/squarkup.json"
    content = File.read(route)
    data = JSON.parse(content)
    config.merge!(data)
  rescue
    raise if _testing
  end

  SITE = REPO / config["paths"]["site"]

  return config
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
    if source = repo_config["paths"]["source"]
      if source.is_a?(Array)
        paths = (REPO / source).map { |path| path.glob "**/*.md" }.flatten
      else
        paths = (REPO / source).glob "**/*.md"
      end
    else
      paths = REPO.glob "**/*.md"
    end
  end

  exclude = repo_config["paths"]["exclude"]

  if exclude
    paths.filter! do |path|
      (exclude.map { |pattern|
        path.realpath.to_s.match(pattern)
      }).none?
    end
  end

  return paths
end
