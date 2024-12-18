require_relative "../config"
require_relative "../utils/log"


def export_file(content, data:, base:, repo_config:)
  route = Routes.site / repo_config["dest"] / data.dest

  # content.svx
  begin
    dest = route / (repo_config["file-name"] + ".svx")
    dest_display = dest.relative_path_from(Routes.repo)
    handle = repo_config["if-no-dir"]

    if !route.exist?
      if handle.include?("ignore")
        return false
      end

      if handle.include?("warn")
        log error: "destination directory does not exist: #{Cols[:blue]}#{dest_display}"
      end
        
      if handle.include?("create")
        log "creating destination directory..."
        route.mkpath()
      else
        return false
      end
    end
    
    File.write(dest, content)

  rescue => e
    log "failed to export `#{repo_config["file-name"]}.svx`!"
    log error: "#{e.class}: #{e.message}"
    error = true

  end

  # +page.svelte
  if base["page.svelte"]
    begin
      dest = route / "+page.svelte"
      content = base["page.svelte"] % {file: repo_config["file-name"]}
      File.write(dest, content)

    rescue => e
      log "failed to export `+page.svelte`!"
      log error: "#{e.class}: #{e.message}"
      error = true

    end
  end

  # +page.js
  if base["page.js"]
    begin
      dest = route / "+page.js"
      content = base["page.js"] % {file: repo_config["file-name"]}
      File.write(dest, content)

    rescue => e
      log "failed to export `+page.js`!"
      log error: "#{e.class}: #{e.message}"
      error = true
  
    end
  end

  return !error
end
