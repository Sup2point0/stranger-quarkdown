$started = false


Cols = {
  white: "\033[38;2;255;255;255m",
  grey: "\033[38;2;169;176;184m",
  pink: "\033[38;2;255;0;144m",
  lilac: "\033[38;2;199;199;255m",
}


def log(action = nil, error: nil, done: false)
  if done
    puts "#{Cols[:grey]}>>> #{Cols[:pink]}Ruby #{Cols[:grey]}/ #{Cols[:lilac]}done!"
  elsif error
    puts "#{Cols[:grey]}-------- / #{Cols[:pink]}#{error}"
  elsif !$started
    puts "#{Cols[:grey]}>>> #{Cols[:pink]}Ruby #{Cols[:grey]}/ #{Cols[:lilac]}#{action}"
  else
    puts "        #{Cols[:grey]}/ #{Cols[:lilac]}#{action}"
  end

  puts Cols[:white]

  $started = true
end
