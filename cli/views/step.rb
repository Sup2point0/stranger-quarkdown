require_relative "../looks"
require_relative "../../squarkdown/utils/ansi"

require_relative "out"
require_relative "wait"


def step(
  after: nil,
  newline: true
)
  out
  before = yield
  wait

  print PREV unless !newline
  print PREV, CLEAR
  out GREY, after || before
  print CLEAR
end
