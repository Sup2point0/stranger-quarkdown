# Using Squarkdown in a SvelteKit project
<!-- #SQUARK live!
| dest = walkthrough
| capt = A guided walkthrough on how to use Squarkdown in a SvelteKit project
-->

Here’s a guided walkthrough on how to use Squarkdown in a SvelteKit project.

> [!Tip]
> It may be helpful to have the [Glossary](glossary.md) open while reading.


<br>


## Introduction

> [!Important]
> Squarkdown was made for integration with SvelteKit projects, and nothing else!

Let’s say we’ve got some Markdown files in a repository we’d like to quickly and easily turn into a static website – perhaps you’re writing a blog or have documentation for software. And since we love nice things, you we Svelte/Kit for building the website.

Awesome, but now you need to find a way to import the content from our repository to the SvelteKit project. These Markdown files could be scattered all throughout the repository, so it’s not exactly something a 1-liner could solve.


<br>


## Setup

So we’re on the same page, suppose our project repo looks like this:

```
./
   docs/
      showerthoughts.md
   README.md
```

### Add Squarkdown
First, add Squarkdown to your project repo as a Git submodule:

```console
git submodule add https://github.com/Sup2point0/stranger-quarkdown
```

This should clone the Squarkdown repo into a `stranger-quarkdown/` folder. If you’d like to give the folder a different name (`.stranger-quarkdown` is good for distinguishing it from the actual project files), just add it after the command:

```console
git submodule add https://github.com/Sup2point0/stranger-quarkdown .stranger-quarkdown
```

If it hasn’t already, clone the submodule:

```console
git submodule update --init
```

Your project should now look like this:

```diff
  ./
     docs/...
+    stranger-quarkdown/
+       Rakefile
+       ...
     README.md
```

### Configure Squarkdown
Next, add a `.squarkdown/` folder in the root of your project. This is where configuration files for Squarkdown will go, just like `.github/` or `.vscode/`.

You’ll then need to add a `squarkup.json` file inside. You can either do this manually, or let Squarkdown generate a template for you:

```
cd stranger-quarkdown
rake setup squarkup
```

We now have:

```diff
  ./
+    .squarkdown/
+       squarkup.json
     docs/...
     stranger-quarkdown/...
     README.md
```

For details of what to put in `squarkup.json`, see [Configuring Squarkup for a Repo](repo-config.md).

### Setup Site
If you haven’t already, create your SvelteKit project. You can do this manually, use `npm create svelte@latest`, or let Squarkdown do it for you:

```
rake setup site
```

This will create the site files in the project root, but you’ll probably want them in a folder instead. In that case, just pass it in as an argument:

```
rake setup site .site
```

Let’s go with the latter. Our repo is now ready for Squarkdown:

```diff
  ./
+    .site/
+       src/
+          routes/
+             ...
+       svelte.config.js
+       ...
     .squarkdown/...
     docs/...
     stranger-quarkdown/...
     README.md
```

> [!Note]
> Squarkdown uses [Sup2point0/svelte-core](https://github.com/Sup2point0/svelte-core) as the skeleton. This is a cleaned up version of the SvelteKit skeleton configured for static sites, with TypeScript, SCSS, MDSveX added.


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
