require "pathname"
require "minitest/autorun"

require_relative "../squarkdown/scripts/prep-fonts"
require_relative "../squarkdown/scripts/prep-assets"
require_relative "../squarkdown/scripts/prep-scss"


class Test_Further_Features < Minitest::Test

  Current = Pathname(__dir__)

  RepoConfig = find_repo_config(from: Current)


  def test_prep_fonts
    prep_fonts(repo_config: RepoConfig)
  end


  def test_prep_assets
    prep_assets()

    route = Current / "static"
    assert route.exist?
    assert (route / "test.png").exist?
    assert (route / "test.jpeg").exist?
    assert (route / "test.svg").exist?
    assert (route / ".include/test.png").exist?
  end


  def test_prep_scss
    scss = load_scss()
    assert scss.is_a?(Array)
    assert scss.length == 2
    assert scss.include?(Current / "resources" / "~test.scss")
    assert scss.include?(Current / "resources" / "~testing.scss")
    assert !scss.include?(Current / "resources" / "ignore.scss")
  end

end
