module Load
  
require "pathname"

require_relative "../types/routes"


## :: Routes
#
# Find the base routes for the rest of the project to operate relative to.
# 
# This does not resolve `routes.site`, which will need to later be read from the user's config.
def self.load_routes(internal: false)

  root = Pathname(__dir__).parent.parent
  log success: "found squarkdown: #{BLUE}#{root}"

  if internal
    repo = root
    log "#{GREY}internal: overriding `repo`"
  else
    repo = nil

    root.parent.ascend do |dir|
      if (dir / ".squarkdown").exist?
        repo = dir
        break
      end
    end
  end

  if repo.nil?
    repo = root.parent
    log error: "failed to find directory with a #{BLUE}#{CONFIG_DIR}/ #{RED}folder"
    log "defaulting your project = #{BLUE}#{repo}"
  else
    log success: "found your project: #{BLUE}#{repo}"
  end

  return RoutesData.new(root:, repo:)
end


end
