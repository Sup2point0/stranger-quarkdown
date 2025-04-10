require "pathname"

require "tty-reader"
require "tty-cursor"

require_relative "looks"
require_relative "views/out"
require_relative "views/wait"
require_relative "views/step"
require_relative "views/select"
require_relative "views/input"

require_relative "../squark.version"
require_relative "../squarkdown/utils/ansi"
require_relative "../squarkdown/core/find"


$reader = TTY::Reader.new
$cursor = TTY::Cursor

# Index of currently selected option
$i = 0
# Total number of options
$t = 0


def script
  config = load_default_repo_config

  puts

  ## intro
  print GREY, " ", PRE_START, "  #{CYAN}Welcome to Squarkdown!  #{GREY}#{LINE * 42}"
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
    choice = select("Continue running this script?", options: {
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

  out
  choice = select(
    before: "What directory should Squarkdown output to? #{GREY} This will be called SITE. It’s usually the directory where your site lives.",
    after: "What directory should Squarkdown output to?",
    options: {
      "site/" => "",
      ".site/" => "",
      "_site/" => "",
      "other" => "enter manually"
    }
  )

  if choice == "other"
    out
    config["paths / root"] = input(
      "What directory should Squarkdown output to?"
    )
  else
    config["paths / root"] = choice
  end

  out
  choice = select(
    before: "Where should Squarkdown look for Markdown files to export? #{GREY} You can choose directories to exclude in the next question.",
    after: "Where should Squarkdown look for Markdown files to export?",
    options: {
      "project root (+all subdirectories)" => "default",
      "project root (only)" => "",
      "specific directories (configure manually)" => ""
    }
  )
  
  config["paths / sources"] = case choice
    when "project root (+all subdirectories)"
      [""]
    when "project root (only)"
      ["."]
    else
      []
  end

  out
  choice = select(
    before: "Which directories should Squarkdown ignore? #{GREY} The entire path of the file will be matched against this pattern(s).",
    after: "Which directories should Squarkdown ignore?",
    options: {
      "none" => "",
      "project root" => "",
      "directories starting with ." => "",
      "directories starting with _" => "",
    }
  )
  
  sep = "(/|\\\\)"
  config["paths / exclude"] = case choice
    when "project root"
      ["."]
    when "directories starting with ."
      [sep + "\..*?" + sep]
    when "directories starting with _"
      [sep + "_.*?" + sep]
    else
      []
  end

  ## exporting files
  out
  out head: "Exporting Files"
  wait

  out
  choice = select(
    "Where should Squarkdown export the JSON file containing all site data, and what should it call the file?",
    options: {
      "/src/site.json" => "default",
      "/src/site-data.json" => "",
      "/src/data/site.json" => ""
    }
  )

  config["out / site-data"] = choice

  out
  choice = select(
    before: "When Squarkdown exports #{BLUE}.md#{WHITE} files, what should it name the output #{BLUE}.svx#{WHITE} file?",
    after: "When Squarkdown exports .md files, what should it name the output .svx file?",
    options: {
      "~content.svx" => "default",
      "_content.svx" => "",
      "content.svx" => "",
      "other" => "enter manually"
    }
  )
  
  if choice == "other"
    config["out / file-name"] = ""  # TODO other
  else
    config["out / file-name"] = choice
  end

  out
  choice = select(
    before: "Should Squarkdown also auto-generate a #{BLUE}+page.svelte~#{WHITE} or #{BLUE}+page.js~#{WHITE} for each exported #{BLUE}.md~#{WHITE} file?",
    after: "Should Squarkdown also auto-generate a +page.svelte~ or +page.js~ for each exported .md~ file?",
    options: {
      "+page.svelte, page.js" => "",
      "+page.svelte only" => "",
      "+page.js only" => "",
      "vary by page" => "configure manually",
      "no" => "",
    }
  )

  ## autogeneration
  if choice.include? "page."
    out
    choice = select(
      "Squarkdown will need templates for these other auto-generated files. Where can it find them?",
      options: {
        "src/lib/bases/" => "",
        "src/parts/bases/" => "",
        "other" => "enter manually",
        "cancel" => "",
      }
    )

    case choice
      when "cancel"
      when "other"
        # TODO enter manually
      else
        config["bases / path"] = choice
    end

    out
    choice = select(
      "When Squarkdown applies page styles, where should it get the stylesheets from?",
      options: {
        "src/styles/" => "",
        "src/styles/bases" => "",
        "src/styles/pages" => "",
        "other" => "enter manually",
        "skip" => "",
      }
    )

    case choice
      when "skip"
      when "other"
        # TODO
      else
        config["styles / path"] = choice
    end
  end

  ## extra features
  out
  out head: "Extras"
  wait

  out
  choice = select(
    "Any other features you’d like to enable?",
    options: {
      "assets preprocessing" => "copy assets from project root to site/static/",
      "SCSS preprocessing" => "collect global SCSS stylesheets into a SCSS config for SvelteKit",
      "Google Fonts preprocessing" => "automatically build Google Fonts query",
    },
    multi: true,
  )

  if choice and choice.include? "assets preprocessing"
    out
    choice = select(
      before: "Where should Squarkdown look for assets? #{GREY} Squarkdown will copy the contents of this directory to #{BLUE}static/#{GREY}.",
      after: "Where should Squarkdown look for assets?",
      options: {
        "./assets/" => "",
        "./.assets/" => "",
        "other" => "enter manually",
        "cancel" => "",
      }
    )

    case choice
      when "cancel"
      when "other"
        # TODO
      when "./assets/"
        config["assets / path"] = choice
        assets = "assets/"
      when "./.assets/"
        config["assets / path"] = choice
        assets = ".assets/"
    end

    out
    choice = select(
      before: "Do you have a folder containing site-specific assets? #{GREY} These files will be copied directly to the root of #{BLUE}static/#{GREY}.",
      after: "Do you have a folder containing site-specific assets?",
      options: {
        "#{assets || '/'}site/" => "",
        "#{assets || '/'}.site/" => "",
        "other" => "enter manually",
        "cancel" => "",
      }
    )

    case choice
      when "cancel"
      when "other"
        # TODO enter manually
      else
        config["assets / site"] = choice
    end

    out
    choice = select(
      "What types of assets should Squarkdown copy?",
      options: {
        ".png" => true,
        ".jpg / .jpeg" => true,
        ".svg" => "",
        ".webp" => "",
        "other" => "configure manually",
        "cancel" => "",
      },
      multi: true,
    )

    case choice
      when "cancel"
      when "other"
        config["assets / extensions"] = []
      else
        config["assets / extensions"] = choice
    end
  end

  ## final touches
  out
  out head: "Final Touches"
  wait

  out
  choice = select(
    "How should Squarkdown handle errors when processing files?",
    options: {
      "kill" => "stop execution",
      "warn" => "log error and skip file",
    }
  )

  config["opts / on-error"] = choice

  out
  choice = select(
    "What should Squarkdown do when an output directory doesn’t exist?",
    options: {
      "warn + create the directory" => "notify, but create the directory",
      "ignore + create the directory" => "silently create the directory",
      "warn + do nothing" => "notify and skip file",
      "ignore" => "silently skip file",
    }
  )

  config["opts / on-missing-dir"] = case choice
    when "warn + create the directory"
      ["warn", "create"]
    when "ignore + create the directory"
      ["ignore", "create"]
    when "warn + do nothing"
      ["warn"]
    else
      ["ignore"]
  end

  ## finalise
  out ""
  out "Finalising..."

  print PREV, CLEAR
  out success: "Your #{BLUE}squarkup.json#{WHITE} has been created in #{BLUE}./.squarkdown/#{WHITE}."
  wait
  
  step "If you selected options that needed to be filled manually, you can find them indicated with placeholder comments."
  step(
    before: "For more help or guidance, please visit the docs on GitHub at #{CYAN}https://sup2point0.github.io/stranger-quarkdown/docs",
    after: "For more help or guidance, please visit the docs on GitHub at https://sup2point0.github.io/stranger-quarkdown/docs"
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
