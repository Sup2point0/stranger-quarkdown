require "minitest/autorun"

require_relative "../squarkdown/core/render"


class SquarkupRender < Minitest::Test

  Data = {
    title: "Squarkdown is epic",
    style: ["essence", "testing"],
  }

  RepoConfig = {
    "repo" => "Squarkdown Tests",
    "style-path" => "stranger-quarkdown/tests/resources",
  }

  
  def test_head
    content = """# Testing
    """

    out = inject_head(content, data: Data, repo_config: RepoConfig)

    assert out.include?("<title> Squarkdown is epic · Squarkdown Tests </title>")
  end


  def test_style
    content = """# Testing
    """

    out = inject_style(content, data: Data, repo_config: RepoConfig)

    assert out.include?("p {\n  font-size: 100%;\n}")
    assert out.include?(".testing {\n  color: red;\n}")
  end


  def test_repl
    content = """# Testing

<!-- #SQUARK leave? -->
unprocessed content
<!-- #SQUARK leave. -->

<!-- #SQUARK only?
processed content
      #SQUARK only. -->

<!-- #SQUARK testing! -->
    """

    out = inject_repl(content)

    assert !out.include?("unprocessed content")
    assert out.include?("processed content")
    assert !out.include?("#SQUARK only")
    assert !out.include?("#SQUARK testing")
  end

end
