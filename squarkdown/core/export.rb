require_relative "../config"
require_relative "../utils/ansi"
require_relative "../utils/log"


def export_file(content, data:, base:, repo_config:)
  route = Routes.site / repo_config["paths / dest"] / data.dest

  # content.svx
  begin
    dest = route / (repo_config["out / file-name"] + ".svx")
    dest_display = dest.relative_path_from(Routes.repo)
    handle = repo_config["opts / on-no-dir"]

    if !route.exist?
      if handle.include?("ignore")
        return false
      end

      if handle.include?("warn")
        log error: "destination directory does not exist: #{BLUE}#{dest_display}"
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
    log "failed to export `#{repo_config["out / file-name"]}.svx`!"
    log error: "#{e.class}: #{e.message}"
    error = true

  end

  # +page.svelte
  if base["bases / page.svelte"]
    begin
      dest = route / "+page.svelte"
      content = base["bases / page.svelte"] % {file: repo_config["out / file-name"]}
      File.write(dest, content)

    rescue => e
      log "failed to export `+page.svelte`!"
      log error: "#{e.class}: #{e.message}"
      error = true

    end
  end

  # +page.js
  if base["bases / page.js"]
    begin
      dest = route / "+page.js"
      content = base["bases / page.js"] % {file: repo_config["out / file-name"]}
      File.write(dest, content)

    rescue => e
      log "failed to export `+page.js`!"
      log error: "#{e.class}: #{e.message}"
      error = true
  
    end
  end

  return !error
end
