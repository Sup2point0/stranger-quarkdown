<div align="center">

![Stranger Quarkdown: A Successor to Quarkdown](assets/quarks-title.png)

[![Tests](https://github.com/Sup2point0/stranger-quarkdown/actions/workflows/test.yml/badge.svg)](https://github.com/Sup2point0/stranger-quarkdown/actions/workflows/test.yml)

</div>

---

**Stranger Quarkdown** (*StrangerQuarkdown*, *Squarkdown*) is a successor to [*Quarkdown*](https://github.com/Sup2point0/Quarkdown), for integration with [Svelte](https://svelte.dev), [SvelteKit](https://kit.svelte.dev) and [MDSveX](https://mdsvex.pngwn.io/).

If you want to write content for a Svelte site in Markdown, but need finer processing or configuration, Squarkdown handles all that for you. It exports `.md` files to `.svx` files, while using [Quarkdown-Flavoured Markdown](https://github.com/Sup2point0/Quarkdown/blob/main/docs/quarks.md) for providing additional data and instructions.

Since this is a child project of Quarkdown, there is a lot of overlap, but also some extension. Squarkdown-specific documentation can be found in [this repo’s docs](docs/) – for the basics of Quarkdown, refer to the [Quarkdown repo](https://github.com/Sup2point0/Quarkdown).


<br>


## Features

 - Fully automated build process
 - Bridge seamlessly between Markdown and MDSveX
 - Extra directives for style and metadata injection
 - Versatile handling of directory navigation


<br>


## Usage

This project is designed to be used as a [Git submodule](https://git-scm.com/book/en/v2/Git-Tools-Submodules).

Add it to a repo:

```console
git submodule add https://github.com/Sup2point0/stranger-quarkdown
```

Clone the submodule:

```console
git submodule update --init
```

Make sure you’ve got your [repo config](docs/config.md) in `./.squarkdown/squarkup.json` if needed.

Run `./stranger-quarkdown/squarkup.rb` to start the squarkup process. You can add this to your `npm run build` definition in `package.json`, so that it executes whenever you build the site:

```diff
  {
    "scripts": {
+     "build": "ruby stranger-quarkdown/squarkup && vite build"
    }
  }
```

Squarkdown will recursively look for all `.md` files starting from the root of the repo, and export them to `content.svx` files in `src/routes`.

Many additional configurations are available to customise this process, although the defaults should cover most projects.

> [!Tip]
> For more walkthroughs, see the [Quarkdown docs](https://github.com/Sup2point0/Quarkdown/tree/main/docs). For finer details, also see the [Squarkdown docs](docs/).


<br>


## License

MIT. It’s not that even that good code, lmao.


<br>
