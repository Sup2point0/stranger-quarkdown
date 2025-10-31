# Squark Charm
<!-- #SQUARK live!
| dest = docs/reference/squark-charm
-->

<div class="quicklinks" align="center">

[Example](#example) <span class="separator">•</span> [Cheatsheet](#cheatsheet)

</div>

The squark charm is an extended squark which should go under the `# h1` title of a page. It provides all the metadata and instructions for how Squarkdown should squarkup the page.


<br>


## Flags

| Flag | Description | Notes |
| :--- | :---------- | :---- |
| `live!` | Active | |
| `dead!` | Inactive | |
| `index!` | Index | |
| `depr!` | Deprecated | |
| `feat!` | Featured | |

Anything else that follows the format `<flag>!` and appears in this first line will be parsed as a flag and also saved in `FileData.flags`.


<br>


## Fields

For fields which accept multiple values, these values should be separated with ` / `.

| Option | Parameters | Values | Description | Default | Notes |
| :----- | :--------- | :----- | :---------- | :------ | :---- |
| **`dest`** | `path` `path[]` | any filepath | Where the file will be exported to. | | Relative to site routes (`.../site/src/routes/`) |
| `title` | `string` | any | Title injected into `<title>` of the exported HTML. | `head` | Different to `head` |
| `desc` | `string` | any | Description injected into `<meta name="description">` of the exported HTML. | `capt` if provided. | Different to `capt` |
| `head` | `string` | any | Displayed `<h1>` text in the page header. | First detected `# ` text in the Markdown file. | Different to `title` |
| `capt` | `string` | any | Short caption text displayed below the header. | | A description of what the page is (such as “Yu-Gi-Oh! Archetype”) rather than a unique concrete description – different to `desc`. |
| `style` | `filename[]` | any `.scss` file | Stylesheets to apply. | Base stylesheet. | Should be a list of file names without file extensions. |
| `duality` | `option` | `light` `dark` <br> `light!` `dark!` | Default colour theme to use if user has no preference. | `light` | User preference can be ignored by following it with a `!`. |
| `index` | `string[]` | any | Where to index the page. | | |
| `date` | `year` `month/season?` `day>` | `<season>`: `spring` `summer` `autumn` `winter` | Creation or publish date of the page. | | |
| `update` | `year` `month/season?` `day>` | `<season>`: `spring` `summer` `autumn` `winter` | Last date on which the page was changed. | | Set manually, not automatically. |
| `clean` | `option` `option[]` | `angles` `braces` `comments` `line-breaks` | Aspects of the text to cleanup. | | See [Cleanup](cleanup.md) for more. |


<br>


## Example

Here’s what a Markdown file with a full squark charm would look like:

```md
# Example: Never Gonna Give You Up
<!-- #SQUARK live! feat!
| dest = rick/roll
| title = Happy April Fools
| desc = Definitely not a rickroll
| head = Never Gonna Give You Up
| capt = Rick Astley
| style = rickroll / joke
| duality = dark
| index = soundtracks
| shard = #INDEX / jokes
| date = 1984 April 1
| clean = line-breaks / comments
-->
```


<br>


## Cheatsheet

Here’s a quick cheatsheet to remind yourself of all the flags and fields:

```md
<!-- #SQUARK live! <flag> ...
| dest = <destination-directory>
| title = <webpage-title>
| desc = <webpage-description>
| head = <page-header>
| capt = <page-caption>
| style = <stylesheet> / <stylesheet> ...
| duality = {light(!) / dark(!)}
| index = <index> / <index> ...
| shard = <shard> / <shard> / ...
| date = <year> <month/season?> <date?>
| clean = ... / ...
-->
```
