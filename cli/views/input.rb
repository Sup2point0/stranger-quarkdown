require_relative "../../squarkdown/utils/ansi"

require_relative "out"


def input(text = nil, before: nil, after: nil)
  out step: before || text
  
  prompt = " " + GREY + PRE_END + "  " + CYAN
  raw = $reader.read_line(prompt)
  print PREV, PREV, CLEAR
  out success: after || text
  out raw
  
  return raw
end
