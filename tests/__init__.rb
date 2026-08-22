require "pathname"
require "minitest/autorun"

require_relative "../squarkdown/core/find"


here = Pathname(__dir__)

TestRoutes = Routes.new(
	root: here.parent,
	repo: here,
	site: here / "test-site"
)

TestConfig = load_repo_config(routes: TestRoutes)
