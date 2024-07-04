require "pathname"


class RoutesConfig
  attr_reader :root, :repo, :site

  def initialize()
    @set = false

    @root = Pathname(__dir__).parent

    # assuming stranger-quarkdown is a submodule of the given repo
    @repo = @root.parent

    # default, but can be overridden
    @site = repo / "site"
  end

  def set_site(path)
    raise if @set
    @site = path
  end

end


Routes = RoutesConfig.new()
