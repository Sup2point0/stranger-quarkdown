# What is Squarkdown?
<!-- #SQUARK live!
| dest = docs/walkthrough/what-is-squarkdown
| desc = An introduction to Stranger Quarkdown
-->

**StrangerQuarkdown**, or *Squarkdown* `/ˌskwɑːkdaʊn/` for short, is a lightweight but feature-rich preprocessing tool for building a Svelte website with Markdown content.

> No, it’s not yet another JavaScript framework, nor is it a documentation generator like [MkDocs](https://www.mkdocs.org/) or CDN like [Wordpress](https://wordpress.com/).

You can write content for a website in [Squarkdown-flavoured Markdown](squarkdown-flavoured-markdown.md), then run Squarkdown to preprocess and collect that content for your website to import and use however you see fit. It means you can put the content anywhere, and use Squarkdown to collect

The best way to understand what Squarkdown is for, is probably to just follow the story of why I created it.

Say you’ve got a project with `.md` (Markdown) files scattered all throughout it. You want to build a website for the project, using the content in these Markdown files. But there’s no easy way you to access the content in them, and control how they’ll be handled and rendered.

Squarkdown does that all for you. You tell it where to find `.md` files, and it’ll search your repo for them and export them to `.svx` files in your site’s directory. You can provide metadata and other preprocessing directives through Squarkdown-flavoured Markdown

Squarkdown is built for integration with [Svelte], [SvelteKit] and [MDsveX]. If it isn’t clear already, it is a pretty niche tool! Exactly what sorts of projects Squarkdown can be used for are explored in [Project Requirements].
