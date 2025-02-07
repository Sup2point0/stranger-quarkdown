require "pathname"

require "tty-reader"
require "tty-cursor"

require_relative "../squark.version"
require_relative "looks"
require_relative "../squarkdown/utils/ansi"

require_relative "views/out"
require_relative "views/wait"
require_relative "views/step"
require_relative "views/select"


$reader = TTY::Reader.new
$cursor = TTY::Cursor

# Index of currently selected option
$i = 0
# Total number of options
$t = 0


def script
  puts

  ## intro
  print GREY, " ", PRE_START, "  #{CYAN}Welcome to Squarkdown!  #{GREY}#{LINE * 69}"
  puts

  step(
    before: "You’re running version #{PINK}#{VERSION}",
    after: "You’re running version #{VERSION}"
  )

  step(
    before: "This script will help you get your project’s #{BLUE}squarkup.json#{WHITE} set up!",
    after: "This script will help you get your project’s squarkup.json set up.",
    newline: false
  )

  step(
    before: "Don’t worry if you’re unsure what a question means, just skip it. You can always manually edit #{BLUE}squarkup.json#{WHITE} afterwards.",
    after: "Don’t worry if you’re unsure what a question means, just skip it. You can always manually edit squarkup.json afterwards."
  )

  step "Note no files will be created until the very end of the script."

  ## existing
  cwd = Pathname(__dir__).parent
  existing = false
  cwd.ascend do |dir|
    if (dir / ".squarkdown/squarkup.json").exist?
      existing = true
    end
  end

  if existing
    out
    out error: "Woah, looks like you already have an existing #{BLUE}.squarkdown/squarkup.json#{WHITE}!"
    wait

    out
    choice = select("Continue running this script?", {
      "Yes" => "all changes will overwrite any existing files.",
      "No" => ""
    })
    kill if choice == "No"
  
  else
    step(
      before: "Just a heads up, Squarkdown is for #{BLUE}SvelteKit#{WHITE} projects that use #{BLUE}MDsveX.",
      after: "Just a heads up, Squarkdown is for SvelteKit projects that use MDsveX.",
      newline: false
    ) 
    step "If this isn’t the case, Squarkdown probably isn’t the tool you’re looking for!"
  
  end

  ## configuring paths
  out
  out head: "Configuring Paths"
  wait

  step(
    before: "What’s your project’s #{YELLOW}root#{WHITE} directory? #{GREY}This will be called ROOT.",
    after: "What’s your project’s root directory?"
  )
end


def setup
  $reader.on(:keyctrl_c, :keyescape) do
    kill
  end

  $reader.on(:keyup, :keyleft) do
    $i -= 1
    if $i < 0
      $i = $t - 1
    end
  end

  $reader.on(:keydown, :keyright) do
    $i += 1
    if $i > $t - 1
      $i = 0
    end
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
