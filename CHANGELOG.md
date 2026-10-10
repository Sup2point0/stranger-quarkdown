# Squarkdown Changelog

<!-- pass in path to squarkup.toml -->


<br>


## Next (v4.2.1)

### Fixes
- Strip colours from `squarkdown --version` for compatibility with other standard command-line tools


## v4.2.0

### New
New squarkup config options:

- `project` options:
  - `project.name`: The displayed name of your project.
  - `project.github`: The GitHub name of your project.

- `format.inject-head` to inject `<head>` metadata into rendered pages (enabled by default):

  ```md
  <svelte:head>
    <title> Some Page · My Project </title>
    <meta name="description" content="Never gonna give you up" />
  </svelte:head>
  ```

  - `My Project` is replaced with `project.name`.

- `errors.debug` to enable more helpful debug output
  - When enabled, error messages will show a snapshot of where in the source text they were encountered!

  ```hs
  ─────────────────────────────
  × 1 error while rendering example.md
  ─────────────────────────────
  × unknown twin squark: unknown
  │  
  │      6 │  Hmmm, this squark looks suspicious
  │      7 │  <!-- #SQUARK unknown? -->
  │      8 │  Squarkdown will ignore it.
  │  
  = hint: valid twin squarks are leave · slash · only
  ─────────────────────────────
  ```


<br>


## v4.1.5

### Fixes
- Fix slight issues with line breaks in render output.


## v4.1.4

### Fixes
- Fix issues with `#SQUARK leave` interacting with `#SQUARK only`.


## v4.1.3

### Fixes
- Fix issue with squarks being processed inside code contexts.
- Remove stray debug output that snuck into production...


## v4.1.2

### Fixes
- Properly handle `<!-- #SQUARK only? ... #SQUARK only. -->`.
- Improve Markdown rendering to cover more edge cases involving `<!-- comments -->`.


## v4.1.1

### Changes
- Use SIMD when parsing Markdown (on x86-64), which should mean speedups in performance!

### Fixes
- Include `.shard` when rendering `+page.ts`.
- Correct `.filepath` -> `.path` in `PageData<"short">`.


## v4.1.0

### Breaking
- Flatten arbitrary fields in `+page.ts` (like in `site.json`).
  - Before:

  ```ts
  return {
    filepath: "readme.md",
    ...,
    other: {
      some_arbitrary_field: true,
      random_message: ["sup", "world"],
    },
  };
  ```

  - Now:

  ```ts
  return {
    filepath: "readme.md",
    ...,
    "some_arbitrary_field": true,
    "random_message": ["sup", "world"],
  };
  ```

  - Arbitrary fields are also now surrounded with `""` for safety.

### New
- Render pages in parallel!
  - Pass `--no-parallel` to disable parallelism (internal feature, likely to be removed in future).
- Infer a page’s destination if not provided when `errors.strict = false`.
  - This means the bare minimum for a file to be squarked up is now just `<!-- #SQUARK live! -->` with nothing more:

  ```md
  # Page
  <!-- #SQUARK live! -->
  ```

  - If this page were under `/docs/page.md`, its destination would inferred to be `/site/src/routes/docs/page/`.
  - However, for safety it’s still best to always explicitly state the destination, so `errors.strict` must be disabled to opt-in to this.


<br>


## v4.0.5

### Fixes
- Fix issues with maths rendering (e.g. `$[1, 2]$` being rendered to `$\[1, 2\]$`).
- `import type { PageData } from` `"stranger-quarkdown"` instead of `"squarkdown"`.

### Changes
- Site data uses sorted `BTreeMap` instead of `HashMap`, so keys are now sorted alphabetically.


## v4.0.4

### New
- Check `out.site-data-path` is under your project root when `errors.strict = true`.

### Fixes
- Fix resolution of links with both `%20` and a `#section` anchor (e.g. `other%20file#introduction`).


## v4.0.3

### New
- Site data now respects `out.shorter-fields = true`.


## v4.0.2

### New
- `squarkdown --version` command

### Fixes
- Improve command arguments parsing to be more robust.


## v4.0.1

### New
- Validate `| field =` that appear before `---` are strictly Squarkdown-native fields:

  ```md
  # Page
  <!-- #SQUARK live!
  | dest = page
  | date = 2026 October 6
  | unknown = not allowed!
  ---
  | arbitrary = allowed
  -->
  ```

  - Opt in with `errors.strict = true`.

### Fixes
- Fix `release-date` and `last-update` long-form fields.
- Correctly override `# Heading` when `| heading =` is set.
  - Before, this would result in the page having `heading: "Upper"`:

    ```md
    # Upper
    <!-- #SQUARK live!
    | heading = Lower
    -->
    ```

  - Now it is correctly overridden to `"Lower"`.
  - (This only applied for the long-form `heading`, not the shorthand `head`.)


<br>


## v4.0

Squarkdown has been rewritten in Rust. Yeah, I’m sorry, lmao.

This keeps the core semantics and functionality of Squarkdown the same, but brings your usual speed and reliability improvements that come with Rust.

While I was at it, I also implemented some way overdue features that were too scary to implement in Ruby!

While this release means more stability for Squarkdown, there’s still a fair amount of stuff that isn’t yet implemented, so I’ll slowly be sorting those out over the next few releases.

