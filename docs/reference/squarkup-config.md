# Squarkup Configuration
<!-- #SQUARK live!
| dest = docs/reference/squarkup-config
| update = 2026 November 4
-->

Squarkdown comes with sensible defaults out-of-the-box, but you’ll almost certainly want to configure it to suit your needs.

Squarkdown reads in your configuration from 1 of 4 places:

- [/squarkup.toml](squarkup-toml.md)
- /squarkup.json (not yet unsupported)
- [/.squarkdown/squarkup.toml](squarkup-toml.md)
- /.squarkdown/squarkup.json (not yet unsupported)

<!-- TODO -->
TOML is the recommended format. JSON support is in progress!

> [!Important]
> You need a squarkup config file – even if empty – to run Squarkdown, because Squarkdown uses it to resolve your project root!


<br>


## Categories

Configuration options are neatly organised into categories. Throughout the docs you’ll see them referred to as `category.option`, e.g. `format.preserve-comments`.

| Category | Description |
| -------- | ----------- |
| [`errors`](#errors)   | Error handling |
| [`project`](#project) | Project metadata |
| [`paths`](#paths)     | Finding files to squarkup |
| [`charm`](#charm)     | Charm squark |
| [`out`](#out)         | Output |
| [`format`](#format)   | Markdown rendering |
| [`assets`](#assets)   | Assets preprocessing |
| [`fonts`](#fonts)     | Fonts preprocessing |

> [!Tip]
> If you’re new to Squarkdown, [`paths.site`](#site) is *the* most important option to set.
>
> From there, [`errors`](#errors) and [`paths`](#paths) are the most important categories to start with.


<br>


## Errors

| Option | Type | Values | Default |
| ------ | ---- | ------ | ------- |
| [`strict`](#strict)                           | boolean | | `true` |
| [`debug`](#debug)                             | boolean | | `true` |
| [`on-error`](#on-error)                       | string | `warn` <br> `kill` | `warn` |
| [`file-already-exists`](#file-already-exists) | string | `overwrite` <br> `error` <br> `skip` | `overwrite` |
| [`link-broken`](#link-broken)                 | string | `mark-dead` <br> `strip-extension` <br> `link-to-github` <br> `error` | `error` |
| [`link-inactive`](#link-inactive)             | string | `mark-dead` <br> `strip-extension` <br> `link-to-github` <br> `error` | `error` |

### `strict`
Enable stricter safety checks?

This includes:

- Requiring `dest`(`ination`) to be explicitly provided in the [charm squark](charm-squark.md)
- Validating charm squark only contains [Squarkdown-native fields]
- Checking directories remain under your project root
- Checking multiple files don't export to the same directory

### `debug`

Enable more helpful debug output?

This includes:

- When a rendering error occurs, showing a snapshot of the source text pointing out the exact error

This requires Squarkdown to do more work, so has a tiny impact on performance. It’s enabled by default since the performance impact is usually negligible, and it makes error messages significantly better!

*New in v4.2*.

### `on-error`
How should Squarkdown react to non-fatal errors?

Defaults to `warn`, meaning Squarkdown will report the error but continue processing. This means one bad page won’t bring down the entire squarkup, and you’d be able to catch more errors in a single run.

In production, you’ll probably want `kill`, so that you don’t get an incomplete build.

### `file-already-exists`
How should Squarkdown react when a file to render to already exists?

Defaults to `overwrite`, meaning Squarkdown will overwrite the existing file.

In production, you may want `error` to avoid Squarkdown overwriting a handwritten `+page.svx` without you knowing.

### `link-broken`
> [!Important]
> Not to be confused with [`link-inactive`](#link-inactive)!

How should Squarkdown handle a link that does not resolve to an existent file or folder?

Defaults to `error`, meaning Squarkdown will raise an error handled according to [`errors.on-error`](#on-error).

*New in v4.6*.

### `link-inactive`
> [!Important]
> Not to be confused with [`link-broken`](#link-broken)!

How should Squarkdown handle a link that does not resolve to an [active](../glossary.md#active) file or folder?

This refers to when a link *would* be broken in production, because the linked file doesn’t have `#SQUARK live!`, and so wouldn’t have a page in the site.

Defaults to `error`, meaning Squarkdown will raise an error handled according to [`errors.on-error`](#on-error).


<br>


## Project

| Option | Type |
| ------ | ---- |
| [`name`](#name)     | string |
| [`github`](#github) | string |

### `name`
The displayed name of the repo, injected into `<title>` when `format.inject-head = true`.

*New in v4.2*.

### `github`
The GitHub repository name, e.g. `Sup2point0/stranger-quarkdown`.

*New in v4.2*.


<br>


## Paths

| Option | Type | Default |
| ------ | ---- | ------- |
| [`site`](#site)       | string | your project root |
| [`sources`](#sources) | string array | your entire project repo |
| [`include`](#include) | string array | `.md` files |
| [`exclude`](#exclude) | string array | `.git/`, `.node_modules/`, `.svelte-kit/` folders |

### `site`
### `sources`
### `include`
### `exclude`


<br>


## Charm

| Option | Type | Default |
| ------ | ---- | ------- |
| [`flags`](#flags)   | string array | none |
| [`fields`](#fields) | string array | none |

### `flags`

*New in v4.8*.

### `fields`

*New in v4.8*.


<br>


## Out

| Option | Type | Notes | Default |
| ------ | ---- | ----- | ------- |
| [`folder`](#folder)                 | string | path relative to project root | `src/routes/` in your site folder |
| [`file-name`](#file-name)           | string | file name with `.svx` extension | `+page.svx` |
| [`site-data-path`](#site-data-path) | string <br> none | path relative to site | none |
| [`render-page-ts`](#render-page-ts) | boolean || `true` |
| [`shorter-fields`](#shorter-fields) | boolean || `false` |

### `folder`
Where to export files relative to.

`destination` paths in files are relative to this folder.

### `file-name`
The file name for exported files, including the (expected) `.svx` extension.

### `site-data-path`
Where to export site data, including the (expected) `.json` extension.

If absent, site data is not exported.

### `render-page-ts`
Should `+page.ts` files be exported?

### `shorter-fields`
Should fields in exported `+page.ts` and `site.json` use more compact identifiers?

For instance, `description` is shortened to `desc`, and `last_update` is shortened to `update`.


<br>


## Format

| Option | Type | Default |
| ------ | ---- | ------- |
| [`inject-head`](#inject-head)             | boolean | `true` |
| [`preserve-heading`](#preserve-heading)   | boolean | `false` |
| [`preserve-comments`](#preserve-comments) | boolean | `false` |
| [`externalise-links`](#externalise-links) | boolean | `true` |
| [`escape-angles`](#escape-angles)         | boolean | `true` |
| [`escape-braces`](#escape-braces)         | boolean | `true` |
| [`repair-tables`](#repair-tables)         | boolean | `true` |

### `inject-head`
Inject a `<head>` element containing `<title>` and `<meta name="description">`?

*New in v4.2*.

### `preserve-heading`
Keep the first `# Heading` in the rendered output?

### `preserve-comments`
Keep `<!-- comments -->` in the rendered output?

### `externalise-links`
Convert links containing `<sup>↗</sup>` to `<a target="_blank">` elements?

### `escape-angles`
Escape `<>` (that don’t form HTML tags) as `&lt;` `&gt;` for HTML safty?

*New in v4.9*.

### `escape-braces`
Escape `{}` as `&lbrace;` `&rbrace;` for Svelte safety?

*New in v4.9*.

### `repair-tables`
> See also: [Repairing HTML](../features/repair.md)

Repair potentially broken HTML that would result in SvelteKit rendering errors?

This includes:

- Adding missing `<tbody>` to `<table>` elements

*New in v4.8*.


<br>


## Assets

These options are used for assets preprocessing and rewriting links to asset files.

If any options are provided, then assets preprocessing will be enabled and asset links will be rewritten.

| Option | Type | Notes | Default |
| ------ | ---- | ----- | ------- |
| [`base-path`](#base-path)                   | string | path relative to site origin | none |
| [`folder`](#folder)                         | string | path relative to project root | your project root |
| [`site-assets-folder`](#site-assets-folder) | string <br> none | path relative to project root | none |
| [`extensions`](#extensions)                 | string array | file extensions | `.png`, `.jpg`, `.jpeg`, `.webp`, `.svg` files |

### `base-path`
Base path to prepend to rewritten asset links.

When assets preprocessing is enabled, Squarkdown will rewrite Markdown links to asset files, so that they work in the site. These links are absolute links, so e.g. `![](../image.png)` becomes `![](/image.png)`.

This is fine if your site is served from the origin root (e.g. `my-site.com/image.png`), but if you’re under a subpath (e.g. `me.github.io/project/image.png`), then these links will break.

This option remedies that by allowing you to specify what to prepend before `/image.png`. So if you provide `my-project`, then the link is rewritten to `/my-project/image.png`.

*New in v4.8*.

### `folder`
The folder containing asset files to copy to SvelteKit’s `static/` folder.

### `site-assets-folder`
The folder containing asset files to copy directly to the root of `static/`, without structure preservation.

### `extensions`
Only files with these extensions will be treated as asset files.


<br>


## Fonts

| Option | Type | Values |
| ------ | ---- | ------ |
| [`queries`](#queries) | string array ||

### `queries`


<br>
