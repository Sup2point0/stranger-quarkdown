require "pathname"

require "tty-reader"
require "tty-cursor"

require_relative "looks"
require_relative "views/out"
require_relative "views/wait"
require_relative "views/step"
require_relative "views/select"
require_relative "views/input"
require_relative "final"

require_relative "../squark.version"
require_relative "../squarkdown/utils/ansi"
require_relative "../squarkdown/core/find"


$reader = TTY::Reader.new
$cursor = TTY::Cursor

# Index of currently selected option
$i = 0
# Total number of options
$t = 0

$created = false


class AbandonFeature < StandardError
end


def script
  config = load_default_repo_config

  puts

  ## == Intro ==
  print GREY, " ", PRE_START, "  #{CYAN}Welcome to Squarkdown!  #{GREY}#{LINE * 42}"
  puts

  step(
    before: "You’re running version #{PINK}#{VERSION}",
    after:  "You’re running version #{VERSION}"
  )

  step(
    before: "This script will get your project’s #{BLUE}squarkup.json#{WHITE} set up!",
    after:  "This script will get your project’s squarkup.json set up!",
    newline: false,
  )

  step(
    "Disclaimer: This CLI is still a work in progress. Apologies if there are bugs!",
  )

  step(
    before: "Don’t worry if you’re unsure what a question means, just skip it. You can always manually edit #{BLUE}squarkup.json#{WHITE} afterwards.",
    after:  "Don’t worry if you’re unsure what a question means, just skip it. You can always manually edit squarkup.json afterwards."
  )

  step "No file will be created until the very end of the script."

  ## == Check Existing ==
  cwd = Pathname(__dir__).parent
  existing = false
  cwd.ascend do |dir|
    if (dir / ".squarkdown/squarkup.json").exist?
      # NOTE: This is to skip Squarkdown's own .squarkdown files
      next if dir.to_s.end_with?("stranger-quarkdown")

      existing = existing || dir
    end
  end

  if existing
    out
    out error: "Woah, looks like you already have an existing #{BLUE}squarkup.json#{WHITE} in #{BLUE}#{existing}/.squarkdown/#{WHITE}!"
    wait

    out
    choice = select("Continue running this script?", options: {
      "No"  => "",
      "Yes" => "all changes will overwrite any existing files.",
    })
    kill if choice == "No"
  
  else
    step(
      before: "Just a heads up, Squarkdown is for #{BLUE}SvelteKit#{WHITE} projects that use #{BLUE}MDsveX.",
      after:  "Just a heads up, Squarkdown is for SvelteKit projects that use MDsveX.",
      newline: false
    ) 
    step "If this isn’t the case, Squarkdown probably isn’t the tool you’re looking for!"
  
  end

  ## == Configuring Paths ==
  out
  out head: "Configuring Paths"
  wait

  out
  choice = select(
    before: "What directory should Squarkdown output to? #{GREY} This will be called SITE. It’s usually the directory where your site lives.",
    after:  "What directory should Squarkdown output to?",
    options: {
      "site/"  => "default",
      ".site/" => "",
      "_site/" => "",
      "other"  => "enter manually"
    }
  )

  config["paths / site"] = case choice
    when "other" then input "What directory should Squarkdown output to?"
    else choice
  end

  out
  choice = select(
    before: "Where should Squarkdown look for Markdown files to export? #{GREY} You can choose directories to exclude in the next question.",
    after:  "Where should Squarkdown look for Markdown files to export?",
    options: {
      "project root + all subdirectories" => "default",
      "project root only"                 => "",
      "other"                             => "enter manually"
    }
  )
  
  config["paths / sources"] = case choice
    when "project root + all subdirectories" then [""]
    when "project root only"                 then ["."]
    when "other"
      paths = input(
        before: "Where should Squarkdown look for Markdown files to export? #{GREY} Provide directories separated by spaces.",
        after: "Where should Squarkdown look for Markdown files to export?"
      )
      paths.split
  end

  out
  choice = select(
    before: "Which directories should Squarkdown ignore? #{GREY} The entire path of the file will be matched against this pattern(s).",
    after:  "Which directories should Squarkdown ignore?",
    options: {
      "none"                        => "",
      "project root"                => "",
      "directories starting with ." => "",
      "directories starting with _" => "",
    }
  )
  
  sep = "(/|\\\\)"
  config["paths / exclude"] = case choice
    when "project root"                then ["."]
    when "directories starting with ." then [sep + "\\..*?" + sep]
    when "directories starting with _" then [sep + "_.*?" + sep]
    else []
  end

  ## == Exporting Files ==
  out
  out head: "Exporting Files"
  wait

  out
  choice = select(
    "Where should Squarkdown export the JSON file containing all site data?",
    options: {
      "/src/site.json"      => "default",
      "/src/site-data.json" => "",
      "/src/data/site.json" => "",
      "other"               => "enter manually",
    }
  )

  config["out / site-data"] = case choice
    when "other" then input "Where should Squarkdown export the JSON file containing all site data?"
    else choice
  end

  out
  choice = select(
    before: "When Squarkdown exports #{BLUE}.md#{WHITE} files, what should it name the output #{BLUE}.svx#{WHITE} file?",
    after:  "When Squarkdown exports .md files, what should it name the output .svx file?",
    options: {
      "~content.svx" => "default",
      "_content.svx" => "",
      "content.svx"  => "",
      "other"        => "enter manually"
    }
  )
  
  config["out / file-name"] = case choice
    when "other" then input "When Squarkdown exports .md files, what should it name the output .svx file?"
    else choice
  end

  out
  choice = select(
    before: "Should Squarkdown also auto-generate a #{BLUE}+page.svelte#{WHITE} or #{BLUE}+page.js#{WHITE} for each exported #{BLUE}.md#{WHITE} file?",
    after:  "Should Squarkdown also auto-generate a +page.svelte or +page.js for each exported .md file?",
    options: {
      "+page.svelte + page.js" => "",
      "+page.svelte"           => "",
      "+page.js"               => "",
      "vary by page"           => "configure manually",
      "no"                     => "skip this feature",
    }
  )
  
  ## == Autogeneration ==
  if (choice.include? "page.") then begin
    out
    choice = select(
      "Squarkdown needs templates for these other auto-generated files. Where can it find them?",
      options: {
        "src/lib/bases/"   => "",
        "src/parts/bases/" => "",
        "other"            => "enter manually",
        "cancel"           => "skip this feature",
      }
    )

    config["bases / path"] = case choice
      when "cancel" then raise AbandonFeature
      when "other"  then input "Where can Squarkdown find templates for auto-generated files?"
      else choice
    end

    out
    choice = select(
      "When Squarkdown applies page styles to these auto-generated pages, where should it get the stylesheets from?",
      options: {
        "src/styles/"     => "",
        "src/lib/styles/" => "",
        "other"           => "enter manually",
        "cancel"          => "skip this feature",
      }
    )

    config["styles / page-styles"] = case choice
      when "cancel" then raise AbandonFeature
      when "other" then input "Where should Squarkdown get stylesheets from for auto-generated pages?"
      else choice
    end
  
  rescue AbandonFeature
    config.delete "styles / page-styles"
  end

  end

  ## == Extra Features ==
  out
  out head: "Extras"
  wait

  out
  choice = select(
    "Any other features you’d like to enable?",
    options: {
      "assets"       => "copy assets from project root to `site/static/`",
      "SCSS"         => "collect global SCSS stylesheets into a SCSS config for SvelteKit",
      "Google Fonts" => "automatically build Google Fonts query",
    },
    multi: true,
  )

  should_prep_assets = (choice and choice.include? "assets")
  should_prep_scss   = (choice and choice.include? "SCSS")
  should_prep_fonts  = (choice and choice.include? "Google Fonts")

  if should_prep_assets then begin
    out
    choice = select(
      before: "Where should Squarkdown look for assets? #{GREY} Squarkdown will copy the contents of this directory to #{BLUE}static/#{GREY}",
      after:  "Where should Squarkdown look for assets?",
      options: {
        "/assets/"  => "",
        "/.assets/" => "",
        "other"     => "enter manually",
        "cancel"    => "",
      }
    )

    case choice
      when "cancel"    then raise AbandonFeature
      when "/assets/"  then config["assets / path"] = "assets/"
      when "/.assets/" then config["assets / path"] = ".assets/"
      when "other"
        config["assets / path"] = input(
          before: "Where should Squarkdown look for assets? #{GREY} Enter 1 directory",
          after:  "Where should Squarkdown look for assets?" 
        )
    end

    out
    choice = select(
      before: "Do you have a folder containing site-specific assets? #{GREY} These files will be copied directly to the root of #{BLUE}static/#{GREY}.",
      after:  "Do you have a folder containing site-specific assets?",
      options: {
        "no"                     => "default",
        "#{choice || '/'}site/"  => "",
        "#{choice || '/'}.site/" => "",
        "other"                  => "enter manually",
        "cancel"                 => "skip this feature",
      }
    )

    config["assets / site-assets"] = case choice
      when "cancel" then raise AbandonFeature
      when "other"  then input "Which folder contains site-specific assets?"
      when "no"     then nil
      else          choice
    end

    out
    choice = select(
      "What types of assets should Squarkdown copy?",
      options: {
        "png"    => true,
        "jpg"    => true,
        "jpeg"   => true,
        "svg"    => true,
        "webp"   => true,
        "other"  => "enter manually",
        "cancel" => "skip this feature",
      },
      multi: true,
    )

    config["assets / extensions"] = case choice
      when "cancel" then raise AbandonFeature
      when "other"
        exts = input "What types of assets should Squarkdown copy? #{GREY} Enter 1+ file extensions separated by spaces"
        exts.split.map { |ext| if ext.start_with "." then ext[1..] else ext end }
      else
        if choice.include? "cancel" then (raise AbandonFeature) else choice end
    end
  
  rescue AbandonFeature
    config.delete "assets / path"
    config.delete "assets / extensions"
  end

  end

  if should_prep_scss then begin
    out
    out head: "SCSS"
    wait

    out
    choice = select(
      before: "Where can Squarkdown find global stylesheets? #{GREY} Global stylesheets start with ~ in their filename",
      after: "Where can Squarkdown find global stylesheets?",
      options: {
        "src/styles"     => "",
        "src/lib/styles" => "",
        "other"          => "enter manually",
        "cancel"         => "",
      }
    )

    config["styles / path"] = case choice
      when "cancel" then raise AbandonFeature
      when "other" then input(
        before: "Where can Squarkdown find global stylesheets? #{GREY} Enter 1 directory",
        after:  "Where can Squarkdown find global stylesheets?"
      )
      else choice
    end

  rescue AbandonFeature
  end

  end

  if should_prep_fonts
    config["fonts / queries"] = ["Fira+Mono:wght@400;500;700"]
  end

  ## == Final Touches ==
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
      "warn   + create the directory" => "notify, but create the directory",
      "ignore + create the directory" => "silently create the directory",
      "warn   + skip directory"       => "notify and skip file",
      "ignore + skip directory"       => "silently skip file",
    }
  )

  config["opts / on-no-dir"] = case choice
    when "warn   + create the directory" then ["warn", "create"]
    when "ignore + create the directory" then ["ignore", "create"]
    when "warn   + skip directory"       then ["warn"]
    when "ignore + skip directory"       then ["ignore"]
  end

  ## == Finalise ==
  out ""
  out "Finalising..."

  config.compact!
  export_json(data: config)
  $created = true

  print PREV, CLEAR
  out success: "Your #{BLUE}squarkup.json#{WHITE} has been created in #{BLUE}#{cwd.parent}/.squarkdown/#{WHITE}."
  wait
  
  step(
    before: "For more help or guidance, please visit the docs at #{CYAN}https://sup2point0.github.io/stranger-quarkdown/docs#{WHITE}!",
    after:  "For more help or guidance, please visit the docs at https://sup2point0.github.io/stranger-quarkdown/docs!"
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


def kill()
  print CLEAR
  out
  out close: "#{CYAN}Thanks for using Squarkdown#{if $created then ', enjoy' else '' end}!#{WHITE} 🥕"
  puts
  exit
end



setup
script
kill
