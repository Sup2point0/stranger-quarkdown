require "pathname"

require_relative "repo-config"


class RoutesData
  attr_reader(
    :root,
    :repo,
    :site,
    :has_resolved_site
  )

  
  def initialize(root:, repo:, site: nil)
    @root = root
    @repo = repo
    @site = site
    @has_resolved_site = !site.nil?
  end


  ## :: Result () Error
  def check_site_resolved()
    if !@has_resolved_site
      raise "CRITICAL: #{WHITE}routes.site #{RED}not configured, aborting"
    end
  end


  ## :: &RepoConfig -> ()
  def resolve_site(repo_config:)
    raise "cannot set `Routes.site` from #{repo_config}" unless repo_config.is_a? RepoConfigData
    
    @site = @repo / repo_config.paths.site
    raise "failed to set `Routes.site` from #{repo_config}" if @site.nil?
    raise "site directory does not exist: #{@site}" unless @site.exist?

    @has_resolved_site = true
  end
end
