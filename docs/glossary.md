# Glossary of Squarkdown Terminology
<!-- #SQUARK live!
| dest = docs/glossary
| title = Glossary
| update = 2026 October 6
-->

Like any project, Squarkdown uses a sprinkle of jargon to refer to things with consistent names.

You don’t by any means need to understand all of these to use Squarkdown, but it can be useful to familiarise with them!


<br>


### Active
A `.md` file is *active* if it contains `#SQUARK live!` in its [charm squark](#charm-squark):

```md
# Page
<!-- #SQUARK live!
| dest = path/to/destination
-->

sup, world!
```

Active files are processed by Squarkdown and exported to `+page.svx` files.


<br>


### Charm Squark
> Main article: [Charm Squark](reference/charm-squark.md)

A special expanded `<!-- #SQUARK -->` [squark](#squark) at the front of your Markdown files that provides metadata to Squarkdown, which is exported to `+page.ts` and `site.json`.

```md
# Page
<!-- #SQUARK live!
| dest = path/to/destination
| capt = This is how Squarkdown works!
| update = 2026 October
-->

Why not YAML frontmatter? Because it’s rendered in Markdown previews!
```


<br>


### Field


<br>


### Inactive
A `.md` file is *inactive* either if it does not contain `#SQUARK live!` in its [charm squark](#charm-squark), or an explicit `#SQUARK dead!`:

```md
# Page
<!-- #SQUARK dead!
| dest = path/to/destination
-->

goodbye, cruel world...
```

Inactive files are not processed by Squarkdown.


<br>


### Squark
> Main article: [squarks](reference/squarks.md)

A special `<!-- #SQUARK -->` comment that tells Squarkdown to do something.

For instance, Squarkdown removes content inside `#SQUARK slash`:

```md
Happy days

<!-- #SQUARK slash? -->
This content won't appear in the output.
<!-- #SQUARK slash. -->

Hpapy days
```

The most important squark is the [charm squark](#charm-squark), a special extended squark at the start of `.md` files.


<br>


### Squarkdown
Short for *Stranger Quarkdown*, the name of this project!

`squarkdown` is the command to run Stranger Quarkdown.


<br>


### Squarkdown-flavoured Markdown
> Main article: [Squarkdown-flavoured Markdown](walkthrough/squarkdown-flavoured-markdown.md)

The special Markdown syntax Squarkdown accepts, using [squarks](#squark) hidden inside comments.


<br>


### Squarkup
The entire processing pipeline of Squarkdown:

- Finding your Markdown files
- Parsing metadata
- Rendering `+page.svx` and `+page.ts`
- Saving `site.json`
- (optionally) Copying assets
- (optionally) Injecting Google Fonts query into `app.html`


<br>


### Squarkup Config
> Main article: [Squarkup Config](reference/squarkup-config.md)

Your configuration for Squarkdown, sourced from either a `squarkup.toml` or `squarkup.json` file in your project repo.

All projects require a squarkup config to run Squarkdown. (While Squarkdown does have sensible defaults, it’s highly unlikely they’ll work out-of-the-box![^defaults])

[^defaults]: Unless you happen to have literally the same workflow as me...


<br>
