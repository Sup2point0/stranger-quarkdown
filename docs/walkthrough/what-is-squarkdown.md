# What is Squarkdown?
<!-- #SQUARK live!
| dest = docs/walkthrough/what-is-squarkdown
| desc = An introduction to Stranger Quarkdown
| update = 2026 July 10
-->

**Stranger Quarkdown**, or *Squarkdown* (`/ˌskwɑːkdaʊn/`) for short, is a Markdown build tool for [SvelteKit<sup>↗</sup>](https://svelte.dev/docs/kit) projects using [MDsveX](https://mdsvex.pngwn.at).

> No, it’s not yet another JavaScript framework, nor is it a documentation generator like [MkDocs<sup>↗</sup>](https://www.mkdocs.org) or CDN like [Wordpress<sup>↗</sup>](https://wordpress.com).

At its core, it takes Markdown files scattered across your repo, and preprocesses them to `+page.svx` and `+page.ts` files in your SvelteKit site:

```diff
  /project/
     docs/
        README.md
        help/
           quickstart.md
  
     site/
        src/
           routes/
+             +page.svx
+             +page.ts
  
+             quickstart/
+                +page.svx
+                +page.ts
```

Squarkdown also allows you to use [Squarkdown-flavoured Markdown](squarkdown-flavoured-markdown.md) hidden inside comments to control how the Markdown is rendered.

For instance, if you have some text that you want in your repo, but *not* in the site, you can tell Squarkdown to remove it using `#SQUARK slash` (a [***squark***](../glossary.md#squark)):

```md
Never gonna give you up

<!-- #SQUARK slash? -->
Hi, GitHub!
<!-- #SQUARK slash. -->

Never gonna let you down
```
