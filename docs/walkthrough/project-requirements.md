# Project Requirements
<!-- #SQUARK live!
| dest = docs/walkthrough/project-requirements
| update = 2026 October 10
-->

Squarkdown is built for any project that has a [Svelte/Kit<sup>↗</sup>](https://svelte.dev) site using [MDsveX<sup>↗</sup>](https://mdsvex.pngwn.at).

It takes Markdown content scattered across your repo, and preprocesses it to be consumed by SvelteKit via [MDsveX](https://mdsvex.pngwn.io). For instance, it could be a CLI tool for which you’d like to have docs online as well as in the repo.[^cli-tool]

[^cli-tool]: Sound familiar? That’s what Squarkdown is, which is why it uses itself!

Those are the 2 prerequisites: SvelteKit and MDsveX.[^specific] How you use the Markdown beyond that is completely up to you!

[^specific]: Yeah, Squarkdown is quite a niche and stack-specific tool, lmao. Make sure you know what you’re using it for!


<br>


## Project Structure

Squarkdown expects a project structure like this:

```hs
/project/
   squarkup.toml
   ...
   site/
      vite.config.ts
      src/
         routes/
      	   ...
```

`squarkup.toml` contains your [squarkup configuration](../reference/squarkup-config.md). `site/` is a SvelteKit project.

Alternatively, you can place your squarkup config under a `.squarkdown/` folder:

```diff
  /project/
+    .squarkdown/
+       squarkup.toml
     ...
     site/
        src/
           vite.config.ts
        	...
           routes/
        	   ...
```

Squarkdown doesn’t care where your `.md` files are located – it will recursively scan for them (configurable via `paths.sources`, `paths.include`, `paths.exclude`).

You will, however, need to add `<!-- #SQUARK live! -->` (a [charm squark](../glossary.md#charm-squark)) to `.md` files you want Squarkdown to process.


<br>
