require "minitest/autorun"

require_relative "../squarkdown/core/process"


class SquarkupProcess < Minitest::Test

  RepoConfig = {}

  def test_live
    content = """# Testing
<!-- #SQUARK live! -->"""

    data = extract_data(
      lines: content.split("\n"),
      repo_config: RepoConfig,
      fill_defaults: false
    )
    assert data.live == true
  end

  def test_dead
    content = """# Testing
<!-- #SQUARK dead! -->"""

    data = extract_data(
      lines: content.split("\n"),
      repo_config: RepoConfig,
      fill_defaults: false
    )
    assert data.nil?
  end

  def test_fields
    content = """# Testing
<!-- #SQUARK live!
| dest = testing/fields
| capt = A unit test
| title = Squarkdown is awesome
| desc = Making sure everything works
| style = #AUTO / test
| duality = dark
| index = tests
| shard = #INDEX / testing
| date = 1984 April 1
-->"""

    data = extract_data(
      lines: content.split("\n"),
      repo_config: RepoConfig
    )

    assert data.dest == "testing/fields"
    assert data.head == "Testing"
    assert data.capt == "A unit test"
    assert data.title == "Squarkdown is awesome"
    assert data.desc == "Making sure everything works"
    assert data.style == ["article", "test"]
    assert data.duality == "dark"
    assert data.index == ["tests"]
    assert data.shard == ["tests", "testing"]
    assert data.date
  end

  def test_fields_default
    content = """# Testing
<!-- #SQUARK live!
| dest = testing/defaults
-->"""

    data = extract_data(
      lines: content.split("\n"),
      repo_config: RepoConfig
    )

    assert data.dest == "testing/defaults"
    assert data.head == "Testing"
    assert data.title == "Testing"
    assert data.style == ["article"]
    assert data.duality == "light"
    assert data.index == []
    assert data.shard == []
  end

end
