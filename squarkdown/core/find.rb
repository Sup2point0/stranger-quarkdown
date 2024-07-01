require "json"


def find_repo_config()
  config = load_default_repo_config()

  begin
    route = REPO / ".squarkdown" / "repo-config.json"
    content = File.read(route)
    data = JSON.parse(content)
    config.merge!(data)
  rescue
  end

  return config
end


def load_default_repo_config()
  content = File.read(ROOT / "resources" / "repo-config-schema.json")
  schema = JSON.parse(content)
  data = schema["defaultSnippets"]
  return data
end


def find_files(repo_config:)
  paths = REPO["**/*.md"]
  exclude = repo_config["exclude"]

  if exclude
    paths.filter! do |path|
      (exclude.each { |pattern| path.match(pattern) }).any?
    end
  end
end
