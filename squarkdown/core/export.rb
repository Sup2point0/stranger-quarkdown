require_relative "../config"
require_relative "../utils/log"


def export_file(content, data:, base:, repo_config:)
  route = Routes.repo / repo_config["dest"] / data.dest
  created = false

  # content.svx
  begin
    dest = route / (repo_config["file-name"] + ".svx")
    handle = repo_config["if-no-dir"]

    if !route.exist?
      if handle.include?("ignore")
        return false
      end

      if handle.include?("warn")
        log error: "destination directory does not exist: #{Cols[:blue]}#{dest.expand_path}"
      end
        
      if handle.include?("create")
        log "creating destination directory..."
        route.mkpath()
        created = true
      else
        return false
      end
    end
    
    File.write(dest, content)
    log "created #{Cols[:blue]}#{dest}" if created

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
