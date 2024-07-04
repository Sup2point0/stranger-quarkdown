require "pathname"
require "minitest/autorun"

require_relative "../squarkdown/core/find"


class SquarkupFind < Minitest::Test

  Current = Pathname(__dir__)

  @@repo_config = find_repo_config(from: Current)


  def test_find_repo_config
    config = find_repo_config(
      from: Current,
      _testing: true
    )
    
    assert config
    assert config["repo"] == "Squarkdown Tests"
    assert config["dest"] == "export/routes"
    assert config["filename"] == "Content"
    assert config["exclude"].length > 0
  end

  
  def test_find_files
    files = find_files(from: Current / "content", repo_config: @@repo_config)

    assert files.length > 0
    assert files.include?(Current / "content" / "test.md")
  end


  def test_find_no_files
    files = find_files(from: Current / "resources", repo_config: @@repo_config)

    assert files.length == 0
  end


  def test_ignore_files
    files = find_files(from: Current / "ignore", repo_config: @@repo_config)

    puts files

    assert files.length == 0
  end

end
