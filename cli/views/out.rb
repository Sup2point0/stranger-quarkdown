require_relative "../looks"
require_relative "../../squarkdown/utils/ansi"


def out(
  *text,
  step: nil,
  head: nil,
  close: nil,
  success: nil,
  error: nil,
  newline: true
)
  if step
    t = YELLOW + " " + PRE_STEP + SPACE + WHITE + step
  elsif head
    t = GREY + " " + PRE_SECT + SPACE + YELLOW + head + "  " + GREY + LINE * (42 - head.length)
  elsif close
    t = GREY + " " + PRE_END + SPACE + WHITE + close
  elsif success
    t = CYAN + " " + TICK + SPACE + WHITE + success
  elsif error
    t = RED + " " + CROSS + SPACE + WHITE + error
  else
    t = GREY + " " + PRE + SPACE + WHITE + text.join
  end

  print t
  puts unless !newline
  return t
end


def line
  print GREY, " ", PRE_SECT, LINE * 69
  puts
end
