require "minitest/autorun"

require_relative "../squarkdown/config"


class SquarkupConfig < Minitest::Test

  def test_paths
    assert Routes.root == Pathname(__dir__).parent
    assert Routes.repo == Pathname(__dir__).parent.parent
  end

end
