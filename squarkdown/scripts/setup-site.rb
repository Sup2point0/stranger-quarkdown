### Creates a new SvelteKit project using Sup2point0/svelte-core

require "octokit"

require_relative "../utils/ansi"
require_relative "../utils/log"


def get_paths
  client = Octokit::Client.new
  contents = client.contents("Sup2point0/svelte-core")
  log contents
end


def setup_site(from:)
  log "fetching project core..."

  paths = get_paths()

  if paths.is_a?(Array) and paths.length > 0
    log success: "found project core!"
  else
    log error: "something went wrong!"
    return
  end

  log "creating site directory..."

  paths.each do |path|
    route = from / path
    route.mkpath()
  end
end
