require "pathname"

require "tty-reader"

require_relative "../squark.version"
require_relative "../squarkdown/utils/ansi"
require_relative "views/line"
require_relative "views/step"


$reader = TTY::Reader.new


def script
  puts
  line

  ## intro
  step "#{YELLOW}Welcome to Squarkdown! #{WHITE}You’re running version #{PINK}#{VERSION}", init: true
  step "This script will help you get your project’s #{BLUE}squarkup.json#{WHITE} set up!"
  step "Don’t worry if you’re unsure what a question means, just skip it. You can always manually edit #{BLUE}squarkup.json#{WHITE} afterwards."
  step "Note no files will be created until the very end of the script."

  puts

  ## existing
  cwd = Pathname(__dir__).parent
  existing = false
  cwd.ascend do |dir|
    if (dir / ".squarkdown/squarkup.json").exist?
      existing = true
    end
  end

  if existing
    out "Woah, looks like you already have an existing #{BLUE}.squarkdown/squarkup.json#{WHITE}!", col: RED
    step "Continue running this script?", prompt: "All changes will overwrite any existing files."
  end
end


def setup
  $reader.on(:keyctrl_c, :keyescape) do
    kill
  end
end


def kill(created: false)
  puts "#{GREY}>> #{WHITE}Thanks for using Squarkdown#{if created then ', enjoy' else '' end}! 🥕"
  line
  puts
  exit
end



setup
script
kill created: true
