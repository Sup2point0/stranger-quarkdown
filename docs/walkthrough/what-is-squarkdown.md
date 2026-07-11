# What is Squarkdown?
<!-- #SQUARK live!
| dest = docs/walkthrough/what-is-squarkdown
| desc = An introduction to Stranger Quarkdown
| update = 2026 July 10
-->

**StrangerQuarkdown**, or *Squarkdown* `/ˌskwɑːkdaʊn/` for short, is a preprocessing tool for using Markdown content in a SvelteKit website.

> No, it’s not yet another JavaScript framework, nor is it a documentation generator like [MkDocs](https://www.mkdocs.org) or CDN like [Wordpress](https://wordpress.com).

Squarkdown allows you to:

- Write Markdown content anywhere in your repo
- Use [Squarkdown-flavoured Markdown](squarkdown-flavoured-markdown.md) to provide extra metadata and instructions
- Use Squarkdown to copy that content into your website

To illustrate, say you’ve got a project with `.md` (Markdown) files scattered all throughout it. You want to build a website for the project, using the content in these Markdown files. But there’s no easy way you to access the content in them, and control how they’ll be handled and rendered.

Squarkdown does that all for you. You tell it where to find `.md` files, and it’ll search your repo for them and export them to `.svx` files in your site’s directory.

Squarkdown is built for integration with [Svelte](https://svelte.dev), [SvelteKit](https://svelte.dev/docs/kit) and [MDsveX](https://mdsvex.pngwn.io). If it isn’t clear already, it is a pretty niche tool! Exactly what sorts of projects Squarkdown can be used for are explored in [Project Requirements](project-requirements.md).
