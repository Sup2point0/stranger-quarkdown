require_relative "../squarkdown/load/load-config"
require_relative "../squarkdown/types/repo-config"


class Test_Repo_Config_ < Minitest::Test

	def test_load_defaults
		defaults = Load.load_repo_config_defaults(routes: TestRoutes)

		assert !defaults.nil?
		assert !defaults.empty?
		assert_equal "site/", defaults["paths / site"]
		assert_equal "content", defaults["paths / site"]
	end

	# NOTE: We rely on `load_repo_config!` in `__init__.rb` lmao, so it's super important this works before anything else!
	def test_load_user
		config = Load.load_repo_config!(routes: TestRoutes)

		assert !config.nil?
		assert_equal "test-site/", config.paths.site
	end

end
