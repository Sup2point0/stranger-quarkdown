# squarkup.toml
<!-- #SQUARK live!
| dest = docs/reference/squarkup-toml
| update = 2026 October 6
-->

<div class="quicklinks" align="center">

[Example](#example)

</div>


<br>


## Example

Here’s what Squarkdown’s own `squarkup.toml` looks like:

```toml
[errors]
strict = true
on-error = "kill"
file-already-exists = "overwrite"
broken-link = "link-to-github"

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
