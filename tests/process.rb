require "minitest/autorun"

require_relative "../squarkdown/core/process"


class SqarkupProcess < Minitest::Test

  RepoConfig = {}

  def test_live
    content = """# Testing
<!-- #SQUARK live! -->"""

    data = extract_data(content.split, repo_config: RepoConfig)
    assert data["live"] == true
  end

  def test_dead
    content = """# Testing
<!-- #SQUARK dead! -->"""

    data = extract_data(content.split("\n"), repo_config: RepoConfig)
    assert data["live"] == false
  end

  def test_fields
    content = """# Testing
<!-- #SQUARK live!
| title = Squarkdown is awesome
| dest = testing/fields
| style = #auto / test
| duality = dark
| date = 1 April 1984
| index = tests
| shard = #index / testing
-->"""

    data = extract_data(content.split("\n"), repo_config: RepoConfig)
    assert data["title"] == "Squarkdown is awesome"
    assert data["dest"] == "testing/fields"
    assert data["style"] == ["essence", "test"]
    assert data["duality"] == "dark"
    assert data["index"] == ["tests"]
    assert data["shard"] == ["tests", "testing"]
  end

  def test_fields_default
    content = """# Testing
<!-- #SQUARK live!
| title = Squarkdown is cool
| dest = testing/defaults
-->"""

    data = extract_data(content.split("\n"), repo_config: RepoConfig)
    assert data["title"] == "Squarkdown is cool"
    assert data["dest"] == "testing/defaults"
    assert data["style"] == ["essence"]
    assert data["duality"] == "light"
    assert data["index"] == []
    assert data["shard"] == []
  end

end
