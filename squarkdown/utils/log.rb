require_relative "ansi"


$started = false


def log(
  text = nil,
  success: nil,
  error: nil,
  done: false,
  **kwargs
)
  out = GREY

  if !$started
    out += ">>> #{PINK}Squark#{GREY} / "
  elsif done
    out += ">>> #{PINK}Squark#{GREY} / "
  elsif success
    out += "           #{CYAN}✓ "
  elsif error
    out += "           #{RED}⨯ "
  else
    out += "           / "
  end

  if success
    out += "#{CYAN}#{success}"
  elsif error
    out += "#{RED}#{error}"
  elsif done
    out += "#{CYAN}done!"
  else
    out += "#{YELLOW}#{text}"
  end

  out += WHITE
  puts out
  $started = true
end
