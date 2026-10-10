# Squarks
<!-- #SQUARK live!
| dest = docs/reference/squarks
| desc = All the squark types in Squarkdown-flavoured Markdown
| update = 2026 October 10
-->

This page documents the squark types in [Squarkdown-flavoured Markdown](../walkthrough/squarkdown-flavoured-markdown.md).


<br>


## Cheatsheet

Here’s a quick reminder of all the squark types:

```md
# Charm Squark
<!-- #SQUARK live! feat!
| dest = charm-squark
-->

<!-- #SQUARK slash? -->
Strip this content from the output
<!-- #SQUARK slash. -->

<!-- #SQUARK only?

Inject this content into the output

     #SQUARK only. -->

<!-- #SQUARK leave? -->
Don't do anything to this
<!-- #SQUARK leave. -->
```


<br>


## Charm Squark

> Main article: [Charm Squark](charm-squark.md)

The ***charm squark*** is a special expanded squark at the start of files:

```md
# Charm Squark
<!-- #SQUARK live!
| destination = charm-squark
| description = This is what a charm squark looks like
| last-update = 2026 October
-->
```

The charm squark provides metadata for the page that Squarkdown renders to `+page.ts` and `site.json`.

All [***active***](../glossary.md#active) files that you want Squarkdown to squarkup must have a charm squark with the `live!` flag.


<br>


## Twin Squarks

***Twin squarks*** always appear in pairs, with a `<!-- #SQUARK squark? -->` to open the block and `<!-- #SQUARK squark. -->` to close it.

### `slash`
```md
<!-- #SQUARK slash? -->
This text won't appear in the rendered output.
<!-- #SQUARK slash. -->
```

Text wrapped in `slash` squarks is removed from the rendered output.

You might need this if:

- You want to keep some content exclusively in the Markdown version of your file

### `only`
```md
<!-- # SQUARK only?

This text will only appear in the rendered output.

     # SQUARK only. -->
```

Text wrapped in `only` squarks isn’t displayed in Markdown (since it’s commented out), but will be present in the final rendered output. During squarkup, the squarks are removed, and the text inside is processed as regular Markdown.

You might need this if:

- You want to show some content exclusively in the site, but not in your Markdown file
- You want to use Svelte or MDsveX functionality that would break in regular Markdown

> [!Tip]
> You can create a Markdown-version and web-version of some text by using the `slash` and `only` squarks together:
> 
> ```md
> <!-- #SQUARK slash? -->
> Hello, Markdown!
> <!-- #SQUARK slash. -->
> 
> <!-- #SQUARK only?
> 
> Hello, Svelte!
> 
>      #SQUARK only. -->
> ```

### `leave`
```md
<!-- #SQUARK leave? -->
Don't rewrite [this link](broken-link.md)!
<!-- #SQUARK leave. -->
```

Text wrapped in `leave` squarks isn’t processed by Squarkdown. It will still appear in the rendered output, it just won’t have any of Squarkdown’s processing applied to it.

You might need this if:

- Squarkdown is running into issues with a particular part of a file
- You have a temporarily broken link you want to ignore
