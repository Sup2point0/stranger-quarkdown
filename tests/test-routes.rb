require "minitest/autorun"

require_relative "../squarkdown/routes"


class Test_Routes_ < Minitest::Test

  def test_paths
    assert_equal Routes.root.cleanpath, Pathname(__dir__).parent
    assert_equal Routes.site.cleanpath, (Pathname(__dir__) / "test-site")

    # FIXME: No good way to test Routes.repo?
  end

end
