def export_file(content, data:, repo_config:)
  dest = repo_config["dest"] / data["dest"] / repo_config["filename"]
  File.write(dest, content)
end
