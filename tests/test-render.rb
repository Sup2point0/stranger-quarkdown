require_relative "../squarkdown/core/render"
require_relative "../squarkdown/core/find"
require_relative "../squarkdown/types/file-data"

require_relative "shared"


def _get_data_(repo_config:)
	data = FileData.new
	data.update_fields("title = Squarkdown is epic", repo_config:)
	data.update_fields("style = #AUTO / testing", repo_config:)
	data.update_fields("clean = braces", repo_config:)
	return data
end


class SquarkupRender < Minitest::Test

	Data = _get_data_(repo_config: TestConfig)

	
	def test_head
		content = """# Testing"""

		out = Squarkdown.inject_head!(content, file_data: Data, repo_config: TestConfig)

		assert out.include?("<title> Squarkdown is epic · Squarkdown Tests </title>"), (got out)
	end

	# TODO FIXME

end
