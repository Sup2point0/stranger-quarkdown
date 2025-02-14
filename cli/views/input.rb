require_relative "../../squarkdown/utils/ansi"

require_relative "out"


def input(before:, after: nil)
  out step: before
  
  prompt = " " + GREY + PRE_END + "  " + CYAN
  raw = $reader.read_line(prompt)
  print PREV, PREV, CLEAR
  out success: after || before
  out raw
  
  return raw
end
