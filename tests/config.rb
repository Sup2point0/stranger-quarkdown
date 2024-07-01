require "minitest/autorun"

require_relative "../squarkdown/config"


class SquarkupConfig < Minitest::Test

  def test_paths
    assert ROOT == Pathname(__dir__).parent
    assert REPO == Pathname(__dir__).parent.parent
  end

end
