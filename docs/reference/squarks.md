# Squarks
<!-- #SQUARK live!
| dest = docs/reference/squarks
| update = 2025 November 21
-->

<br>


## Singleton Squarks

***Singleton squarks*** appear as a standalone squark, indicating some kind of flag or providing data.

Currently, the only existing singleton squark is the critical ***squark charm***, but this may change in future.

### Squark Charm
> *See* [Squark Charm](squark-charm.md).


<br>


## Twin Squarks

***Twin squarks*** always appear in pairs, with a `<!-- #SQUARK squark? -->` to open the block and `<!-- #SQUARK squark. -->` to close it.

### `only`
```md
<!-- # SQUARK only?

This text will only appear in the rendered output.

     # SQUARK only. -->
```

Text wrapped in `only` squarks isn’t displayed in Markdown (since it’s commented out), but will be present in the final rendered output. (During squarkup, the squarks are removed, and the text inside is processed.)

### `leave`
Text wrapped in `leave` squarks isn’t processed by Squarkdown. It will still appear in the rendered output, it just won’t have any processing applied to it.

### `slash`
Text wrapped in `slash` squarks is removed from the rendered output.

You can use this for content specific to the Markdown version of your document.

> [!Tip]
> You can create a Markdown-version and web-version of some text by using both the `slash` and `only` squarks – wrap the Markdown-only content in `slash`, and the web-only content in `only`.


<br>


## Anchor Squarks

***Anchor squarks*** are special kinds of singleton squarks that provide an ‘anchor’ indicating where Squarkdown should inject content.
