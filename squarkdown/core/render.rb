require_relative "../config"
require_relative "../maps/squarks"
require_relative "../maps/cleanup"


def render_file(content, data:, repo_config:)
  content = inject_repl(content)
  content = cleanup(content, data:)
  content = fix_links(content)
  content = inject_index(content, data:)
  content = inject_head(content, data:, repo_config:)
  content = inject_script(content, data:, repo_config:)
  content = inject_style(content, data:, repo_config:)
  content = inject_data(content, data:)
  return content
end


def inject_repl(content)
  Replace.each do |pattern, repl|
    content.sub!(pattern, repl)
  end

  return content
end


def cleanup(content, data:)
  if clean = data.clean
    if clean.include?("braces")
      content = Cleanup[:braces].call(content)
    end

    if clean.include?("angles")
      content = Cleanup[:angles].call(content)
    end
  end

  return content
end


def fix_links(content)
  content = content.gsub(/(\.\.\/)*\.?assets\/(\.?site\/)?/, "{base}/")
  content = content.gsub(/\.md\]/, "]")
  return content
end


def inject_head(content, data:, repo_config:)
  return """<svelte:head>
  <title> #{data.title} · #{repo_config["repo"]} </title>
</svelte:head>

""" + content
end


def inject_index(content, data:)
  return content.gsub(/<!-- ?#SQUARK index~ ?-->/, "<IndexView />")
end


def inject_script(content, data:, repo_config:)
  return """<script>

import { base } from \"$app/paths\";
#{_import_index_(data:, repo_config:)}
</script>

""" + content
end


def _import_index_(data:, repo_config:)
  return (
    if data.isIndex
      then "import IndexView from \"#{repo_config["index-view"]}\";"
    else ""
    end
  )
end


def inject_style(content, data:, repo_config:)
  path = repo_config["page-styles"]
  if path.nil? then return content end
  
  styles = data.style.map do |style|
    "@use './#{path}/#{style}' as *;"
  end

  content = if content.include?("<style") then
    content.sub(/<style( lang="scss")?>/,
      """<style lang=\"scss\">

#{styles.join("\n")}
""")
  
  else content + """
<style lang=\"scss\">

#{styles.join("\n")}
</style>
"""

  end

  return content
end


def inject_data(content, data:)
  return data.to_yaml + content
end
