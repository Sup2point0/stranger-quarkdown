require_relative "../../squarkdown/utils/ansi"


def out(text, col: nil)
  print GREY
  print ">> "
  print WHITE
  puts text
end


def step(
  text,
  col: nil,
  init: false,
  prompt: nil
)
  out text

  if prompt
    print "   #{GREY}#{prompt}"
  else
    print "   #{GREY}any key to continue"
    if init
      print ", #{BLUE}esc#{GREY} to exit"
    end
  end

  print "\r   "
  $reader.read_char
  print CLEAR
end
