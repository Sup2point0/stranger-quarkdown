require_relative "../config"
require_relative "../utils/log"


def export_file(content, data:, base:, repo_config:)
  route = Routes.repo / repo_config["dest"] / data.dest]

  # content.svx
  begin
    dest = route / (repo_config["file-name"] + ".svx")

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
