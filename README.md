<div align="center">

![Stranger Quarkdown: A Successor to Quarkdown](.github/squark-cover.png)

[![crates.io](https://img.shields.io/crates/v/squarkdown?color=fb8717&style=flat-square)](https://crates.io/crates/squarkdown)
[![npm](https://img.shields.io/npm/v/stranger-quarkdown?color=fb8717&style=flat-square)](https://www.npmjs.com/package/stranger-quarkdown)
[![tests](https://img.shields.io/github/actions/workflow/status/Sup2point0/stranger-quarkdown/test.yaml?label=tests&style=flat-square)](https://github.com/Sup2point0/stranger-quarkdown/actions/workflows/test.yaml)
[![site](https://img.shields.io/github/actions/workflow/status/Sup2point0/stranger-quarkdown/site.yaml?label=site&style=flat-square)](https://github.com/Sup2point0/stranger-quarkdown/actions/workflows/site.yaml)

</div>

---

<div align="center">

[Docs](docs/)&ensp;·&ensp;[Quickstart](docs/walkthrough/quickstart.md)&ensp;·&ensp;[FAQ](FAQ.md)&ensp;·&ensp;[Changelog](CHANGELOG.md)&ensp;·&ensp;[Site](https://sup2point0.github.io/stranger-quarkdown/docs)

</div>

> [!Warning]
> Squarkdown has been freshly rewritten in Rust, which does mean it has a much more extensive test suite, but also means it’ll probably have some rough edges!

**Stranger Quarkdown** (*Squarkdown*) is a build tool for [SvelteKit<sup>↗</sup>](https://svelte.dev/docs/kit/introduction) and [MDsveX<sup>↗</sup>](https://mdsvex.pngwn.io) projects.

Write Markdown content anywhere in your project repo, with [special syntax](docs/walkthrough/squarkdown-flavoured-markdown.md 'Squarkdown-Flavoured Markdown') hidden inside comments, then use Squarkdown to mass-export them into `+page.svx` and `+page.ts` files in your SvelteKit project.

```md
# Welcome to Squarkdown!
<!-- #SQUARK live! feat!
| dest = walkthrough/welcome
| capt = An introduction to what Squarkdown can do
| date = 2024 July 1
-->

This link to [other-file.md](other-file.md) will have its `.md` extension stripped.

<!-- #SQUARK slash? -->
This content won’t be included in the output.
<!-- #SQUARK slash. -->

<!-- #SQUARK only?

This content only shows up in the output.

     #SQUARK only. -->
```


<br>


## Features

Squarkdown lets you:

- Write Markdown anywhere in your repo, independently of your SvelteKit site
- Use [Squarkdown-Flavoured Markdown](docs/walkthrough/squarkdown-flavoured-markdown.md) to control how Squarkdown renders the content – all hidden inside `<!-- #SQUARK -->` comments, meaning no disruption to the original Markdown preview!
- Attach metadata to pages that is passed to SvelteKit via `+page.ts` and/or `site.json`
- Link between Markdown files and to asset files without worry; Squarkdown fixes the links for the site for you

> In my experience, it’s quite difficult to explain to people what Squarkdown is exactly. But it’s a tool I absolutely need – and use in so many projects! – so trust me, it *is* useful ;)


<br>


## Quickstart

### Cargo (cross-platform)
```bash
# Install:
> cargo install squarkdown

# Setup (once per project):
# > squarkdown init
# (under development)

# Run:
> squarkdown

# With extras:
> squarkdown --assets
```

### npm (Windows/Linux)
```bash
# Install:
> npm install stranger-quarkdown

# Setup (once per project):
# > npx squarkdown init
# (under development)

# Run:
> npx squarkdown

# With extras:
> npx squarkdown --assets
```

> [!Tip]
> Use `squarkdown --help` to remind yourself of the above.

For more on how Squarkdown works, how to configure and customise Squarkdown, and the features available, read [the docs](docs/) in this repo, or view them on [the website](https://sup2point0.github.io/stranger-quarkdown/docs)!


<br>


## Generative AI

<a href="https://brainmade.org">
  <img align="right" height="40" src=".github/brainmade-black.svg" />
</a>

All lovingly handcrafted <3


<br>
