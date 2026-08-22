require "pathname"
require "minitest/autorun"

require_relative "../squarkdown/types/routes"
require_relative "../squarkdown/load/load-config"


here = Pathname(__dir__)

TestRoutes = Routes.new(
	root: here.parent,
	repo: here,
	site: here / "test-site"
)

TestConfig = Load.load_repo_config!(routes: TestRoutes)
