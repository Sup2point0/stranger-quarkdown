# Squarkdown-flavoured Markdown
<!-- #SQUARK live!
| dest = docs/walkthrough/squarkdown-flavoured-markdown
-->

The magic of Squarkdown lies in ***squarks***, which let you provide instructions for how Squarkdown should process a Markdown file. ***Squarkdown-flavoured Markdown*** is what this syntax is called.


<br>


## Overview

Squarks are simply Markdown/HTML comments that follow a specific format. They look like this:

```md
<!-- #SQUARK instruction? -->
```

You can use these anywhere in your file to customise how Squarkdown should process the Markdown when exporting it. Of course, because they’re just comments, they **won’t show up** when previewing the Markdown – so you can hide all the instructions for Squarkdown in the raw text, without worrying about it disrupting the Markdown preview in any way![^disrupt-preview]

[^disrupt-preview]: [YAML Frontmatter](https://docs.github.com/en/contributing/writing-for-github-docs/using-yaml-frontmatter) unfortunately leaves a fully visible table of the values in GitHub’s Markdown preview. Maybe this’ll change in future, but until then, this is what Squarkdown avoids!

```md
# What is Squarkdown-flavoured Markdown?
&lt;!-- #SQUARK live! feat!
| dest = tests/sq-fl-mk
| capt = It’s Markdown with wacky comments.
| tags = tests / demos / no-deploy
| date = 2025 March 14
--&gt;

&lt;!-- #SQUARK leave? --&gt;
Squarks won’t change your Markdown preview in any way.
&lt;!-- #SQUARK leave. --&gt;

&lt;!-- #SQUARK only?
But they can influence how it’s processed when rendered to HTML!
     #SQUARK only. --&gt;
```


<br>


## Squarks

### Syntax
Squarks begin with a capitalised `#SQUARK` to clearly differentiate them from regular comments. After this comes a short 1-word ***directive*** which provides Squarkdown with a particular instruction.

> [!Tip]
> Spaces in `<!-- #SQUARK [squark] -->` are recommended for tidiness, but are not required for Squarkdown to handle them properly.

### Flavours
The behaviour of a squark is indicated by the punctuation mark that follows its directive. For instance, in `<!-- #SQUARK live! -->` it’s a `!` (exclamation mark).

| flavour       | mark | example | description | notes |
| :------------ | :--- | :------ | :---------- | :---- |
| flag          | `!`  | `#SQUARK live!` |
| section open  | `?`  | `#SQUARK leave?` |
| section close | `.`  | `#SQUARK leave.` |
| anchor        | `~`  | `#SQUARK index~` |


<br>


## Section Squarks

```md
<!-- #SQUARK leave? -->
This text will be ignored by Squarkdown.
<!-- #SQUARK leave. -->
```

This type of squark comes in pairs, with one to open the section and one to close it.


<br>


## Squark Charm

> Main article: [Squark Charm](../ref/squark-charm.md)

### Active

### Metadata
