<!-- @component CopyCode

A wrapper that adds copy buttons to code blocks on a page.
-->

<script lang="ts">

import CopyClicky from "#parts/ui/copy-clicky.svelte"

import { mount, onMount } from "svelte";
import { browser } from "$app/environment";

let { children } = $props();


let mounted = false;

onMount(() => {
  if (mounted || !browser) return;

  let blocks = document.getElementsByTagName("pre");

  let lang: boolean;

  for (let block of blocks) {
    lang = [...block.classList].some(cls => cls.startsWith("language-"));
    if (!lang) continue;

    block.style.position = "relative";

    mount(CopyClicky, {
      target: block,
      props: { value: block.innerText }
    });
  }

  mounted = true;
});

</script>


{@render children?.()}
