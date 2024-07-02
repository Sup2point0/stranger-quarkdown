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
    if _testing then raise end
  end

  return config
end


def load_default_repo_config()
  content = File.read(ROOT / "squarkdown/resources/repo-config-schema.json")
  schema = JSON.parse(content)
  data = schema["defaultSnippets"]["default"]
  return data
end


def find_files(from: REPO, repo_config:)
  paths = from.glob "**/*.md"
  exclude = repo_config["exclude"]

  if exclude
    paths.filter! do |path|
      (
        exclude.each do |pattern|
          path.realpath.to_s.match(pattern)
        end
      ).any?
    end
  end

  return paths
end
