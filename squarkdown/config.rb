require "pathname"

require_relative "utils/log"


class RoutesConfig
  attr_reader :root, :repo
  attr_accessor :site

  def initialize()
    log "locating routes..."

    # root directory of Squarkdown
    @root = Pathname(__dir__).parent
    log success: "found root#{GREY} = #{BLUE}#{@root}"

    # find directory with .squarkdown folder
    @repo = nil

    @root.ascend do |dir|
      if (dir / ".squarkdown").exist?
        @repo = dir
      end
    end

    if @repo.nil?
      raise "failed to find directory with a .squarkdown/ folder"
    else
      log success: "found repo#{GREY} = #{BLUE}#{@repo}"
    end

    # default, but can be overridden
    @site = repo / "site"
  end
end


Routes = RoutesConfig.new()
