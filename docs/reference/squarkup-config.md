# Squarkup Configuration
<!-- #SQUARK live!
| dest = docs/reference/squarkup-config
| update = 2026 October 6
-->

> [!Warning]
> This page is outdated as of Squarkdown v4.0, which rewrites Squarkdown in Rust. Bear with me while I bring it up to date!

You can (and should) configure Squarkdown to suit your needs.

Squarkdown provides many configuration options to customise its behaviour, neatly sorted into categories. You’ll see these referred to as `category.option` throughout the docs.

Squarkdown reads in your configuration from 1 of 4 places:

- `/squarkup.toml`
- `/squarkup.json`
- `/.squarkdown/squarkup.toml`
- `/.squarkdown/squarkup.json`

<!-- TODO -->
TOML is the recommended format. JSON support is in progress!


<br>


## Fields

> [!Note]
> This table is auto-generated from the exact specification as laid out by the [JSON schema](https://sup2point0.github.io/stranger-quarkdown/squarkup-schema/latest.json).

<!-- #SQUARK leave? -->

<!-- #SQUARK inject? -->
| Field | Type | Values | Default | Description |
| :---- | :--- | :----- | :------ | :---------- |
| `repo` | `string` |  | `~` | Displayed name of the repository. This is injected into `<title>` in `<head>` of exported pages. |
| `paths / site` | `string` |  | `site/` | [ relative to root ]<br>Base directory of the site. Many other fields will provide paths relative to this directory. If your entire repo is the site, set this to `"."`. |
| `paths / sources` | `array` `null` |  |  | [ relative to root ]<br>Squarkdown recursively searches these directories for `.md` files to squarkup.<br>Use `"."` to target `.md` files in your project’s root.<br>Set this field to `null` to disable Markdown exporting entirely. |
| `paths / exclude` | `array` `null` |  |  | If a file’s path matches any of these RegEx patterns, squarkup will be skipped for it.<br>Note the RegEx is matched against the file’s absolute path. |
| `paths / dest` | `string` |  | `src/routes/` | [ relative to site ]<br>Markdown files are exported relative to this directory. |
| `out / file-name` | `string` |  | `~content` | Name of exported `.svx` files (without the `.svx` file extension). |
| `out / site-data` | `string` |  | `src/site.json` | File where Squarkdown exports the site data. Relative to site. |
| `opts / on-error` | `option` | `warn` `kill` | `warn` | Action to take if an error is encountered while processing a file. |
| `opts / on-no-dir` | `string[]` | `ignore` `warn` `create` | `["warn"]` | Action to take if an export directory does not exist. |
| `bases / path` | `string` |  |  | [ relative to site ]<br>Squarkdown looks here for templates for generated `+page.svelte` and `+page.js`. |
| `bases / page.svelte` | `string` |  |  | [ relative to `bases / path` ]<br>Squarkdown uses this file as a template for generated `+page.svelte` files.<br>Required for Markdown exporting. |
| `bases / index.svelte` | `string` `null` |  |  | [ relative to `bases / path` ]<br>Squarkdown uses this file as a template for generated `+page.svelte` files for **index** pages. |
| `bases / page.js` | `string` `null` |  |  | [ relative to `bases / path` ]<br>Squarkdown uses this file as a template for generated `+page.js` files.<br>If not supplied, Squarkdown will not create `+page.js` files. |
| `bases / index-view` | `string` `null` |  |  | The component imported and used to render page lists in index pages.<br>If not supplied, Squarkdown will not create or inject index pages. |
| `styles / path` | `string` |  |  | [ relative to site ]<br>Squarkdown looks here for stylesheets. |
| `styles / page-styles` | `string` |  |  | [ relative to site ]<br>Squarkdown looks here for stylesheets to inject during squarkup. |
| `styles / base-style` | `string` |  |  | [ relative to `styles/page-styles` ]<br>Squarkdown injects this stylesheet into every page. |
| `assets / path` | `string` |  |  | [ relative to root ]<br>Squarkdown looks here for static assets to preprocess. |
| `assets / site-assets` | `string` `null` |  |  | [ relative to root ]<br>Squarkdown moves assets in this directory straight to the root of `site`/`static`. |
| `assets / extensions` | `string[]` |  | `["jpg", "jpeg", "png", "svg", "webp"]` | Only files with these extensions will be preprocessed by Squarkdown. |
| `fonts / queries` | `string[]` |  |  | Individual URL query params for requesting fonts from Google Fonts. |
<!-- #SQUARK inject. -->

<!-- #SQUARK leave. -->


<br>
