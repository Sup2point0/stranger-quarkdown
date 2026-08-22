require_relative "../squarkdown/core/find"

require_relative "../squarkdown/types/routes"


class Test_Find_ < Minitest::Test
	
	def test_finds_files
		files = Squarkdown.find_files_to_squarkup(routes: TestRoutes, repo_config: TestConfig)

		puts "files = #{files}"

		assert_operator files.length, :>, 0
		assert_includes files, (Tests / "content/absent.md")
		assert_includes files, (Tests / "content/dead.md")
		assert_includes files, (Tests / "content/test.md")
	end
	
	def test_finds_files_nested
		files = Squarkdown.find_files_to_squarkup(routes: TestRoutes, repo_config: TestConfig)

		assert_equal    5, files.length
		assert_includes files, (Tests / "content/nested/surprise.md")
	end

	def test_ignores_files
		files = Squarkdown.find_files_to_squarkup(routes: TestRoutes, repo_config: TestConfig)

		assert !files.include?(Tests / "ignore/_ignore.md")
		assert !files.include?(Tests / "ignore/-ignore.md")
	end

	def test_ignores_files_nested
		files = Squarkdown.find_files_to_squarkup(routes: TestRoutes, repo_config: TestConfig)

		assert_includes files, (Tests / "ignore/nested/keep.md")

		assert !files.include?(Tests / "ignore//nested/_still-ignore.md")
		assert !files.include?(Tests / "ignore//nested/-still-ignore.md")
	end

end
