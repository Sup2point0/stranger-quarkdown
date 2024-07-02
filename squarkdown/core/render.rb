require_relative "squarks"


def render_file(content, data:, repo_config:)
  content = inject_head(content, data:, repo_config:)
  content = inject_style(content, data:, repo_config:)
  content = inject_repl(content)
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


def inject_style(content, data:, repo_config:)
  if path = repo_config["style-path"]
    route = REPO / path
  else
    route = REPO / "site/src/styles/sheets"
  end
  
  styles = data[:style].map do |style|
    File.read(route / "#{style}.scss")
  end

  text = """<style lang=\"scss\">

#{styles.join("\n\n")}

</style>
"""

  content += text
  return content
end


def inject_repl(content)
  Replace.each do |pattern, repl|
    content.sub!(pattern, repl)
  end

  return content
end
