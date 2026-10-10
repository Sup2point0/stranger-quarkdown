# Using Squarkdown in a SvelteKit project
<!-- #SQUARK live!
| destination = docs/walkthrough/quickstart
| capt = A guided walkthrough on how to setup and use Squarkdown
| update = 2026 October 6
-->

Welcome to Squarkdown!

This is a zero-to-one guided walkthrough on how to set up and use Squarkdown in your SvelteKit project.

We’ll get Squarkdown installed, configured and running, then look at some common things you might want to do with Squarkdown.

This guide will stick to recommended options to keep it simple, but there are many places where multiple options are available!

> [!Tip]
> It may be helpful to have the [Glossary](../glossary.md) open while reading.


<br>


## Introduction

> [!Important]
> Squarkdown was made for integration with [SvelteKit<sup>↗</sup>](https://svelte.dev/docs/kit) + [MDsveX<sup>↗</sup>](https://mdsvex.pngwn.at) projects, and nothing else![^nothing-else]

[^nothing-else]: Maybe you can get it work with other frameworks, who knows :P but SvelteKit + MDsveX is the intended target.

So we’re on the same page, suppose our project repo looks like this:

```
project/
   README.md
   
   docs/
      showerthoughts.md
   
   site/
      vite.config.ts
      ...
      src/
         routes/
            ...
```

We’ve got some Markdown files under `docs/` that we’d like to quickly and easily turn into a static website under `site/`. That’s what Squarkdown will do for us.


<br>


## Install Squarkdown

Let’s install Squarkdown from NPM:

```bash
/project/> cd site
/project/site> npm install stranger-quarkdown
```

We can check we have access to Squarkdown with:

```bash
squarkdown --version
```

> [!Note]
> Pre-4.0, Squarkdown (written in Ruby) required installation as a Git submodule. Now it’s properly published to the [crates.io<sup>↗</sup>](https://crates.io/crates/squarkdown) and [NPM<sup>↗</sup>](https://www.npmjs.com/package/stranger-quarkdown) registries ;)

> [!Tip]
> Try running `squarkdown` now. What happens?


<br>


## Configure Squarkdown

We’ll configure Squarkdown in a `squarkup.toml` at the root of our project:

```diff
  project/
     README.md
+    squarkup.toml
     
     docs/
        showerthoughts.md
     
     site/
        vite.config.ts
        ...
        src/
           routes/
              ...
```

For now, all we need is to specify that our site lives under `/site/`:

```toml
[paths]
site = "site/"
```

> See [Squarkup Configuration](../reference/squarkup-config.md) for full details on all the options that can go in `squarkup.toml`.

> [!Tip]
> Try running `squarkdown` now. What happens?


<br>


## Setup Files

Let’s take `showerthoughts.md`:

```md
# Showerthoughts

Popsicle and ice lolly are lollipop and icicle swapped around...
```

### Activate the file
By default, Squarkdown won’t do anything with Markdown files. We need to explicitly mark them as [***active***](../glossary.md#active) to tell Squarkdown it should process them.

We do this by adding a special comment to the top of these files, under the heading:

```diff
  # Showerthoughts
+ <!-- #SQUARK live! -->

  Popsicle and ice lolly are lollipop and icicle swapped around...
```

This is a [***squark***](../glossary.md#squark). Since it’s a comment, it won’t show up when previewing the Markdown, but it *is* kept in the raw text for Squarkdown to process.

All squarks start with `#SQUARK` so Squarkdown knows they’re a special comment.

Here, `live!` is a **flag** telling Squarkdown *“Hey, this file is active!”* Only files with this flag will be processed and exported.

### Configure the destination
But, where should Squarkdown export this file to?

We need to provide this metadata through a [***field***](../glossary.md#field). We add fields below `live!`, forming an expanded squark called the [***charm squark***](../glossary.md#charm-squark).

Here, we provide `destination`:

```diff
# Showerthoughts
  <!-- #SQUARK live!
+ | destination = showerthoughts
  -->
  
  Popsicle and ice lolly are lollipop and icicle swapped around...
```

This means `showerthoughts.md` will be exported to `/site/src/routes/showerthoughts/+page.svx` (as well as an adjacent `+page.ts`).


<br>


## Squarkup!

We’re now ready to run Squarkdown!

```bash
/project/site> squarkdown
```

You should see output like this:

```bash
Squarkdown v4.0.0
────────────────────────
 ✓ found your project: <project>
 › resolving config...
 ✓ found your squarkup config: <project>/squarkup.toml
 › reading config...
 › read successful!
 › validating config...
 ✓ config looks good, all set!
 ✓ found your site: <project>/site
 › finding files to squarkup...

...
```

That means Squarkdown’s done its magic. We should find freshly generated `+page.svx` and `+page.ts` files:

```diff
  project/
     README.md
     squarkup.toml
     
     docs/
        showerthoughts.md
     
     site/
        vite.config.ts
        ...
        src/
           routes/
+             showerthoughts/
+                +page.svx
+                +page.ts
```

Have a click into `+page.svx` and `+page.ts` to see what Squarkdown generates.

And that’s it, you’re good to go!


<br>


## Git Ignore

You probably don’t want all of Squarkdown’s autogenerated files polluting Git.

Ignoring Squarkdown output is a little tricky, though: You’ll probably have other `+page.ts` under `routes/`, so ignoring `routes/**/+page.ts` doesn’t work.

<!-- TODO explain (layout) -->
The solution I use is to create a dedicated `(layout)/` folder:

```diff
  project/
     README.md
     squarkup.toml
     
     docs/
        showerthoughts.md
     
     site/
        vite.config.ts
        ...
        src/
           routes/
+             (squarkdown)/
                 showerthoughts/
                    +page.svx
                    +page.ts
```

Now in our `.gitignore` we can add:

```diff
+ site/src/routes/(squarkdown)/**/+page.svx
+ site/src/routes/(squarkdown)/**/+page.ts
```


<br>


## More Fields
`destination` is just one of many metadata fields we can provide.

We can provide a short `description` for the page:

```diff
# Showerthoughts
  <!-- #SQUARK live!
  | destination = showerthoughts
+ | description = Just some of our showerthoughts
  -->
```

We can provide a list of `tags`, separated by `/`:

```diff
# Showerthoughts
  <!-- #SQUARK live!
  | destination = showerthoughts
  | description = Just some of our showerthoughts
+ | tags = docs / examples / writing
  -->
```

We can set the `release-date` and `last-update` in the format `<year> <month> <date>`:

```diff
# Showerthoughts
  <!-- #SQUARK live!
  | destination = showerthoughts
  | description = Just some of our showerthoughts
  | tags = docs / examples / writing
+ | release-date = 2022 February 2
+ | last-update = 2022 April 1
  -->
```

Or if we want a less precise date, we can omit the date, or even month:

```diff
# Showerthoughts
  <!-- #SQUARK live!
  | destination = showerthoughts
  | description = Just some of our showerthoughts
  | tags = docs / examples / writing
- | release-date = 2022 February 2
+ | release-date = 2022 February
- | last-update = 2022 April 1
+ | last-update = 2022
  -->
```

Squarkdown also supports seasons:

```diff
# Showerthoughts
  <!-- #SQUARK live!
  | destination = showerthoughts
  | description = Just some of our showerthoughts
  | tags = docs / examples / writing
  | release-date = 2022 February
- | last-update = 2022
+ | last-update = 2022 spring
  -->
```


<br>


## Next Steps

Here’s some things you might want to try next:

- Modify your [Squarkup Config](../reference/squarkup-config.md) to suit your needs
- Use [Squarkdown-flavoured Markdown](squarkdown-flavoured-markdown.md) to customise how Squarkdown processes and renders your Markdown
- Use `squarkdown --assets` to handle assets and asset links


<br>
