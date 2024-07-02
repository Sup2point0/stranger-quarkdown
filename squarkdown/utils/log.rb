$started = false


def log(action = nil, error: nil, done: false)
  if done
    puts ">>> Ruby / done!"
  elsif error
    puts "-------- / #{error}"
  elsif !$started
    puts ">>> Ruby / #{action}"
  else
    puts "        / #{action}"
  end

  $started = true
end
