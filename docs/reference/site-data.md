# Site Data
<!-- #SQUARK live!
| dest = docs/reference/site-data
| date = 2025 November 4
| update = 2026 October 11
-->

After parsing every active file’s charm squark, Squarkdown has collected *quite a lot* of data about your site that you may want to keep. Squarkdown exports that data to a JSON file for you to use however you see fit.

```json
{
  "stats": {
    "built_on": "2020-04-01 00:00:00",
    "checked_files": 69,
    "active_pages": 42,
    "assets": 0
  },
  "pages": {
    "example-file.md": {
      "filepath": "docs/example-file.md",
      "destination": "docs/example-file",
      "flags": [],
      "title": "Example File",
      "heading": "Example File",
      "tags": ["examples"]
    },
  },
  "tags": {
    "examples": [
      "example-file.md"
    ]
  }
}
```

By default Squarkdown saves this file to `src/site.json` under your SvelteKit site folder. You can configure an alternative destination via [`out.site-data-path`](squarkup-config.md#site-data-path).


<br>


## Data

| Field | Type | Notes |
| ----- | ---- | ----- |
| `stats` | stat → value | Site-wide metadata and statistics |
| `pages` | filepath → `PageData<>` | Centralised access to every individual page’s metadata. Field names follow [`out. shorter-fields`](squarkup-config.md#shorter-fields). |
| `tags`  | tag name → filepaths | All of the tags found, and the pages with those tags. |


<br>


## Using Data

One way of using the exported data is to directly import the JSON in your JavaScript code.

```ts
import site from "#src/site.json" with { type: "json" };
```

> [!Warning]
> Yeah, this is not memory-efficient at all since it loads a ton of potentially unused data, but I haven’t found a better approach for static sites with client-side only logic. I’ve never had performance issues using it in my sites, even large ones like [Integrity](https://sup2point0.github.io/integrity), sooo...

Now, you can access the attributes directly with `dot.access`. For instance, to collect the metadata for pages with a particular tag:

```ts
let docs_pages = site.tags.docs.map(path => site.pages[path]);
```
