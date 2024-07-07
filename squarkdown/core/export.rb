require_relative "../config"
require_relative "../utils/log"


def export_file(content, data:, repo_config:)
  begin
    dest = Routes.repo / repo_config["dest"] / data[:dest] / (repo_config["file-name"] + ".svx")

    if !dest.exist?
      case repo_config["if-no-dir"]
        when "ignore"
          return false

        when "notify"
          log error: "destination directory does not exist: #{Cols[:blue]}#{dest.expand_path}"
          return false

        when "warn"
          log "creating destination directory: #{Cols[:blue]}#{dest.expand_path}"
          dest.parent.mkpath()

        when "create"
          dest.parent.mkpath()

      end
    end
    
    File.write(dest, content)

  rescue => e
    log "export failed!"
    log error: "#{e.class}: #{e.message}"
    return false

  else
    return true
  end
end
