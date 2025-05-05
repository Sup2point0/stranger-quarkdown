require "pathname"

require_relative "utils/ansi"
require_relative "utils/log"


class RoutesConfig
  attr_reader :root, :repo
  attr_accessor :site

  def initialize()
    silent = ARGV.include?("--silent")
    root = ARGV.include?("root")

    log "locating routes..." unless silent

    # root directory of Squarkdown
    @root = Pathname(__dir__).parent
    log success: "found root#{GREY} = #{BLUE}#{@root}" unless silent

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
      log error: "failed to find directory with a #{WHITE}.squarkdown/#{RED} folder" unless silent
      @repo = @root.parent
      log "set repo = #{BLUE}#{@repo}" unless silent
    else
      log success: "found repo#{GREY} = #{BLUE}#{@repo}" unless silent
    end

    # default, but can be overridden
    @site = repo / "site"
  end
end


Routes = RoutesConfig.new()
