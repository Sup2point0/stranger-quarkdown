# squarkup.json
<!-- #SQUARK live!
| dest = docs/reference/squarkup-json
| update = 2026 July 10
-->

<div class="quicklinks" align="center">

[Example](#example)

</div>

`squarkup.json` goes in a `.squarkdown/` directory in your project’s root directory. It’s how you configure Squarkdown to suit your project’s needs.

The structure is flat to keep things simple. Keys follow a `<category> / <option>` format.

Most settings simply tell Squarkdown what filepaths to use for your particular project, since these can obviously be different between projects!

<br>


## Fields

> [!Note]
> This table is auto-generated from the exact specification as laid out by the [JSON schema](https://sup2point0.github.io/stranger-quarkdown/squarkup-schema/latest.json).

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


<br>


## Example

Here’s what Squarkdown’s own `squarkup.json` looks like:

```json
{
  "$schema": "../squarkdown/resources/squarkup-schema.json",

  "repo": "Squarkdown",

  "paths / site": "site",
  "paths / sources": [
    "./",
    "docs/",
    ".squarkdown/content/"
  ],
  "paths / dest": "src/routes/(docs)/",
  
  "out / file-name": "~content",
  "out / site-data": "src/site.json",

  "opts / on-no-dir": ["warn", "create"],
  "opts / on-error": "kill",

  "bases / path": "src/parts/bases/",
  "bases / page.svelte": "~page.svelte",
  "bases / page.js": "~page.js",

  "styles / path": "src/styles/",
  "styles / page-styles": "src/styles/",
  "styles / base-style": "article",

  "assets / path": ".assets",
  "assets / site-assets": ".assets/site",
  "assets / extensions": [
    "jpg", "jpeg", "png", "svg", "ttf"
  ],

  "fonts / queries": [
    "Sora:wght@100..800"
  ]
}
```

> (comments added for clarity)

Hopefully, it should be pretty self-explanatory what all the settings do!

Writing it by hand can be tedious, so remember the Squarkdown CLI can [do it for you](cli/)!
