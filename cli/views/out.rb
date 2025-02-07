require_relative "../looks"
require_relative "../../squarkdown/utils/ansi"


def out(
  *text,
  step: nil,
  close: nil,
  success: nil,
  error: nil,
  newline: true
)
  if step
    t = YELLOW + " " + PRE_ACTIVE + SPACE + WHITE + step
  elsif close
    t = GREY + " " + PRE_END + SPACE + WHITE + close
  elsif success
    t = CYAN + " ✓" + SPACE + WHITE + success
  elsif error
    t = RED + " ⨯" + SPACE + WHITE + error
  else
    t = GREY + " " + PRE_STEP + SPACE + WHITE + text.join
  end

  print t
  puts unless !newline
  return t
end
