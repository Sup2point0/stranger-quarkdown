module Squarkdown

require "find"
require "pathname"


## :: keyof RepoConfig.bases -> *Routes -> *RepoConfig -> String
#
# Find the template for `base_type` (`+page.svelte` or `+page.js`).
def self.find_base_for(base_type, routes:, repo_config:)

  filename = repo_config.bases.instance_variable_get(base_type)
  return if filename.nil? or filename.empty?

  routes.check_site_resolved()

  filepath = routes.site / repo_config.bases.path / filename
  unless filepath.exist?
    squark_error("no base for #{BLUE}#{base_type}#{RED} found!", repo_config:)
  end

  log success: "found base for #{WHITE}#{base_type}#{CYAN}: #{BLUE}#{filepath}"

  out = File.read(filepath)
  if out.nil?
    squark_error("#{WHITE}#{base_type} #{RED}appears to be empty!", repo_config:)
  end

  return out
end


## :: *Routes -> *RepoConfig -> [Pathname]
#
# Recursively search for `.md` files to squarkup.
def self.find_files_to_squarkup(routes:, repo_config:)

  from = routes.repo
  sources = repo_config.paths.sources

  paths = (
    unless sources.nil? or sources.empty?
      sources.flat_map do |path|
        if path == "." or path == "./"
          from.glob("*.md", File::FNM_DOTMATCH)
        else
          # FIXME ignore .git, etc.
          (from / path).glob("**/*.md", File::FNM_DOTMATCH)
        end
      end
    else
      from.glob("**/*.md", File::FNM_DOTMATCH)
    end
  )

  exclude = repo_config.paths.exclude

  unless exclude.nil? or exclude.empty?
    paths.filter! do |path|
      (exclude.map { |pattern|
        path.realpath.to_s.match(pattern)
      }).none?
    end
  end

  return paths
end


end
