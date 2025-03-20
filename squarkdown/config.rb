require "pathname"

require_relative "utils/log"


class RoutesConfig
  attr_reader :root, :repo, :site

  def initialize()
    @set = false

    # root directory of Squarkdown
    @root = Pathname(__dir__).parent
    log success: "found root#{GREY} = #{BLUE}#{@root}"

    # find directory with .squarkdown folder
    @repo = nil

    @root.parent.ascend do |dir|
      if (dir / ".squarkdown").exist?
        @repo = dir
        break
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

  def set_site(path)
    raise if @set
    @site = path
  end

end


Routes = RoutesConfig.new()
