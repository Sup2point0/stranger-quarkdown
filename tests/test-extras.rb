# require_relative "../squarkdown/extras/prep-fonts"
# require_relative "../squarkdown/extras/prep-assets"
# require_relative "../squarkdown/extras/prep-scss"


# class Test_Extras_  < Minitest::Test

# 	def test_prep_fonts
# 		out = TestRoutes.site / "src/app.html"
# 		before = File.read(out)

# 		Extras.prep_fonts(routes: TestRoutes, repo_config: TestConfig)
# 		after = File.read(out)

# 		assert before != after
# 		assert_includes after, "display=swap"
# 		assert_includes after, "Test+Sans"
# 		assert_includes after, "</head>"

# 		File.write(out, before)
# 	end


# 	def test_prep_assets
# 		Extras.prep_assets(routes: TestRoutes, repo_config: TestConfig)

# 		out = TestRoutes.site / "static"
# 		assert_path_exists out
		
# 		assert_path_exists (out / "squark-cover.png")
# 		assert_path_exists (out / "squark-icon.png")

# 		assert !(out / "site").exist?
# 		assert_path_exists (out / "arrow.svg")
# 		assert_path_exists (out / "copy.svg")
# 		assert_path_exists (out / "tick.svg")
# 	end


# 	def test_prep_scss
# 		Extras.prep_scss(routes: TestRoutes, repo_config: TestConfig)

# 		out = TestRoutes.site / "scss-config.js"
# 		assert_path_exists out

# 		after = File.read(out)
# 		assert !after.nil?, "failed to read #{out}"

# 		assert_includes after, "scssConfig"
# 		assert_includes after, "export default scssConfig"
# 		assert_includes after, "includePaths"
# 		assert_includes after, TestConfig.styles.path

# 		assert_includes after, "~article"
# 		assert_includes after, "~mixins"
# 		assert_includes after, "~testing"
# 		assert !after.include?("ignore")
		
# 		assert_operator (after.index "~article"), :<, (after.index "~mixins")
# 		assert_operator (after.index "~mixins"), :<, (after.index "~testing")

# 		assert_includes(after,

# "
# const scssConfig = {
# 	includePaths: [\"src/styles/\"],
# 	prependData: `
# 		@use '~article' as *;
# 		@use '~mixins' as *;
# 		@use '~testing' as *;
# 	`
# };
# export default scssConfig;
# "
# 		)
	
# 	end

# end
