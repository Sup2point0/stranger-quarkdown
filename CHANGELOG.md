# Squarkdown Changelog


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


## v3.1.1

### Fixes
- Data extraction now correctly exits when it encounters the closing `-->` of the squark charm.


## v3.1.0

### New
- The squark charm now supports an `update` field. Use `date` for the original writing/publish date of a page, and `update` for subsequent updates.


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
