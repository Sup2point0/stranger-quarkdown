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
  
  data = schema["properties"].map do |prop, data|
    [prop, data["default"]]
  end.
  data = data.to_h
  
  return data
end


def find_files(from: nil, repo_config:)
  if from.nil?
    if repo_config["source"]
      from = REPO / repo_config["source"]
    else
      from = REPO
    end
  end

  paths = from.glob "**/*.md"
  exclude = repo_config["exclude"]

  puts "found paths: #{paths}"

  if exclude
    paths.filter! do |path|
      (exclude.map { |pattern|
        puts "pattern: #{pattern}"
        puts "path: #{path.realpath.to_s}"
        path.realpath.to_s.match(pattern)
      }).none?
    end
  end

  puts "filtered paths: #{paths}"

  return paths
end
