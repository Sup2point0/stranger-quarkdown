# Configuring Squarkup for a File
<!-- #SQUARK dead!
| dest = docs/walkthrough/file-config
| desc = Flags and fields for how a Markdown file is processed by Squarkdown
-->

<div class="quicklinks" align="center">

[Example](#example) <span class="separator">•</span> [Cheatsheet](#cheatsheet)

</div>

> *See also: [Configuring Squarkup for a Repo](repo-config.md)*

When Squarkdown processes files, it will only export them if they indicate they are **active** and provide necessary metadata. This takes the form of an expanded squark known as the **squark charm**, which looks like this:

```mdion
<!-- #SQUARK live!
| dest = path/to/destination
| capt = This is a squark charm!
| ...
-->
```


<br>


## Overview

> [!Tip]
> Place the squark charm at the start of the text, below the title if you prefer. The sooner Squarkdown can find it, the more time saved – across a large repo, it adds up!

The squark charm is broken over multiple lines. After `#SQUARK` comes a series of [Flags](#Flags). These tell Squarkdown, *Hey, keep this in mind!*

Below are the [Fields](#Fields), neatly arranged with pipe signs and equals, which can be set to desired values. Squarkdown processes and stores these internally for processing and rendering.

So the overall format is:

```md
<!-- #SQUARK live! <flag>
| <field> = <value>
-->
```

Precise details of the available flags and fields are covered below.


<br>


## Flags

| Flag | Description | Notes |
| :--- | :---------- | :---- |
| `live!` | File is active. | If this flag is not detected within a few lines, the file is assumed to be inactive and processing is skipped – this saves a lot of otherwise wasted time!
| `dead!` | File is inactive. | Little difference to omitting core entirely, but explicitly indicates to developers “we’re not squarking this up, for a reason”. |
| `index!` | Index file for listing other pages, treated differently during rendering. | See [Index Files](index-files.md) for more. |

Anything else that follows the format `<identifier>!` and appears in this first line will be parsed as a flag and saved in `FileData.flags`.


<br>


## Fields

For fields which accept multiple values, these should be separated with ` / `.

| Option | Parameters | Values | Description | Default | Notes |
| :----- | :--------- | :----- | :---------- | :------ | :---- |
| **`dest`** | `<path>` | any | Where the file will be exported to. | | Relative to site routes (`<path/to/site>/src/routes`) |
| `title` | `<title>` | any | Title injected into `<head>` of the exported HTML within `<title>`. | `head`. | Different to `head`. |
| `desc` | `<description>` | any | Description injected into `<head>` of the exported HTML within `<meta name="description">` | `capt` if provided. | Different to `capt`. |
| `head` | `<head>` | any | Displayed `<h1>` text in the page header. | First detected `# ` text in the Markdown file. | Different to `title`. |
| `capt` | `<caption>` | any | Short caption text displayed below the header. | | A description of what the page is (such as “Yu-Gi-Oh! Archetype”) rather than a unique concrete description – different to `desc`. |
| `style` | `<style(s)>` | stylesheets | Stylesheets to apply. | Base stylesheet. | Should be a list of file names without file extensions. |
| `duality` | `<duality>` | `light` `dark` <br> `light!` `dark!` | Default colour theme to use if user has no preference. | `light` | User preference can be ignored by following it with a `!`. |
| `index` | `<index(s)>` | any | Where to index the page. | | |
| `date` | `<year> / <month/season?> / <day?>` | `<season>`: `spring` `summer` `autumn` `winter` | Creation or publish date of the page. | | Used as a sort parameter when searching. |
| `clean` | `<clean-aspect(s)>` | `line-breaks` `comments` `braces` `angles` | Aspects of the text to cleanup. | | See [Cleanup](cleanup.md) for more. |


<br>


## Example

Here’s what a Markdown file with a full squark charm would look like:

```md
# Never Gonna Give You Up
<!-- #SQUARK live!
| dest = rick/roll
| desc = Definitely not a rickroll
| capt = Rick Astley
| style = rickroll
| duality = dark
| index = songs
| shard = #INDEX / jokes
| date = 1984 April 1
| clean = line-breaks / comments
-->
```


<br>


## Cheatsheet

And for reference, here’s a quick cheatsheet for all the flags and fields:

```md
<!-- #SQUARK live! feat! ...
| dest = <destination-directory>
| title = <webpage-title>
| desc = <meta-description>
| head = <header-text>
| capt = <caption-text>
| style = <stylesheet> / <stylesheet> ...
| duality = {light(!) / dark(!)}
| index = <index> / <index> ...
| shard = <shard> / <shard> / ...
| date = <year> <month/season?> <date?>
| clean = ... / ...
-->
```
