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
  print GREY, " ", PRE_START
  line

  ## intro
  out "#{CYAN}Welcome to Squarkdown! #{WHITE}You’re running version #{PINK}#{VERSION}"
  out

  step after: "This script will help you get your project’s squarkup.json set up." do
    out "This script will help you get your project’s #{BLUE}squarkup.json#{WHITE} set up!"
  end
  
  step(
    before: "Don’t worry if you’re unsure what a question means, just skip it. You can always manually edit #{BLUE}squarkup.json#{WHITE} afterwards.",
    after: "Don’t worry if you’re unsure what a question means, just skip it. You can always manually edit squarkup.json afterwards."
  )

  step before: "Note no files will be created until the very end of the script."
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
