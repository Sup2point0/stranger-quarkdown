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
