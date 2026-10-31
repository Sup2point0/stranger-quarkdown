# squarkup.toml
<!-- #SQUARK live!
| dest = docs/reference/squarkup-toml
| update = 2026 October 31
-->

<div class="quicklinks" align="center">

[Example](#example)

</div>

The main (and currently only) supported language for your [squarkup config](squarkup-config.md) is [TOML<sup>↗</sup>](https://toml.io/en/).


<br>


## Example

Here’s what Squarkdown’s own squarkup.toml looks like:

```toml
#:schema ../squarkdown/src/config/schema.json

[errors]
strict = true
debug = true
on-error = "kill"
file-already-exists = "overwrite"
link-broken = "error"
link-inactive = "link-to-github"

[paths]
site = "/site/"
sources = [
	"/",
	"/docs/",
	"/.squarkdown/content/",
]

[out]
folder = "src/routes/(docs)/(dyna)/"
file-name = "+page.svx"
site-data-path = "src/site.json"
render-page-ts = true
shorter-fields = false

[format]
preserve-comments = true
externalise-links = true

[fonts]
queries = [
	"Sora:wght@100..800",
]
```


<br>


## Tips

- Link the [config schema](https://sup2point0.github.io/stranger-quarkdown/schemas) at the top of your file to get error-checking, autocomplete and hover tooltips in your IDE:

  ```diff
  + #:schema https://sup2point0.github.io/stranger-quarkdown/schemas/latest.json

    [project]
    name = "Tips"
  ```

- Use TOML raw strings for RegEx patterns, so that you don’t need to escape `\` backslashes:

  ```diff
  [paths]
  exclude = [
    '/\..*\.md'
  ]
  ```
