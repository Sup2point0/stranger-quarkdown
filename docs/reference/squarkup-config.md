# Squarkup Configuration
<!-- #SQUARK live!
| dest = docs/reference/squarkup-config
| update = 2026 October 10
-->

Squarkdown comes with sensible defaults out-of-the-box, but you’ll almost certainly want to configure it to suit your needs.

Squarkdown reads in your configuration from 1 of 4 places:

- [`/squarkup.toml`](squarkup-toml.md)
- `/squarkup.json`
- [`/.squarkdown/squarkup.toml`](squarkup-toml.md)
- `/.squarkdown/squarkup.json`

<!-- TODO -->
TOML is the recommended format. JSON support is in progress!

> [!Important]
> You need a squarkup config file – even if empty – to run Squarkdown, otherwise Squarkdown can’t find your project root!


<br>


## Categories

Configuration options are neatly organised into categories. Throughout the docs you’ll see them referred to as `category.option`, e.g. `format.preserve-comments`.

| Category | Description |
| -------- | ----------- |
| [`errors`](#errors)   | Error handling |
| [`project`](#project) | Project metadata |
| [`paths`](#paths)     | Where Squarkdown should find files |
| [`out`](#out)         | Output |
| [`format`](#format)   | Markdown rendering |
| [`assets`](#assets)   | Assets preprocessing |
| [`fonts`](#fonts)     | Fonts preprocessing |

> [!Tip]
> If you’re new to Squarkdown, [`errors`](#errors) and [`paths`](#paths) are the most important to start with!


<br>


## Errors

| Option | Type | Values | Default |
| ------ | ---- | ------ | ------- |
| [`strict`](#strict)                           | boolean | | `true` |
| [`debug`](#debug)                             | boolean | | `true` |
| [`on-error`](#on-error)                       | string | `warn` <br> `kill` | `warn` |
| [`file-already-exists`](#file-already-exists) | string | `overwrite` <br> `error` <br> `skip` | `overwrite` |
| [`link-broken`](#link-broken)                 | string | `mark-invalid` <br> `strip-extension` <br> `link-to-github` <br> `error` | `strip-extension` |

### `strict`
### `debug`
### `on-error`
### `file-already-exists`
### `link-broken`


<br>


## Project

| Option | Type |
| ------ | ---- |
| [`name`](#name)     | string |
| [`github`](#github) | string |

### `name`
### `github`


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


## Out

| Option | Type | Notes | Default |
| ------ | ---- | ----- | ------- |
| [`folder`](#folder)                 | string | path relative to project root | `src/routes/` in your site folder |
| [`site-data-path`](#site-data-path) | string <br> none | path relative to site | none |
| [`render-page-ts`](#render-page-ts) | boolean || `true` |
| [`shorter-fields`](#shorter-fields) | boolean || `false` |

### `folder`
### `site-data-path`
### `render-page-ts`
### `shorter-fields`


<br>


## Format

| Option | Type | Default |
| ------ | ---- | ------- |
| [`inject-head`](#inject-head)             | boolean | `true` |
| [`preserve-heading`](#preserve-heading)   | boolean | `false` |
| [`preserve-comments`](#preserve-comments) | boolean | `false` |
| [`externalise-links`](#externalise-links) | boolean | `true` |

### `inject-head`
### `preserve-heading`
### `preserve-comments`
### `externalise-links`


<br>


## Assets

| Option | Type | Notes | Default |
| ------ | ---- | ----- | ------- |
| [`folder`](#folder)                         | string | path relative to project root | your project root |
| [`site-assets-folder`](#site-assets-folder) | string <br> none | path relative to project root | none |
| [`extensions`](#extensions)                 | string array || `.png`, `.jpg`, `.jpeg`, `.webp`, `.svg` files |

### `folder`
### `site-assets-folder`
### `extensions`


<br>


## Fonts

| Option | Type | Values |
| ------ | ---- | ------ |
| [`queries`](#queries) | string array ||

### `queries`


<br>


| Field | Type | Values | Default | Description |
| :---- | :--- | :----- | :------ | :---------- |
| `repo` | string |  | `~` | Displayed name of the repository. This is injected into `<title>` in `<head>` of exported pages. |
| `paths / site` | string |  | `site/` | [ relative to root ]<br>Base directory of the site. Many other fields will provide paths relative to this directory. If your entire repo is the site, set this to `"."`. |
| `paths / sources` | `array` `null` |  |  | [ relative to root ]<br>Squarkdown recursively searches these directories for `.md` files to squarkup.<br>Use `"."` to target `.md` files in your project’s root.<br>Set this field to `null` to disable Markdown exporting entirely. |
| `paths / exclude` | `array` `null` |  |  | If a file’s path matches any of these RegEx patterns, squarkup will be skipped for it.<br>Note the RegEx is matched against the file’s absolute path. |
| `paths / dest` | string |  | `src/routes/` | [ relative to site ]<br>Markdown files are exported relative to this directory. |
| `out / file-name` | string |  | `~content` | Name of exported `.svx` files (without the `.svx` file extension). |
| `out / site-data` | string |  | `src/site.json` | File where Squarkdown exports the site data. Relative to site. |
| `asses / path` | string |  |  | [ relative to root ]<br>Squarkdown looks here for static assets to preprocess. |
| `assets / site-assets` | string `null` |  |  | [ relative to root ]<br>Squarkdown moves assets in this directory straight to the root of `site`/`static`. |
| `assets / extensions` | `string[]` |  | `["jpg", "jpeg", "png", "svg", "webp"]` | Only files with these extensions will be preprocessed by Squarkdown. |
| `fonts / queries` | `string[]` |  |  | Individual URL query params for requesting fonts from Google Fonts. |


<br>
