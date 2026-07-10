# Squarkdown-flavoured Markdown
<!-- #SQUARK live!
| dest = docs/walkthrough/squarkdown-flavoured-markdown
| update = 2026 July 10
-->

The magic of Squarkdown lies in ***squarks***. Which are just fancy comments hidden under your Markdown, that provide specific instructions for how Squarkdown should process that Markdown.

This syntax is called ***Squarkdown-flavoured Markdown***!


<br>


## Overview

Squarks are Markdown/HTML comments starting with `#SQUARK`. They look like this:

```md
<!-- #SQUARK instruction? -->
I’m a little teapot (not part of the squark)
<!-- #SQUARK instruction. -->
```

You can use these anywhere in your file to customise how Squarkdown should process the Markdown when exporting it. Of course, because they’re just comments, **they won’t show up** when previewing the Markdown – so you can hide all the instructions for Squarkdown in the raw text, without worrying about it disrupting the Markdown preview in any way![^disrupt-preview]

[^disrupt-preview]: [YAML Frontmatter](https://docs.github.com/en/contributing/writing-for-github-docs/using-yaml-frontmatter) unfortunately leaves a fully visible table of the values in GitHub’s Markdown preview. Maybe this’ll change in future, but until then, this is what Squarkdown avoids!

```md
# What is Squarkdown-flavoured Markdown?
<!-- #SQUARK live! feat!
| dest = tests/sq-fl-mk
| capt = It’s Markdown with wacky comments.
| tags = tests / demos / no-deploy
| date = 2025 March 14
-->

<!-- NOTE: There shouldn't be spaces between the `#` and the rest of the squark, but having a real squark in the demo code interferes with *actual* Squarkdown LMAO -->

<!-- # SQUARK leave? -->
Squarks won’t change your Markdown preview in any way.
<!-- # SQUARK leave. -->

<!-- # SQUARK only?
But they can influence how it’s processed when rendered to HTML!
     # SQUARK only. -->

```


<br>


## Squarks

### Syntax
Squarks begin with a capitalised `#SQUARK` to clearly differentiate them from regular comments. There must be **no** space between the `#` and `SQUARK`.

After this comes a short 1-word ***directive*** which provides Squarkdown with a particular instruction.

> [!Tip]
> The spaces present in `<!-- #SQUARK [squark] -->` are recommended for tidiness, but are not technically required for Squarkdown to handle them properly.

### Flavours
The behaviour of a squark is indicated by the punctuation mark that follows its directive. For instance, in `<!-- #SQUARK live! -->` it’s a `!` (exclamation mark).

| flavour       | mark | example | description | notes |
| :------------ | :--- | :------ | :---------- | :---- |
| flag          | `!`  | `#SQUARK live!` | tells Squarkdown a fact about the file |
| section open  | `?`  | `#SQUARK leave?` | opens a section of specialised processing |
| section close | `.`  | `#SQUARK leave.` | closes a section of specialised processing |
| anchor        | `~`  | `#SQUARK index~` | a target for Squarkdown to find and replace with something |


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
