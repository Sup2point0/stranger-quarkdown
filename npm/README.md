# Stranger Quarkdown

Markdown preprocessing automation for Svelte/Kit projects.

Please visit [the GitHub](https://github.com/Sup2point0/stranger-quarkdown#readme) for the full README :]


<br>


## Quickstart

Run:

```bash
> npx squarkdown
```

With extras:

```bash
> npx squarkdown --assets --fonts
```

Remind yourself:

```bash
> npx squarkdown --help
```


<br>


## Library

```svelte
<script lang="ts">
	import type { PageData } from "stranger-quarkdown";

	import { page } from "$app/state";

	let page_data: PageData<"long"> = $derived(page.data);
	
	// if you set `out.shorter-fields = true`:
	let shorter_page_data: PageData<"short"> = $derived(page.data);
</script>
```


<br>


## Squarkdown-Flavoured Markdown

```md
# Welcome to Squarkdown!
<!-- #SQUARK live!
| dest = walkthrough welcome
| capt = An introduction to what Squarkdown can do
| date = 2024 July 1
-->

This link to [other-file.md](other-file.md) will have its `.md` extension stripped.

<!-- #SQUARK slash? -->
This content won’t be included in the output.
<!-- #SQUARK slash. -->

<!-- #SQUARK only?

This content only shows up in the output.

     #SQUARK only. -->
```
