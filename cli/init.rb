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


def script
  config = load_default_repo_config

  puts

  ## == Intro == ##
  print GREY, " ", PRE_START, "  #{CYAN}Welcome to Squarkdown!  #{GREY}#{LINE * 42}"
  puts

  step(
    before: "You’re running version #{PINK}#{VERSION}",
    after:  "You’re running version #{VERSION}"
  )

  step(
    before: "This script will help you get your project’s #{BLUE}squarkup.json#{WHITE} set up!",
    after:  "This script will help you get your project’s squarkup.json set up.",
    newline: false,
  )

  step(
    "Disclaimer: This CLI is still a work in progress. Apologies if there are bugs!",
  )

  step(
    before: "Don’t worry if you’re unsure what a question means, just skip it. You can always manually edit #{BLUE}squarkup.json#{WHITE} afterwards.",
    after:  "Don’t worry if you’re unsure what a question means, just skip it. You can always manually edit squarkup.json afterwards."
  )

  step "Note no files will be created until the very end of the script."

  ## == Check Existing == ##
  cwd = Pathname(__dir__).parent
  existing = false
  cwd.ascend do |dir|
    if (dir / ".squarkdown/squarkup.json").exist?
      if dir.to_s.end_with?("stranger-quarkdown") then next end

      existing = existing || dir
    end
  end

  if existing
    out
    out error: "Woah, looks like you already have an existing #{BLUE}squarkup.json#{WHITE} in #{BLUE}#{existing}/.stranger-quarkdown/#{WHITE}!"
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

  ## == Configuring Paths == ##
  out
  out head: "Configuring Paths"
  wait

  # TODO site, dest
  # out
  # choice = select(
  #   before: "What directory folder contains your site?",
  #   after:  "What directory should Squarkdown output to?",
  #   options: {
  #     "site/" => "",
  #     ".site/" => "",
  #     "_site/" => "",
  #     "other" => "enter manually"
  #   }
  # )

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
    when "other"
      input "What directory should Squarkdown output to?"
    else
      choice
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
    when "project root + all subdirectories"
      [""]
    
    when "project root only"
      ["."]
    
    when "other"
      paths = input "Where should Squarkdown look for Markdown files to export? #{GREY} Provide directories separated by spaces."
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
    when "project root"                ["."]
    when "directories starting with ." [sep + "\..*?" + sep]
    when "directories starting with _" [sep + "_.*?" + sep]
    else                               []
  end

  ## == Exporting Files == ##
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
    when "other"
      input "Where should Squarkdown export the JSON file containing all site data?"
    else
      choice
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
    when "other"
      input "When Squarkdown exports .md files, what should it name the output .svx file?"
    else
      choice
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

  should_autogen = choice.include? "page."
  config_save = config
  
  ## == Autogeneration == ##
  while should_autogen
    out
    choice = select(
      "Squarkdown will need templates for these other auto-generated files. Where can it find them?",
      options: {
        "src/lib/bases/"   => "",
        "src/parts/bases/" => "",
        "other"            => "enter manually",
        "cancel"           => "skip this feature",
      }
    )

    config["bases / path"] = case choice
      when "cancel"
        break
      when "other"
        input "Where can Squarkdown find templates for auto-generated files?"
      else
        choice
    end

    out
    choice = select(
      "When Squarkdown applies page styles to these auto-generated pages, where should it get the stylesheets from?",
      options: {
        "src/styles/"      => "",
        "src/styles/bases" => "",
        "src/styles/pages" => "",
        "other"            => "enter manually",
        "cancel"           => "skip this feature",
      }
    )

    config["styles / path"] = case choice
      when "cancel"
        break
      when "other"
        input "Where should Squarkdown get stylesheets from for auto-generated pages?"
      else
        choice
    end

    should_autogen = false  # NOTE: To avoid infinite loop!
  end

  # NOTE: If we broke early `should_autogen` won't be reset to `false`, so we need to restore the save.
  if should_autogen
    config = config_save
  end

  ## == Extra Features == ##
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
  config_save = config

  while should_prep_assets
    out
    choice = select(
      before: "Where should Squarkdown look for assets? #{GREY} Squarkdown will copy the contents of this directory to #{BLUE}static/#{GREY}.",
      after:  "Where should Squarkdown look for assets?",
      options: {
        "./assets/"  => "",
        "./.assets/" => "",
        "other"      => "enter manually",
        "cancel"     => "",
      }
    )

    case choice
      when "cancel"
        break
      
      when "other"
        config["assets / path"] =
          input "Where should Squarkdown look for assets?"
      
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
      after:  "Do you have a folder containing site-specific assets?",
      options: {
        "no"                     => "default",
        "#{assets || '/'}site/"  => "",
        "#{assets || '/'}.site/" => "",
        "other"                  => "enter manually",
        "cancel"                 => "skip this feature",
      }
    )

    config["assets / site"] = case choice
      when "cancel"
        break
      when "other"
        input "Which folder contains site-specific assets?"
      when "no"
        nil
      else
        choice
    end

    out
    choice = select(
      "What types of assets should Squarkdown copy?",
      options: {
        "png"    => true,
        "jpg"    => true,
        "jpeg"   => true,
        "svg"    => true,
        "webp"   => "",
        "other"  => "enter manually",
        "cancel" => "skip this feature",
      },
      multi: true,
    )

    config["assets / extensions"] = case choice
      when "cancel"
        break
      when "other"
        exts = input "What types of assets should Squarkdown copy? #{GREY} Provide 1 or more file extensions (without leading `.`) separated by spaces."
        exts.map { |ext| "." + ext }
      else
        if choice.include? "cancel"
          break
        else
          choice
        end
    end

    should_prep_assets = false  # NOTE: To avoid infinite loop!
  end

  if should_prep_assets
    config = config_save
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
      "warn   + create the directory" => "notify, but create the directory",
      "ignore + create the directory" => "silently create the directory",
      "warn   + skip directory"       => "notify and skip file",
      "ignore + skip directory"       => "silently skip file",
    }
  )

  config["opts / on-no-dir"] = case choice
    when "warn   + create the directory"
      ["warn", "create"]
    
    when "ignore + create the directory"
      ["ignore", "create"]
    
    when "warn   + skip directory"
      ["warn"]
    
    when "ignore + skip directory"
      ["ignore"]
  end

  ## == Finalise == ##
  out ""
  out "Finalising..."

  config.compact!
  export_json(data: config)

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
