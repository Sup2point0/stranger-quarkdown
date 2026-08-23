class Test_Squarkup_ < Minitest::Test
		
	def test_squarkup
		require_relative "../squarkdown/squarkup"
		Squarkup.squarkup(routes: TestRoutes, repo_config: TestConfig)

		out = TestRoutes.site / "src/routes/export"
		assert_path_exists (out / "testing/~content.svx")
		assert_path_exists (out / "testing/+page.svelte")
	end

end
