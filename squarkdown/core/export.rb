##
# Export a processed `.md` file to ``.svx``, as well as its `+page.svelte` and `+page.js` if desired.
def export_file(content, data:, bases:, routes:, repo_config:)
  route = routes.site / repo_config.paths.dest / data.dest
  filename = repo_config.out.file_name

  ## == .svx ==
  begin
    dest = route / (filename + ".svx")
    handle = repo_config.opts.on_no_dir

    if !route.exist?
      if handle.include?("ignore")
        return false
      end

      if handle.include?("warn")
        dest_display = dest.relative_path_from(routes.repo)
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
    log "failed to export `#{filename}.svx`!"
    squark_error(e, repo_config:)
    error = true

  end

  ## == +page.svelte ==
  if bases["page.svelte"]
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
  if bases["page.js"]
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


## :: *SiteData -> *Routes -> *RepoConfig -> ()
def save_site_data(data, routes:, repo_config:)

  log "saving site data..."

  dest = routes.site / repo_config.out.site_data
  unless dest.exist?
    squark_error("#{WHITE}out / site-data #{RED}does not exist: #{BLUE}#{dest}", repo_config:)
  end

  File.write(dest, data)

  log success: "saved site data to #{BLUE}#{repo_config.out.site_data}"
end
