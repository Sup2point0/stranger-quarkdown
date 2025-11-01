# Making Full Use of Squarkdown
<!-- #SQUARK live!
| dest = docs/walkthrough/further-features
| desc = A further walkthrough of additional Squarkdown features
| capt = A Further Walkthrough of Additional Squarkdown Features
-->

Squarkdown has grown much beyond moving Markdown files around into a full-blown content deployment framework.[^framework] There are many other features that can help with automating the process of building up a website, which can be enabled at an instant.

[^framework]: Okay, ‘framework’ is a bit of a stretch.


<br>


## Fonts

Squarkdown can build a Google Fonts API request for you, so that you don’t have to manually tweak the exact request string.

Let’s say we’d like to use the Montserrat font from Google Fonts. On [Google Fonts<sup>↗</sup>](https://fonts.google.com) you’ll be given a request URL that looks like this:

```
https://fonts.googleapis.com/css2?family=Montserrat:ital,wght@0,100..900;1,100..900&display=swap
```

But most of that’s boilerplate! And in fact, when you use multiple fonts you can combine them into one request, without changing any of that boilerplate. The other fonts just get added in as more `family` parameters.

So, we only really care about the actual info about the font, which looks like `Montserrat:ital,wght@0,100..900;1,100..900`. We take this, and add it to the `fonts` option in `squarkup.json`:

```json
{
  ...

  "fonts": [
    "Montserrat:ital,wght@0,100..900;1,100..900"
  ]
}
```

Squarkdown will take the fragments from this `fonts` option, build them into a single request, and inject it into `app.html`. Naturally, Squarkdown looks in our `site` directory as configured in `squarkup.json` for `/src/app.html`.

To enable this feature, add the `fonts` option when running the task:

``` console
rake run squark[fonts]
```


<br>


## Assets

> [!Note]
> ‘Assets’ refers to images, videos, audio files, and other media.

SvelteKit wants assets in `static/`, but this isn’t always practical. Oftentimes, the site is just part of a larger project, so your asset files need to be shared across the entire project. They probably end up in their own folder at the project root, like so:

```
project/
   assets/
      collection/
         rickroll.mp4
         sus-thumbnail.png
      ...

   site/
      src/...
      static/...
      svelte.config.js
      ...

   README.md
```

Getting SvelteKit to look there for assets is gonna be more than a little fiddly (maybe ever impossible), so Squarkdown can save you the pain and just copy all asset files from `assets/` to `site/static/`.

> Now of course, this essentially duplicates the files locally, which can waste a lot of storage space (and assets tend to be the largest part of a repo too) – there’s not really a permanent solution to this, but if you're not working on the site, you can clear the `static/` folder, since the files will be regenerated next time anyway.

To enable this feature, add the `assets` option when running the task:

``` console
rake run squark[assets]
```

Squarkdown copies the file structure exactly, transferring all `.png`, `.jpg`, `.jpeg`, `.svg` files and ignoring those with other extensions. So after running Squarkdown, our repo will look like so:

```diff
  project/
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
  project/
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
  project/
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


## SCSS


<br>
