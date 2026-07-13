module Squarkdown

require_relative "../maps/squarks"
require_relative "../maps/cleanup"


## :: FileContent -> *FileData -> *RepoConfig -> FileContent
#
# Render `content` using `file_data:` and `repo_config:`, returning the content to be written to the `.svx` file.
def self.render_file!(content, file_data:, repo_config:)
  self.inject_repl!(content)
  self.cleanup!(content, file_data:)
  self.fix_links!(content)
  self.inject_index!(content, file_data:)
  self.inject_head!(content, file_data:, repo_config:)
  self.inject_script!(content, file_data:, repo_config:)
  self.inject_style!(content, file_data:, repo_config:)
  self.inject_data!(content, file_data:)
  return content
end


def self.inject_repl!(content)
  Replace.each do |pattern, repl|
    content.gsub!(pattern, repl)
  end
end


def self.cleanup!(content, file_data:)
  if clean = file_data.clean
    if clean.include?("braces")
      Cleanup[:braces].call(content)
    end

    if clean.include?("angles")
      Cleanup[:angles].call(content)
    end
  end
end


def self.fix_links!(content)
  content.gsub!(/(\.\.\/)*\.?assets\/(\.?site\/)?/, '{base}/')
  content.gsub!(/\.md(#[^\)]*?)?\)/, '\1)')
end


def self.inject_head!(content, file_data:, repo_config:)
  content.prepend "<svelte:head>
  <title> #{file_data.title} · #{repo_config.core.repo} </title>
</svelte:head>

"
end


def self.inject_index!(content, file_data:)
  content.gsub!(/<!-- ?#SQUARK index~ ?-->/, "<IndexView />")
end


def self.inject_script!(content, file_data:, repo_config:)
  content.prepend "<script>

import { base } from \"$app/paths\";
#{self._import_index_(file_data:, repo_config:)}
</script>

"
end


def self._import_index_(file_data:, repo_config:)
  return (
    file_data.isIndex ?
      "import IndexView from \"#{repo_config.bases.index_svelte}\";"
    : ""
  )
end


def self.inject_style!(content, file_data:, repo_config:)
  path = repo_config.styles.page_styles
  return if path.nil?
  
  styles = file_data.style.map do |style|
    "@use '#{File.join(path, style)}' as *;"
  end

  content = if content.include?("<style") and content.include?(("</style>")) then
    content.sub!(/<style( lang="scss")?>/,
      "<style lang=\"scss\">

#{styles.join("\n")}
")
  
  else content + "
<style lang=\"scss\">

#{styles.join("\n")}
</style>
"
  end
end


def self.inject_data!(content, file_data:)
  content.prepend(file_data.to_yaml)
end

end
