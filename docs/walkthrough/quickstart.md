# Using Squarkdown in a SvelteKit project
<!-- #SQUARK live!
| dest = docs/walkthrough/quickstart
| capt = A guided walkthrough on how to setup and use Squarkdown
| update 
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
npx squarkdown --version
```

> [!Note]
> Pre-4.0, Squarkdown (written in Ruby) required installation as a Git submodule. Now it’s properly published to the [crates.io<sup>↗</sup>](https://crates.io/crates/squarkdown) and [NPM<sup>↗</sup>](https://www.npmjs.com/package/stranger-quarkdown) registries ;)


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

See [Squarkup Configuration](squarkup-config.md) for full details on all the things that can go in `squarkup.toml`.

For now, all we need is to specify that our site lives under `/site/`:

```toml
[paths]
site = "site/"
```


<br>


## Squarkdown-Flavoured Markdown

Let’s add Squarkdown-Flavoured Markdown to `docs/showerthoughts.md` so it can be squarked up.

```md
# Showerthoughts

Popsicle and ice lolly are lollipop and icicle swapped around...

```

### Activate the file
First, we need to mark the file as active. At the start, add a comment starting with `#SQUARK` followed by `live!`:

```md
# Showerthoughts
<!-- #SQUARK live! -->
```

This is a **squark**. It won’t show up when previewing the Markdown, but is kept in the raw text for Squarkdown to process. All squarks start with `#SQUARK` so Squarkdown knows they’re a special comment.

Here, `live!` is a **flag** telling Squarkdown *“Hey, this file is active!”* Only files with this flag will be processed and exported.

### Configure the destination
But, where to? We need to provide this metadata through a **field**. These go in the first squark where `live!` is, forming an expanded squark block called the **squark charm**.

Let’s export our file to `./site/src/routes/showerthoughts/content.svx`. Remember in `./.squarkdown/squarkup.json` we’ve already configured our site directory (`./site/`), destination directory (`/src/routes/`), and file name (`content.svx`). So, all we need is `showerthoughts`, and Squarkdown will handle the rest:

```md
# Showerthoughts
<!-- #SQUARK live!
| dest = showerthoughts
-->
```

### Provide other metadata
There’s plenty of other metadata we can provide.

To configure the title and description that go in the `head` of the page, set the `title` and `desc` fields:

```md
# Showerthoughts
<!-- #SQUARK live!
| dest = showerthoughts
| title = Our Showerthoughts
| desc = Just some of our showerthoughts
-->
```

To use a particular stylesheet(s), set `style`:

```md
# Showerthoughts
<!-- #SQUARK live!
| dest = showerthoughts
| style = cute
-->
```

To use multiple stylesheets, separate each one with ` / `:

```md
# Showerthoughts
<!-- #SQUARK live!
| dest = showerthoughts
| style = cute / special
-->
```

If you’d like to set a preferred light/dark theme, set `duality`:

```md
# Showerthoughts
<!-- #SQUARK live!
| dest = showerthoughts
| duality = dark
-->
```

For a release/publish date, the format is `<year> <month> <date>`:

```md
# Showerthoughts
<!-- #SQUARK live!
| dest = showerthoughts
| date = 2022 February 2
-->
```

We can omit the date and month if desired, and can even supply a season instead of a month:

```md
# Showerthoughts
<!-- #SQUARK live!
| dest = showerthoughts
| date = 1984 winter
-->
```


<br>


## Squarkup!

Alright, we’re now set to squarkup our file. Squarkdown provides tasks through a `Rakefile` which you can invoke if you have `rake` installed. Here, all we’ll need to do is:

```
rake squarkup
```

Then let the magic happen as Squarkdown does its stuff!

```
>>> squarkdown / squarking up...
               / ...
               / done!
```

If nothing’s gone wrong, we now have:

```diff
  ./
     .squarkdown/...
     docs/...
     site/
        src/
           routes/
+             showerthoughts/
+                +page.svelte
+                content.svx
        ...
     stranger-quarkdown/...
     README.md
```


<br>
