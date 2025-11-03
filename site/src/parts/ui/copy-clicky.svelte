<!-- @component CopyClicky

A button to copy something to clipboard.
-->

<script module>

/** Counter for assigning unique IDs to each copy button. */
let total = 0;

/** The shard of the last clicked copy button.*/
let current = $state(0);

</script>

<script lang="ts">

import { base } from "$app/paths";


interface Props {
  value: string | undefined;
}

let { value }: Props = $props();


total += 1;
const shard = total;

</script>


<button class="copy"
  onclick={() => {
    if (!value) return;
    navigator.clipboard.writeText(value);
    current = shard;
  }}
>
  {#if current === shard}
    <img alt="" src="{base}/tick.svg" />
  {:else}
    <img alt="" src="{base}/copy.svg" />
  {/if}
</button>


<style lang="scss">

button.copy {
  width: 2rem;
  height: 2rem;
  aspect-ratio: 1;
  padding: 0.5em;
  position: absolute;
  top: 0.6rem;
  right: 1rem;
  background: rgb(black, 12%);
  border: none;
  border-radius: 0.5em;
  transition: background 0.12s ease-out;
  filter: invert(100%);

  &:hover, &:focus {
    cursor: pointer;
    background: rgb(black, 24%);
  }

  &:active {
    background: rgb(black, 18%);
  }
}

img {
  width: 2rem;
  max-width: 100%;
}

</style>
