require "pathname"
require "minitest/autorun"

require_relative "../squarkdown/core/find"


Tests = Pathname(__dir__)
TestSite = Tests / "test-site"

RepoConfig = find_repo_config(from: Tests)
