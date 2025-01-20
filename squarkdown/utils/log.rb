$started = false


Cols = {
  white: "\033[0m",
  grey: "\033[90m",

  red: "\033[31m",
  green: "\033[92m",
  yellow: "\033[93m",
  blue: "\033[94m",
  pink: "\033[95m",
  cyan: "\033[96m"
}


def log(
  text = nil,
  success: nil,
  error: nil,
  done: false,
  time: nil,
  **kwargs
)
  out = Cols[:grey]

  if !$started
    out += ">>> #{Cols[:pink]}Squark#{Cols[:grey]} / "
  elsif done
    out += ">>> #{Cols[:pink]}Squark#{Cols[:grey]} ✓ "
  elsif success
    out += "           #{Cols[:cyan]}✓ "
  elsif error
    out += "           #{Cols[:red]}⨯ "
  else
    out += "           / "
  end

  if success
    out += "#{Cols[:cyan]}#{success}"
  elsif error
    out += "#{Cols[:red]}#{error}"
  elsif done
    out += "#{Cols[:cyan]}done!"
  elsif time
    out += "#{Cols[:grey]}finished in #{Cols[:yellow]}#{time}#{Cols[:grey]} ms"
  else
    out += "#{Cols[:yellow]}#{text}"
  end

  out += Cols[:white]
  puts out
  $started = true
end
