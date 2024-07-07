$started = false


Cols = {
  white: "\033[0m",
  grey: "\033[90m",

  red: "\033[31m",
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
  **kwargs
)
  out = Cols[:grey]

  if !$started or done
    out += ">>> #{Cols[:pink]}Ruby#{Cols[:grey]} / "
  elsif error
    out += "-------- / "
  else
    out += "         / "
  end

  if done
    out += "#{Cols[:cyan]}done!"
  elsif error
    out += "#{Cols[:red]}#{error}"
  else
    out += "#{Cols[:yellow]}#{text}"
  end

  out += Cols[:white]
  puts out
  $started = true
end
