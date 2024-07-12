require_relative "../config"
require_relative "../maps/squarks"
require_relative "../maps/cleanup"


def render_file(content, data:, repo_config:)
  content = inject_head(content, data:, repo_config:)
  content = inject_style(content, data:, repo_config:)
  content = inject_repl(content)
  content = cleanup(content, data:)
  return content
end


def inject_head(content, data:, repo_config:)
  text = """<svelte:head>
  <title> #{data[:title]} · #{repo_config["repo"]} </title>
</svelte:head>
"""

  content = text + content
  return content
end


def inject_data(content, data:)
  fields = data.map do |field, value|
    "#{field}: #{value}"
  end
  
  text = """---
#{fields.join('\n')}
---
"""
  
  content = text + content
  return content
end


def inject_style(content, data:, repo_config:)
  path = repo_config["page-styles"]
  route = Routes.site / path
  
  styles = data[:style].map do |style|
    "@use './#{path}/#{style}' as *;"
  end

  content = if content.include?("<style")
    then
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


def cleanup(content, data:)
  if clean = data[:clean]
    if clean.include?("braces")
      content = Cleanup[:braces].call(content)
    end
  end

  return content
end
