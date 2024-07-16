require_relative "../config"
require_relative "../maps/squarks"
require_relative "../maps/cleanup"


def render_file(content, data:, repo_config:)
  content = inject_repl(content)
  content = cleanup(content, data:)
  content = inject_head(content, data:, repo_config:)
  content = inject_base(content)
  content = inject_style(content, data:, repo_config:)
  content = inject_data(content, data:)
  content = fix_links(content)
  return content
end


def inject_head(content, data:, repo_config:)
  return """<svelte:head>
  <title> #{data.title} · #{repo_config["repo"]} </title>
</svelte:head>

""" + content
end


def inject_base(content)
  return """<script>

import { base } from \"$app/paths\";

</script>

""" + content
end


def inject_style(content, data:, repo_config:)
  path = repo_config["page-styles"]
  route = Routes.site / path
  
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


def inject_repl(content)
  Replace.each do |pattern, repl|
    content.sub!(pattern, repl)
  end

  return content
end


def inject_data(content, data:)
  return data.to_yaml + content
end


def fix_links(content)
  content = content.gsub(/(\.\.\/)*\.assets(\/\.site)?/, "{base}")
  content = content.gsub(/\.md\]/, "]")
  return content
end


def cleanup(content, data:)
  if clean = data.clean
    if clean.include?("braces")
      content = Cleanup[:braces].call(content)
    end
  end

  return content
end
