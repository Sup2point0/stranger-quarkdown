require "minitest/autorun"

require_relative "../squarkdown/core/render"


class SquarkupRender < Minitest::Test

  Data = {
    title: "Squarkdown is epic",
  }

  RepoConfig = {
    "repo" => "Squarkdown Tests"
  }

end
