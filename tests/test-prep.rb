require_relative "../squarkdown/scripts/prep-fonts"
require_relative "../squarkdown/scripts/prep-assets"
require_relative "../squarkdown/scripts/prep-scss"


class Test_Further_Features_  < Minitest::Test

  def test_prep_fonts
    path = TestSite / "src/app.html"
    before = File.read(path)

    prep_fonts(repo_config: RepoConfig)
    after = File.read(path)

    assert before != after
    assert_includes after, "display=swap"
    assert_includes after, "Test+Sans"
    assert_includes after, "</head>"

    File.write(path, before)
  end


  def test_prep_assets
    prep_assets(repo_config: RepoConfig)

    path = TestSite / "static"
    assert_path_exists path
    
    assert_path_exists (path / "squark-cover.png")
    assert_path_exists (path / "squark-icon.png")

    assert !(path / "site").exist?
    assert_path_exists (path / "arrow.svg")
    assert_path_exists (path / "copy.svg")
    assert_path_exists (path / "tick.svg")
  end


  def test_prep_scss
    prep_scss(repo_config: RepoConfig)

    path = TestSite / "scss-config.js"
    assert_path_exists path

    after = File.read(path)
    assert !after.nil?, "failed to read #{path}"

    assert_includes after, "scssConfig"
    assert_includes after, "export default scssConfig"
    assert_includes after, "includePaths"
    assert_includes after, RepoConfig["styles / path"]

    assert_includes after, "~article"
    assert_includes after, "~mixins"
    assert_includes after, "~testing"
    assert !after.include?("ignore")
    
    assert_operator (after.index "~article"), :<, (after.index "~mixins")
    assert_operator (after.index "~mixins"), :<, (after.index "~testing")

    assert_includes(after,

"
const scssConfig = {
  includePaths: [\"src/styles/\"],
  prependData: `
    @use '~article' as *;
    @use '~mixins' as *;
    @use '~testing' as *;
  `
};
export default scssConfig;
"
    )
  
  end

end