### Breaking
- Squarkdown is now installable as a cross-platform binary from [crates.io](https://crates.io/crates/squarkdown) or [npm](https://www.npmjs.com/package/stranger-quarkdown), instead of requiring a local Ruby installation.
  - Install it with `cargo install squarkdown`/`npm install stranger-quarkdown`
  - Run it with `squarkdown`/`npx squarkdown`
- Squarkup configuration has been overhauled; see [config](#config)
- `+page.svelte` and `+page.js` bases have been removed, since they were unnecessary
- SCSS preprocessing has been removed, since it wasn’t that useful

### New
- Squarkdown now resolves links, rewriting internal links to other Markdown files or assets into links to webpages
  - Links with `%20`-encoded spaces are normalised to `-`
- Comment stripping when rendering
- Improved error messages with contexts, hints and diagnostics
- Safer path resolution, with stricter checks to ensure paths remain inside your repository
- `stranger-quarkdown` npm package exposes `PageData<"short">` and `PageData<"long">` types for `+page.ts` files

### Rendering
- Squarkdown now uses a proper context-aware Markdown parsing engine, instead of global RegEx substitutions, so `#SQUARK`s are properly handled inside code blocks!
- New `#SQUARK slash` squark, for removing content from rendered output

### Config
- Squarkdown now uses `squarkup.toml` instead of `squarkup.json`
  - Squarkdown also accepts a `squarkup.toml` in your project root, instead of `.squarkdown/squarkup.toml`
- Many config options have been renamed:
  - `opts / on-error` -> `errors.on-error`
  - `opts / on-no-dir` -> `errors.file-already-exists` (with semantic change)
  - `paths / dest` -> `out.folder`
  - `out / site-data` -> `out.site-data-path`
  - `assets / path` -> `assets.folder`
  - `assets / site-assets` -> `assets.site-assets-folder`
- New options:
  - `errors.strict` to enable stricter checks
  - `errors.link-broken` for broken link handling
  - `out.render-page-ts` to produce `+page.ts` files
  - `out.shorter-fields` to prefer shorter field names
  - `out.site-data-path` for where to export `site.json`
  - `format.preserve-heading` to preserve `# Heading`
  - `format.preserve-comments` to preserve `<!-- comments -->`
  - `format.externalise-links` (not yet implemented)


<br>


## v3.4.3

Squarkup Schema version: `5.0.10`

### Fixes
- `rake squark`: Fix dead error handling code when finding `squarkup.json`
- `rake squark`: Remove spurious "no file base found" errors
- `[fonts]`: Automatically replace ` ` with `+` in queries
- `rake init`: Fix incorrect key names
- `rake init`: Fix incorrect handling of extensions
- `rake init`: Fix incorrect path name in error message
- Print backtraces on errors


## v3.4.2

Squarkup Schema version: `5.0.10`

### Fixes
- `rake squark`: Correctly terminate when neither `paths / sources` nor `paths / exclude` provided in `squarkup.json`
- `rake squark`: Correctly include `.dir` and `.file` paths regardless of `paths / sources` settings
- File processing: Stricten RegEx parsing of fields so things like `| header =` can't be mistaken for `| head =`
- `rake init`: Fix improper escaping when selecting `.` for `paths / exclude`


## v3.4.1

Squarkup Schema version: `5.0.9`

### Fixes
- SCSS prep: Updated to use `includePaths` for compatibility with newer versions of SCSS
- SCSS prep: Remove unnecessary `.scss` extensions from `@use` statements
- Fonts prep: Handle empty `fonts / queries` edge case
- Fonts prep: When there’s no existing Google Fonts `<link>` query, inject it instead of silently no-opping
- `rake init`: fix issues
- Update gem dependencies


## v3.4.0

### Breaking
- Updated vulnerable dependencies

### Fixes
- Fix list and non-list distinguishing for arbitrary fields


<br>


## v3.3.3

### Fixes
- Distinguish between non-list and singleton list values for arbitrary fields


## v3.3.2

### Fixes
- Arbitrary fields auto-convert `-` in identifiers to `_` in exported metadata


## v3.3.1

### Fixes
- Fix arbitrary field processing
- Correctly export `rest` arbitrary fields in file data


## v3.3.0

### New
- Squarkdown now accepts values over multiple lines
- Squarkdown now accepts arbitrary fields in the squark charm after a `---` delimiter


<br>


## v3.2.5

### New
- Squarkdown now accepts spaces around `=` in squark charm, if you wish to align the `=` with extra whitespace


## v3.2.4

### New
- New options in CLI
- CLI now adds `$schema` field to exported JSON

### Fixes
- Visual, logic and fallback fixes for CLI
  - All `enter manually` options now ask for input
  - Output JSON no longer has comments or malformed syntax


## v3.2.3

### Fixes
- `@update` and `@update_display` fields for files are now correctly exposed
- `FileData.update_fields()` has more exhaustive RegEx checks for correct patterns


## v3.2.2

### New
- Added `ESC to exit` hint in CLI


## v3.2.1

### Fixes
- Fix syntax error in `rake init` script...


## v3.2.0

### Breaking
- Bumped Ruby version from `3.3.5` to `3.4.7`


<br>


## v3.1.1

### Fixes
- Data extraction now correctly exits when it encounters the closing `-->` of the squark charm.


## v3.1.0

### New
- The squark charm now supports an `update` field. Use `date` for the original writing/publish date of a page, and `update` for subsequent updates.


<br>


## v3.0.4

### Fixes
- [Fixes v3.0.2] Errors now pass the `repo_config:` parameter


## v3.0.3

### Fixes
- `rake init` now ignores `/stranger-quarkdown/`'s own `.squarkdown` folder.


## v3.0.2 [broken]

### Fixes
- Errors now correctly crash execution when `opts / on-error` is set to `kill`.


## v3.0.1

### Fixes
- Anchor links in Markdown now correctly have `.md` removed while keeping the `#anchor`.


## v3.0.0

### New
- All-new CLI for setting up Squarkdown in a project
- Revamped `squarkup.json` field names to be clearer and less ambiguous
  - Extensions for asset preprocessing can now be specified

### Breaking
- Renamed `shard` to `tag` (should’ve done this way sooner)

### Fixes
- Improved debug output
