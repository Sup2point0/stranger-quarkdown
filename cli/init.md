squark/cli/init.rb
------------------------------------------

<!-- > Welcome to Squarkdown! You’re currently running version [#{}].
  any key to continue
> This script will help you get your project’s [squarkup.json] set up!
> Don’t worry if you’re unsure what a question means, just skip it You can always manually edit [squarkup.json] afterwards. Note no files will be created until the very end of the script. -->

---
> Woah, looks like you already have an existing [.squarkdown/] directory!
> Continue running this script?
  All changes will overwrite any existing files.
---
> Just a heads up, Squarkdown is for [SvelteKit] projects that use [MDsveX].
  If this isn’t the case, Squarkdown probably isn’t the tool you’re looking for!
---


------------------------------------------

# Configuring Paths

> What’s your project’s [root] directory?
  This will be called ROOT.
_

<!-- > What directory should Squarkdown output to?
  This will be called SITE. It’s usually the directory where your site lives.
~ site/
~ .site/
~ _site/
~ other enter manually -->

<!-- > Where should Squarkdown look for Markdown files to export?
  You can choose directories to exclude in the next question.
~ project root (+ all subdirectories) default
~ project root (only)
~ specific directories (configure manually) -->

<!-- > Which directories should Squarkdown ignore?
~ none
~ project root
~ directories starting with .
~ directories starting with _ -->


------------------------------------------

# Exporting Files

<!-- > Where should Squarkdown export the JSON file containing all site data, and what should it call the file?
~ [/src/site.json] default
~ [/src/site-data.json]
~ [/src/data/site.json] -->

<!-- > When Squarkdown exports [.md] files, what should it name the output [.svx] file?
~ ~content.svx default
~ _content.svx
~ content.svx
~ other enter manually -->

<!-- > Should Squarkdown also auto-generate a [+page.svelte] or [+page.js] for each exported [.md] file?
~ [+page.svelte], [+page.js]
~ [+page.svelte] only
~ [+page.js] only
~ vary by page configure manually
~ no -->

---
<!-- > Squarkdown will need templates for these other auto-generated files. Where can it find them?
  Provide a path relative to Site.
~ [src/lib/bases/]
~ [src/parts/bases/]
~ other enter manually
~ cancel -->
---

<!-- > When Squarkdown applies page styles, where should it get the stylesheets from?
~ [SITE/src/styles/]
~ [SITE/src/styles/bases]
~ [SITE/src/styles/pages]
~ other enter manually
~ skip -->


------------------------------------------

<!-- # More Features

> Any other features you’d like to enable?
= assets preprocessing
= SCSS preprocessing
= Google Fonts preprocessing
~ continue -->


------------------------------------------

# Preprocessing Assets

<!-- > Where should Squarkdown look for assets?
  Squarkdown will copy the contents of this directory to [Site/static/].
~ [./assets]
~ [./.assets]
~ other enter manually
~ cancel -->

<!-- > Do you have a folder containing site-specific assets?
  These files will be copied directly to the root of [Site/static/], instead of being kept in that folder.
~ [/site/]
~ [/.site/]
~ other enter manually
~ cancel -->

> What types of assets should Squarkdown copy?
|= .png
|= .jpg / .jpeg
|= .svg
= .webp
= other enter manually
~ continue
~ cancel


------------------------------------------

# Preprocessing SCSS

> 


------------------------------------------

<!-- # Final Touches

> How should Squarkdown handle errors when processing files?
~ kill stop execution
~ warn log and skip file

> What should Squarkdown do when an output directory doesn’t exist?
~ warn + create the directory default
~ warn + do nothing -->


------------------------------------------

<!-- >>> Finalising...

> Your [squarkup.json] has been created in [./.squarkdown/].
> If you selected options that needed to be filled manually, you can find them indicated with placeholder comments.
> For more help or guidance, please visit the docs on GitHub at https://github.com/Sup2point0/stranger-quarkdown
> Thanks for using Squarkdown, enjoy! 🥕 -->
