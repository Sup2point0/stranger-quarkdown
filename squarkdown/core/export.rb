require_relative "../config"
require_relative "../utils/log"


def export_file(content, data:, base:, repo_config:)
  route = Routes.repo / repo_config["dest"] / data.dest

  # content.svx
  begin
    dest = route / (repo_config["file-name"] + ".svx")
    handle = repo_config["if-no-dir"]

    if !dest.exist?
      if handle.include?("ignore")
        return false
      end

      if handle.include?("warn")
        log error: "destination directory does not exist: #{Cols[:blue]}#{dest.expand_path}"
      end
        
      if handle.include?("create")
        log "creating destination directory: #{Cols[:blue]}#{dest.expand_path}"
        dest.parent.mkpath()
      end
    end
    
    File.write(dest, content)

  rescue => e
    log "failed to export `#{repo_config["file-name"]}.svx`!"
    log error: "#{e.class}: #{e.message}"
    error = true

  end

  # +page.svelte
  begin
    dest = route / "+page.svelte"
    content = base % {file: repo_config["file-name"]}
    File.write(dest, content)

  rescue => e
    log "failed to export `+page.svelte`!"
    log error: "#{e.class}: #{e.message}"
    error = true

  end

  return !error
end
