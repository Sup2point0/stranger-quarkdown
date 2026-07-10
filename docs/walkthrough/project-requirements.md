# Project Requirements
<!-- #SQUARK live!
| dest = docs/walkthrough/project-requirements
| update = 2026 July 10
-->

Squarkdown is built for any project that has a [Svelte](https://svelte.dev)/[Kit](https://svelte.dev/docs/kit) website.

It takes Markdown content scattered across your repo, and preprocesses it to be consumed by SvelteKit via [MDsveX](https://mdsvex.pngwn.io).

Those are the 2 prerequisites: SvelteKit and MDsveX.[^specific] How you use the Markdown beyond that is completely up to you!

[^specific]: Yeah, Squarkdown is a very niche and stack-specific tool, lmao. Make sure you know what you’re using it for!

Squarkdown doesn’t care where your files are located – it will recursively scan for them, with rules configurable to suit your needs. You will, however, need to add `<!-- #SUQARK live! -->` headers to `.md` files you wish for Squarkdown to process.

For instance, it could be a CLI tool for which you’d like to have docs online.[^cli-tool] Squarkdown would help you extract the docs from the repo to the built site.

[^cli-tool]: Sound familiar? That’s what Squarkdown is, which is why it uses itself!
