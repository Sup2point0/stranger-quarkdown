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


  ## :: &RepoConfig -> ()
  def resolve_site(repo_config:)
    raise "cannot set `Routes.site` from #{repo_config}" unless repo_config.is_a? RepoConfigData
    
    @site = repo_config.paths.site
    raise "failed to set `Routes.site` from #{repo_config}" if @site.nil?

    @has_resolved_site = true
  end
end
