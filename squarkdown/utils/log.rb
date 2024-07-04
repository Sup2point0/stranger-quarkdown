$started = false


Cols = {
  white: "\033[38;2;255;255;255m",
  grey: "\033[38;2;169;176;184m",
  pink: "\033[38;2;255;0;144m",
  lilac: "\033[38;2;199;199;255m",
}


def log(action = nil, error: nil, details: nil, done: false)
  if done
    puts "#{Cols[:grey]}>>> #{Cols[:pink]}Ruby #{Cols[:grey]}/ #{Cols[:lilac]}done!#{Cols[:white]}"
  elsif error
    puts "#{Cols[:grey]}-------- / #{Cols[:pink]}#{error}#{Cols[:white]}"
  elsif details
    puts "#{Cols[:grey]}---------- / #{Cols[:pink]}#{error}#{Cols[:white]}"
  elsif !$started
    puts "#{Cols[:grey]}>>> #{Cols[:pink]}Ruby #{Cols[:grey]}/ #{Cols[:lilac]}#{action}#{Cols[:white]}"
  else
    puts "         #{Cols[:grey]}/ #{Cols[:lilac]}#{action}#{Cols[:white]}"
  end

  $started = true
end
