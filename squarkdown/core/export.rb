require_relative "../utils/log"


def export_file(content, data:, repo_config:)
  begin
    dest = repo_config["paths"]["dest"] / data["dest"] / repo_config["file-name"]
    File.write(dest, content)
  rescue => e
    log "export failed!"
    log error: "#{e.class}: #{e.message}"
    return false
  else
    return true
  end
end
