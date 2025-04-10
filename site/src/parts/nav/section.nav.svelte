<!-- @component NavSection

A section in the left navigation pane.
-->

<script lang="ts">

import { slide } from "svelte/transition";
import { base } from "$app/paths";


interface Props {
  title: string;
  link?: string;
    intern?: string;
  children?: any;
}

let { title, link, intern, children }: Props = $props();


let open = $state(true);

</script>


<section>
  <div class="title">
    <a href="{link || `${base}/${intern}`}">
      {title}
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
      {@render children?.()}
    </div>
  {/if}
</section>


<style lang="scss">

@use 'nav.interact.scss' as *;


section {
  width: 100%;
  padding-bottom: 1.5rem;
}

.title {
  width: 100%;
  display: flex;
  flex-flow: row nowrap;
  justify-content: stretch;
  align-content: stretch;
  
  a {
    flex-grow: 1;
    display: block;
    padding: 0.3em 0.4em;
    @include font-ui;
    font-size: 120%;
    font-weight: 450;
    color: $col-squark;
    text-decoration: none;
    border-radius: 0.4rem;
    transition: color 0.16s, background 0.1s ease-out;
    @include nav-interact;
  }
}

.arrow {
  padding: 0 0.75rem;
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
  padding-top: 0.4rem;
  display: flex;
  flex-flow: column nowrap;
  gap: 0.25rem;
}

</style>
