require "pathname"

require_relative "utils/ansi"
require_relative "utils/log"


class RoutesConfig
  attr_reader :root, :repo
  attr_accessor :site

  def initialize()
    root = ARGV.include?("root")

    log "locating routes..."

    # root directory of Squarkdown
    @root = Pathname(__dir__).parent
    log success: "found root#{GREY} = #{BLUE}#{@root}"

    if root
      @repo = @root
    else
      # find directory with .squarkdown folder
      @repo = nil

      @root.parent.ascend do |dir|
        if (dir / ".squarkdown").exist?
          @repo = dir
          break
        end
      end
    end

    if @repo.nil?
      log error: "failed to find directory with a #{WHITE}.squarkdown/#{RED} folder"
      @repo = @root.parent
      log "set repo = #{BLUE}#{@repo}"
    else
      log success: "found repo#{GREY} = #{BLUE}#{@repo}"
    end

    # default, but can (and should) be overridden
    @site = @repo / "site"
  end
end
