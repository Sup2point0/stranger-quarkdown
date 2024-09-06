require "pathname"

require_relative "utils/log"


class RoutesConfig
  attr_reader :root, :repo, :site

  def initialize()
    @set = false

    # root directory of Squarkdown
    @root = Pathname(__dir__).parent
    log "found root = #{Cols[:blue]}#{@root}"

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
      log "found repo = #{Cols[:blue]}#{@repo}"
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
