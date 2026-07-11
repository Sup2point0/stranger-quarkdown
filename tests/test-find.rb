require "pathname"

require_relative "../squarkdown/core/find"


class Test_Find_ < Minitest::Test

  def test_find_repo_config
    config = find_repo_config(from: Tests)
    
    assert !config.nil?
    assert config.is_a?(Hash)
    assert config["repo"] == "Squarkdown Tests"
  end

  
  def test_find_files
    files = find_files(from: Tests.parent, repo_config: RepoConfig)

    assert_operator files.length, :>, 0
    assert_includes files, (Tests / "content/test.md")
  end


  def test_find_no_files
    files = find_files(from: Tests / "ignore", repo_config: RepoConfig)
    assert_empty files
  end


  def test_ignore_files
    files = find_files(from: Tests, repo_config: RepoConfig)

    assert files.length == 0
  end

end
