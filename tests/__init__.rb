require "pathname"

require "minitest/autorun"
require "minitest/reporters"

require_relative "../squarkdown/types/routes"
require_relative "../squarkdown/load/load-config"


Tests = Pathname(__dir__)

TestRoutes = Routes.new(
	root: Tests.parent,
	repo: Tests,
	site: Tests / "test-site"
)

TestConfig = Load.load_repo_config!(routes: TestRoutes)


Minitest::Reporters.use! [Minitest::Reporters::DefaultReporter.new(:color => true)]
