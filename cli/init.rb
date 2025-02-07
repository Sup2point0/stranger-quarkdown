require "pathname"

require "tty-reader"

require_relative "../squark.version"
require_relative "looks"
require_relative "../squarkdown/utils/ansi"

require_relative "views/out"
require_relative "views/line"
require_relative "views/wait"
require_relative "views/step"
require_relative "views/select"


$reader = TTY::Reader.new


def script
  puts

  ## intro
  print GREY, " ", PRE_START, "  #{CYAN}Welcome to Squarkdown!"
  puts
  step after: "You’re running version #{VERSION}" do
    out "You’re running version #{PINK}#{VERSION}"
  end

  step after: "This script will help you get your project’s squarkup.json set up.", newline: false do
    out "This script will help you get your project’s #{BLUE}squarkup.json#{WHITE} set up!"
  end
  
  step after: "Don’t worry if you’re unsure what a question means, just skip it. You can always manually edit squarkup.json afterwards." do
    out "Don’t worry if you’re unsure what a question means, just skip it. You can always manually edit #{BLUE}squarkup.json#{WHITE} afterwards."
  end

  step after: "Note no files will be created until the very end of the script." do
    out "Note no files will be created until the very end of the script."
  end

  out

  ## existing
  cwd = Pathname(__dir__).parent
  existing = false
  cwd.ascend do |dir|
    if (dir / ".squarkdown/squarkup.json").exist?
      existing = true
    end
  end

  if existing
    out error: "Woah, looks like you already have an existing #{BLUE}.squarkdown/squarkup.json#{WHITE}!"
    wait
    out "Continue running this script?"
    select({
      "Yes" => "All changes will overwrite any existing files.",
      "No" => ""
    })
  end
end


def setup
  $reader.on(:keyctrl_c, :keyescape) do
    kill
  end
end


def kill(created: false)
  print CLEAR
  out
  out close: "#{CYAN}Thanks for using Squarkdown#{if created then ', enjoy' else '' end}!#{WHITE} 🥕"
  puts
  exit
end



setup
script
kill created: true
