# Squarkdown Changelog


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
