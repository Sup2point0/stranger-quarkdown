require_relative "../looks"
require_relative "../../squarkdown/utils/ansi"

require_relative "out"
require_relative "wait"


def step(
  after: nil
)
  before = yield
  wait

  print PREV
  print PREV, CLEAR
  out GREY, after || before
  print CLEAR
end
