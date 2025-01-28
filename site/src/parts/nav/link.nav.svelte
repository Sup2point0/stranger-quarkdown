<!-- @component NavLink

A link in the navigation pane.
-->

<script lang="ts">

import { slide } from "svelte/transition";
import { base } from "$app/paths";


interface Props {
  text?: string;
    code?: string;
  link?: string;
    intern?: string;
    extern?: string;
  children?: any;
}

let { text, code, link, intern, extern, children }: Props = $props();


let open = $state(true);

</script>


<div class="nav-link">
  <a href={link || extern || `${base}/${intern}`}
    target={extern ? "_blank" : "_self"}
  >
    {#if code}
      <code>{code}</code>
    {:else}
      {text}
    {/if}
  </a>

  {#if children}
    <div class="arrow"
      onclick={() => open = !open}
      onkeydown={e => (e.key === "Enter" || e.key === "Space") && (open = !open)}
    >
      <img class:open alt="X" src="{base}/arrow.svg" />
    </div>
  {/if}
</div>

{#if children && open}
  <div class="child-links" transition:slide={{ duration: 300 }}>
    {@render children()}
  </div>
{/if}


<style lang="scss">

@use 'nav.interact.scss' as *;


.nav-link {
  width: 100%;
  display: flex;
  flex-flow: row nowrap;
  justify-content: stretch;
  align-content: stretch;
}

a {
  width: 100%;
  padding: 0.25em 0.5em;
  display: block;
  @include font-ui;
  color: light-dark(#0f0f0f, #f0f0f0);
  font-weight: 350;
  text-decoration: none;
  border-radius: 0.4rem;
  transition: color 0.16s, background 0.1s ease-out;

  @include nav-interact;

  code {
    @include font-code;
    font-size: 100%;
  }
}

.arrow {
  padding: 0.25em 0.4em;
  flex-grow: 0;
  display: flex;
  justify-content: center;
  align-items: center;
  border-radius: 0.4em;

  @include nav-interact;
}

img {
  height: 0.4em;
  opacity: 20%;
  transition: transform 0.2s ease-out;

  &.open {
    transform: rotate(180deg);
  }
}

.child-links {
  padding-left: 1em;
  padding-right: 0.5em;
}

</style>
