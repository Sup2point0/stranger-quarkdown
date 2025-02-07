require_relative "../looks"
require_relative "../../squarkdown/utils/ansi"

require_relative "out"
require_relative "wait"


def step(
  text = nil,
  before: nil,
  after: nil,
  newline: true
)
  out
  out step: before || text
  wait

  print PREV unless !newline
  print PREV, CLEAR
  out GREY, after || text

  print CLEAR
end
