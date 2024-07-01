started = false


def log(action)
  if !started
    puts ">>> Ruby / #{action}"
  else
    puts "        / #{action}"
  end

  started = true
end
