# Cleanup
<!-- #SQUARK live!
| dest = squarkup/cleanup
-->

Sometimes there’s a couple things in a Markdown file that need to be cleaned up when it’s processed by Squarkdown. We can let Squarkdown know it needs to handle these through the `clean` field in the [squark charm](file-config.md).


<br>


## Braces

Curly braces `{}` pose an issue since Svelte and MDSveX use them for interpolation. This means when we try to import the `.svx` file, errors are raised as Svelte attempts to process the text inside as JavaScript.

To solve this, Squarkdown replaces all occurrences of `{` with `&amp;&lbrace;` (and likewise for `}`). The double escape is necessary since `&lbrace;` alone would still be rendered to `{` by MDSveX.


<br>


## HTML Tags

Similarly, Svelte will try to process `<` and `>` as HTML tags, even if they aren’t used as such.


<br>


## Line Breaks

This is something perhaps more specific to me, but Squarkdown can also remove extraneous line breaks if you have any. This replaces all instances of `\n\n<br>\n\n` in the source text with just `\n\n`.

I often use this to separate 2nd-level headings so that they render with a more distinct gap in Markdown, but in HTML this gap can be added with CSS so the extra line break is unnecessary.
