require_relative "../routes"
require_relative "../utils/ansi"
require_relative "../utils/log"
require_relative "../utils/error"


##
# Export a processed `.md` file to ``.svx``, as well as its `+page.svelte` and `+page.js` if desired.
def export_file(content, data:, bases:, repo_config:)
  route = Routes.site / repo_config["paths / dest"] / data.dest
  filename = repo_config["out / file-name"]

  ## == .svx ==
  begin
    dest = route / (filename + ".svx")
    handle = repo_config["opts / on-no-dir"]

    if !route.exist?
      if handle.include?("ignore")
        return false
      end

      if handle.include?("warn")
        dest = dest.relative_path_from(Routes.repo)
        log error: "destination directory does not exist: #{BLUE}#{dest}"
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
    log "failed to export `#{filename}.svx`!"
    squark_error(e, repo_config:)
    error = true

  end

  ## == +page.svelte ==
  if bases["bases / page.svelte"]
    begin
      dest = route / "+page.svelte"
      content = bases["bases / page.svelte"] % {file: filename}
      File.write(dest, content)

    rescue => e
      log "failed to export `+page.svelte`!"
      squark_error(e, repo_config:)
      error = true

    end
  end

  ## == +page.js ==
  if bases["bases / page.js"]
    begin
      dest = route / "+page.js"
      content = bases["bases / page.js"] % {file: filename}
      File.write(dest, content)

    rescue => e
      log "failed to export `+page.js`!"
      squark_error(e, repo_config:)
      error = true
  
    end
  end

  return !error
end


def save_site_data(data, repo_config:)
  route = Routes.site / repo_config["out / site-data"]
  File.write(route, data)
end
