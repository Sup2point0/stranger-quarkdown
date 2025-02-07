require_relative "../looks"
require_relative "../../squarkdown/utils/ansi"


def wait(
  text = nil,
  init: false
)
  if text
    print GREY, " "*BASE_INDENT, "#{text}"
  else
    print GREY, " "*BASE_INDENT, "any key to continue"
    if init
      print ", #{BLUE}esc#{GREY} to exit"
    end
  end

  print "\r"
  out close: "", newline: false
  $reader.read_char

  print CLEAR
end
