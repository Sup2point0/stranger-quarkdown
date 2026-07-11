require "minitest/autorun"

require_relative "utils"

require_relative "../squarkdown/types/file-data"
require_relative "../squarkdown/core/render"
require_relative "../squarkdown/core/find"


def _get_data_(repo_config:)
  data = FileData.new
  data.update_fields("title = Squarkdown is epic", repo_config:)
  data.update_fields("style = #AUTO / testing", repo_config:)
  data.update_fields("clean = braces", repo_config:)
  return data
end


class SquarkupRender < Minitest::Test

  RepoConfig = load_default_repo_config().merge({
    "repo" => "Squarkdown Tests",
    "paths / site" => "stranger-quarkdown/tests/",
    "styles / path" => "resources/",
    "styles / page-styles" => "resources/",
    "styles / base-style" => "basic",
  })

  Data = _get_data_(repo_config: RepoConfig)

  
  def test_head
    content = """# Testing
    """

    out = inject_head(content, data: Data, repo_config: RepoConfig)

    assert out.include?("<title> Squarkdown is epic · Squarkdown Tests </title>"), (got out)
  end


  def test_style
    Routes.site = Routes.root / "tests"

    content = """# Testing
    """

    out = inject_style(content, data: Data, repo_config: RepoConfig)

    assert out.include?("resources/basic")
    assert out.include?("resources/testing")
  end


  def test_repl
    content = """# Testing

<!-- #SQUARK leave? -->
unprocessed content
<!-- #SQUARK leave. -->

<!-- #SQUARK only?
processed content
      #SQUARK only. -->
    """

    out = inject_repl(content)

    assert !out.include?("unprocessed content")
    assert out.include?("processed content")
    assert !out.include?("#SQUARK only")
  end

  def test_clean
    content = """# Testing

{surround}
{{ignore}}
"""

    out = cleanup(content, data: Data)

    assert out.include?("&amp;lbrace;surround&amp;rbrace;")
    assert out.include?("{{ignore}}")
    assert !out.include?("&amp;lbrace;ignore&amp;rbrace;")
  end

end
