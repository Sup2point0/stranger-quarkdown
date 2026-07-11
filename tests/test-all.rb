class Test_Squarkup_ < Minitest::Test
    
  def test_squarkup
    require_relative "../squarkdown/squarkup"
    squarkup repo_config: RepoConfig

    path = TestSite / "src/routes/export"
    assert_path_exists (path / "testing/~content.svx")
    assert_path_exists (path / "testing/+page.svelte")
  end

end
