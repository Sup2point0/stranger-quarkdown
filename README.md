<div align="center">

![Stranger Quarkdown: A Successor to Quarkdown](assets/quarks-title.png)

[![Tests](https://github.com/Sup2point0/stranger-quarkdown/actions/workflows/test.yml/badge.svg)](https://github.com/Sup2point0/stranger-quarkdown/actions/workflows/test.yml)

</div>

---

**Stranger Quarkdown** (*StrangerQuarkdown*, *Squarkdown*) is a successor to [*Quarkdown*](https://github.com/Sup2point0/Quarkdown), for integration with [Svelte<sup>↗</sup>](https://svelte.dev), [SvelteKit](https://kit.svelte.dev) and [MDSveX<sup>↗</sup>](https://mdsvex.pngwn.io).

If you want to write content for a Svelte site in Markdown, but need finer flexibility in preprocessing and configuration, Squarkdown handles all that for you. It exports `.md` files to `.svx` files, while using [Quarkdown-Flavoured Markdown](https://github.com/Sup2point0/Quarkdown/blob/main/docs/quarks.md) for providing additional data and instructions.

> [!Note]
> Since this is a child project, there is both overlap and extension. Squarkdown-specific documentation can be found in [this repo’s docs](docs/). For the basics of Quarkdown, refer to the [Quarkdown repo](https://github.com/Sup2point0/Quarkdown).


<br>


## Features

### Core
- Automates several parts of the build process for SvelteKit projects
- Many flexible configuration 
- Multiple Rake tasks for flexibility
- Intelligently handles directories for more versatile navigation

### Extra
- Moves assets from a different folder to the SvelteKit `static/` directory.
- Identifies and collects fonts, building the [Google Fonts<sup>↗</sup>](https://fonts.google.com) `<link>` tag for you.

### Future
- Correct internal relative links to correct absolute links in production


<br>


## Usage

This project is designed to be used as a [Git submodule](https://git-scm.com/book/en/v2/Git-Tools-Submodules).

Add it to a repo:

```console
git submodule add https://github.com/Sup2point0/stranger-quarkdown
```

If it hasn’t already, clone the submodule:

```console
git submodule update --init
```

Make sure you’ve got your [repo config](docs/config.md) in `./.squarkdown/squarkup.json` if needed.

Run `rake` in the `./stranger-quarkdown/` directory to start the squarkup process. You can add this to your `npm run build` definition in `package.json`, so that it executes whenever you build the site:

```diff
  {
    "scripts": {
+     "build": "cd stranger-quarkdown && rake && cd .. && vite build"
    }
  }
```

Squarkdown will recursively look for all `.md` files starting from the root of the repo, and export them to `.svx` files in `src/routes`.

Many additional configurations are available to customise this process, although the defaults should cover most projects.

> [!Tip]
> For more walkthroughs, see the [Quarkdown docs](https://github.com/Sup2point0/Quarkdown/tree/main/docs). For finer details, also see the [Squarkdown docs](docs/).


<br>


## License

MIT. It’s not that even that good code, lmao.


<br>
