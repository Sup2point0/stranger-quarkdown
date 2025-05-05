require "minitest/autorun"

require_relative "../squarkdown/config"


class SquarkupConfig < Minitest::Test

  def test_paths
    assert Routes.root == Pathname(__dir__).parent
    # NOTE no good way to test Routes.repo?
  end

end
