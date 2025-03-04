<!-- @component Nav

The left navigation sidebar.
-->

<script lang="ts">

import NavSection from "#src/parts/nav/section.nav.svelte";
import NavLink from "#src/parts/nav/link.nav.svelte";

import { fade } from "svelte/transition";


let open = $state(true);

const delay = 240;

</script>


<nav style:--delay="{delay}ms"
  class:collapsed={!open}
  onclick={() => !open && (open = !open)}
>
  {#if open}
    <div
      in:fade={{ duration: 100, delay }}
      out:fade={{ duration: 100 }}
    >

  <NavSection title="Home" link="https://sup2point0.github.io/stranger-quarkdown">
    <NavLink text="FAQ" intern="info/faq" />
  </NavSection>

  <NavSection title="Walkthrough" intern="docs/walkthrough">
    <NavLink text="quickstart" intern="docs/quickstart" />
    <NavLink text="what is squarkdown?" intern="docs/what-is-squarkdown" />
    <NavLink text="project requirements" intern="docs/project-requirements" />
    <NavLink text="squarkdown-flavoured markdown" intern="docs/squarkdown-flavoured-markdown" />
    <NavLink text="project structure" intern="docs/project-structure" />
    <NavLink text="configuring squarkup for a repo" intern="docs/repo-config" />
    <NavLink text="configuring squarkup for a file" intern="docs/file-config" />
  </NavSection>

  <NavSection title="Features" intern="features">
    <NavLink text="page generation" intern="features/page-gen" />
    <NavLink text="assets" intern="features/assets" />
    <NavLink text="SCSS" intern="features/scss" />
    <NavLink text="fonts" intern="features/fonts" />
  </NavSection>

  <NavSection title="Docs" intern="docs">
    <NavLink code="glossary" intern="docs/glossary" />
    <NavLink code="squarkup.json" intern="docs/squarkup-json">
      <NavLink code="repo" intern="docs/squark-json#" />
      <NavLink code="paths / site" intern="docs/squark-json#site" />
      <NavLink code="paths / sources" intern="docs/squark-json#sources" />
      <NavLink code="paths / exclude" intern="docs/squark-json#exclude" />
      <NavLink code="paths / dest" intern="docs/squark-json#dest" />
      <NavLink code="out / file-name" intern="docs/squark-json#file-name" />
      <NavLink code="out / site-data" intern="docs/squark-json#site-data" />
      <NavLink code="opts / if-no-dir" intern="docs/squark-json#if-no-dir" />
      <NavLink code="opts / on-error" intern="docs/squark-json#on-error" />
      <NavLink code="bases / path" intern="docs/squark-json#path" />
      <NavLink code="bases / page.svelte" intern="docs/squark-json#page.svelte" />
      <NavLink code="bases / page.js" intern="docs/squark-json#page.js" />
      <NavLink code="styles / path" intern="docs/squark-json#path" />
      <NavLink code="styles / page-styles" intern="docs/squark-json#page-styles" />
      <NavLink code="assets / path" intern="docs/squark-json#path" />
      <NavLink code="assets / site-assets" intern="docs/squark-json#site-assets" />
      <NavLink code="assets / extensions" intern="docs/squark-json#extensions" />
      <NavLink code="fonts / queries" intern="docs/squark-json#queries" />
    </NavLink>
    <NavLink text="squark charm" intern="docs/squark-charm">
      <NavLink code="live" intern="docs/squark-charm#live" />
      <NavLink code="flags" intern="docs/squark-charm#flags" />
      <NavLink code="dest" intern="docs/squark-charm#dest" />
      <NavLink code="title" intern="docs/squark-charm#title" />
      <NavLink code="desc" intern="docs/squark-charm#desc" />
      <NavLink code="head" intern="docs/squark-charm#head" />
      <NavLink code="capt" intern="docs/squark-charm#capt" />
      <NavLink code="style" intern="docs/squark-charm#style" />
      <NavLink code="duality" intern="docs/squark-charm#duality" />
      <NavLink code="index" intern="docs/squark-charm#index" />
      <NavLink code="tags" intern="docs/squark-charm#tags" />
      <NavLink code="date" intern="docs/squark-charm#date" />
      <NavLink code="clean" intern="docs/squark-charm#clean" />
    </NavLink>
    <NavLink text="squarks" intern="docs/squarks">
      <NavLink code="leave" intern="docs/squarks/leave" />
      <NavLink code="only" intern="docs/squarks/only" />
    </NavLink>
    <NavLink text="site data" intern="docs/squarkup/site-data" />
    <NavLink text="cleanup" intern="docs/squarkup/cleanup" />
  </NavSection>

  <NavSection title="Info" intern="info/synopsis">
    <NavLink text="synopsis" intern="info/synopsis" />
    <NavLink text="rationale" intern="info/rationale" />
    <NavLink text="decoded" intern="info/decoded" />
    <NavLink text="credits" intern="info/credits" />
    <NavLink text="license" intern="info/license" />
  </NavSection>

  <div style:padding="1rem"></div>

    </div>
  {/if}

  <div class="container-button">
    <div class="button-back">
      <button id="show-hide" onclick={e => {
        e.stopPropagation();
        open = !open;
      }}>
        &larr;
      </button>
    </div>
  </div>
</nav>


<style lang="scss">

@use 'nav.interact' as *;


$base-width: max(15rem, 18vw);

nav {
  width: 100%;
  max-width: $base-width;
  padding: 2rem 1rem 0;
  position: relative;
  overflow-x: hidden;
  overflow-y: scroll;
  scrollbar-width: thin;
  background: light-dark(#fcfcfc, black);
  transition: max-width 0.6s cubic-bezier(0.19, 1, 0.22, 1);  // ease-out-expo
  transition-delay: 0ms;
  
  &.collapsed {
    max-width: 2rem;
    transition-delay: var(--delay);
    overflow: hidden;

    @include nav-interact;

    > * {
      opacity: 0;
    }
  }
}

// show-hide button
$size: 2.5rem;

.container-button {
  display: flex;
  flex-flow: row;
  justify-content: end;
  position: sticky;
  bottom: 1rem;
}

.button-back {
  width: $size;
  height: $size;
  position: absolute;
  right: 0;
  bottom: 0;
  background: white;
}

button#show-hide {
  width: $size;
  height: $size;
  @include font-code;
  font-size: 150%;
  background: light-dark(white, black);
  border: none;
  border-radius: $size / 2;
  box-shadow: 0 1px 3px $col-shadow;
  transition: background 0.1s ease-out;

  @include nav-interact($col-hover: black);
}

</style>
