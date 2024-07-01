Replace = {
  /<!-- #SQUARK leave\?.*?#SQUARK leave\. -->/m =>
    "",

  # Constants
  # /#SQUARK font-(.*?) / =>
  #   styleMixins['\1'].join(", "),

  # Cleanup
  /<!-- #SQUARK only\?/ =>
    "",
  /#SQUARK only\. -->/ =>
    "",
  /<!-- #SQUARK.*?-->/m =>
    "",
}


def render_file(content, data:, repo_config:)
  inject_head!(content, data:, repo_config:)
  inject_style!(content, data:)

  Replace.each do |pattern, repl|
    content.sub!(pattern, repl)
  end
end


def inject_head!(content, pattern:, data:, repo_config:)
  text = """<svelte:head>
  <title> #{data[:title]} · #{repo_config["repo"]} </title>
</svelte:head>
"""

  content.sub!(pattern, text)
end


def inject_style!(content, data:)
  styles = data.style.map do |style|
    File.read "./src/styles/sheets/#{style}.scss"
  end

  text = """<style lang=\"scss\">

#{styles.join("\n\n")}

</style>
"""

  pattern = /<!-- #SQUARK style! -->/

  content.sub!(pattern, text)
end
