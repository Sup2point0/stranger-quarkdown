# Making Full Use of Squarkdown
<!-- #SQUARK live!
| dest = docs/walkthrough/further-features
| desc = Further walkthrough for additional Squarkdown features
| capt = More Squarkdown Features
| update = 2026 October 19
-->

> [!Warning]
> This page is outdated as of Squarkdown v4.0, which rewrites Squarkdown in Rust. Bear with me while I bring it up to date!

Beyond just Markdown, Squarkdown can also help with other preprocessing. It’s all opt-in, so you don’t need to use it if it’s not relevant to you =)


<br>


## Site Data

> Main article: [Site Data](../reference/site-data.md)

After parsing every active file’s charm squark, Squarkdown has collected *quite a lot* of data about your site that you may want to keep. Squarkdown exports that data to a JSON file for you to use however you see fit.


<br>


## Fonts

```bash
> squarkdown --fonts
```

Squarkdown can build a Google Fonts API request for you, so that you don’t have to manually tweak the exact request string.

Let’s say we’d like to use the Montserrat font from Google Fonts. On [Google Fonts<sup>↗</sup>](https://fonts.google.com) you’ll be given a request URL that looks like this:

```
https://fonts.googleapis.com/css2?family=Montserrat:ital,wght@0,100..900;1,100..900&display=swap
```

But most of that’s boilerplate! And in fact, when you use multiple fonts you can combine them into one request, without changing any of that boilerplate. The other fonts just get added in as more `family` parameters.

So, we only really care about the actual info about the font, which looks like `Montserrat:ital,wght@0,100..900;1,100..900`. Take that, and add it to `fonts.queries`:

```toml
[fonts]
queries = [
  "Montserrat:ital,wght@0,100..900;1,100..900",
]
```

Squarkdown will take the fragments, build them into a single request, and inject it into `app.html`.


<br>


## Assets

```bash
> squarkdown --assets
```

> [!Note]
> “Assets” refers to images, videos, audio files, and other media.

SvelteKit wants assets in `static/`, but this isn’t always practical.

Oftentimes, the site is just part of a larger project, so your asset files need to be shared across the entire project. Maybe they’re alongside the `docs/`, or they’re in their own folder at the project root:

```py
your-project/
   README.md

   assets/
      collection/
         rickroll.mp4
         sus-thumbnail.png
      ...

   site/
      src/
      static/
```

Getting SvelteKit to look there for assets is gonna be more than a little fiddly (maybe even impossible), so Squarkdown can save you the pain and just copy all asset files from `assets/` to `site/static/`.

> Now of course, this duplicates the files locally, which can waste a lot of storage space (and assets tend to be the largest part of a repo too). Hopefully, for larger projects you’re using a dedicated CDN for media.

> [!Tip]
> If you store all your assets in `assets/` and gitignore `static/`, then you can safely delete it when you’re not working on a project to save on storage space – Squarkdown will regenerate all the assets next time you need it anyway!

Squarkdown copies the file structure exactly. So after running Squarkdown, our repo will look like so:

```diff
  your-project/
     assets/
        collection/
           rickroll.mp4
           sus-thumbnail.png
        ...

     site/
        static/
+          collection/
+             sus-thumbnail.png
+          ...
```

But what if we had assets we’re only using on the website? We’d probably store them under their own folder `site/` in `assets/`:

```diff
  your-project/
     assets/
        collection/
           rickroll.mp4
           sus-thumbnail.png
+       site/
+          favicon.png
        ...
```

Before this would’ve gone in `static/` and we could’ve accessed it directly as `favicon.png`. But now we need to prefix it with `site/` to `site/favicon.png`. Just a tad annoying, but very relevant – so why not solve it.

Just provide a directory to the `site-assets` option in `squarkup.json`, and Squarkdown will move all assets/folders in that directory straight to the root of `static/`. So if we set `site-assets` to `assets/site`, after running Squarkdown we would have:

```diff
  your-project/
     assets/
        collection/
           rickroll.mp4
           sus-thumbnail.png
        site/
           favicon.png
        ...

     site/
        static/
           collection/
              sus-thumbnail.png
+          favicon.png
           ...
```


<br>
