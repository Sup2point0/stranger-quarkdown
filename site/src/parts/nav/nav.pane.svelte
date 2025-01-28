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

  <NavSection title="Home">
    <NavLink text="what is squarkdown?" />
    <NavLink text="project requirements" />
    <NavLink text="getting started" />
    <NavLink text="FAQ" />
  </NavSection>

  <NavSection title="Features">
    <NavLink text="assets" />
    <NavLink text="SCSS" />
    <NavLink text="Google Fonts" />
  </NavSection>

  <NavSection title="Reference">
    <NavLink code="squarkup.json" />
    <NavLink text="project structure" />
    <NavLink text="squark charm">
      <NavLink code="live" />
      <NavLink code="flags" />
      <NavLink code="dest" />
      <NavLink code="title" />
      <NavLink code="desc" />
      <NavLink code="head" />
      <NavLink code="capt" />
      <NavLink code="style" />
      <NavLink code="duality" />
      <NavLink code="index" />
      <NavLink code="shard" />
      <NavLink code="date" />
      <NavLink code="clean" />
    </NavLink>
    <NavLink text="squarks">
      <NavLink text="only" />
      <NavLink text="leave" />
    </NavLink>
  </NavSection>

  <NavSection title="Info">
    <NavLink text="synopsis" />
    <NavLink text="decoded" />
    <NavLink text="credits" />
    <NavLink text="license" />
  </NavSection>

  <div style:padding="1rem"></div>

    </div>
  {/if}

  <div class="container-button">
    <button id="show-hide" onclick={e => {
      e.stopPropagation();
      open = !open;
    }}>
      &larr;
    </button>
  </div>
</nav>


<style lang="scss">

@use 'nav.interact' as *;


$base-width: 15rem;

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

    > * {
      opacity: 0;
    }
  }
}

.container-button {
  display: flex;
  flex-flow: row;
  justify-content: end;
  position: sticky;
  bottom: 1rem;
}

button#show-hide {
  width: 2rem;
  height: 2rem;
  position: absolute;
  right: 0;
  bottom: 0;
  @include font-code;
  font-size: 150%;
  background: none;
  border: none;
  border-radius: 1rem;
  @include nav-interact;
}

</style>
