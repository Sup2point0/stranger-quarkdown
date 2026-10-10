<!-- @component Nav

The left navigation sidebar.
-->

<script lang="ts">

import Schema from "#static/squarkup-schema/latest.json" with { type: "json" };

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
    <NavLink text="Glossary" intern="docs/glossary" />
  </NavSection>

  <NavSection title="Walkthrough" intern="docs/walkthrough">
    <NavLink text="Quickstart" intern="docs/walkthrough/quickstart" />
    <NavLink text="What is Squarkdown?" intern="docs/walkthrough/what-is-squarkdown" />
    <NavLink text="Project Requirements" intern="docs/walkthrough/project-requirements" />
    <NavLink text="Project Structure" intern="docs/walkthrough/project-structure" />
    <NavLink text="Squarkdown-flavoured Markdown" intern="docs/walkthrough/squarkdown-flavoured-markdown" />
    <NavLink text="Making full use of Squarkdown" intern="docs/walkthrough/further-features" />
  </NavSection>

  <NavSection title="Reference" intern="reference">
    <NavLink text="Squarkup Config" intern="docs/reference/squarkup-config">
      <NavLink code="paths.site"    intern="docs/reference/squark-config#site" />
      <NavLink code="paths.sources" intern="docs/reference/squark-config#sources" />
      <NavLink code="paths.include" intern="docs/reference/squark-config#include" />
      <NavLink code="paths.exclude" intern="docs/reference/squark-config#exclude" />
      <NavLink code="out.folder" intern="docs/reference/squark-config#out-folder" />
      <NavLink code="out.file&#8209;name" intern="docs/reference/squark-config#file-name" />
      <NavLink code="out.site&#8209;data&#8209;path" intern="docs/reference/squark-config#site-data-path" />
      <NavLink code="assets.folder" intern="docs/reference/squark-config#assets-folder" />
      <NavLink code="assets.site&#8209;assets&#8209;folder" intern="docs/reference/squark-config#site-assets-folder" />
      <NavLink code="assets.extensions" intern="docs/reference/squark-config#extensions" />
      <NavLink code="fonts.queries" intern="docs/reference/squark-config#queries" />
      <NavLink code="errors.strict" intern="docs/reference/squark-config#on-no-dir" />
      <NavLink code="errors.on&#8209;error" intern="docs/reference/squark-config#on-no-dir" />
      <NavLink code="errors.file&#8209;already&#8209;exists" intern="docs/reference/squark-config#on-no-dir" />
      <NavLink code="errors.link&#8209;broken" intern="docs/reference/squark-config#on-no-dir" />
    </NavLink>
    <NavLink text="Charm Squark" intern="docs/reference/charm-squark">
      <NavLink code="live!"        intern="docs/reference/charm-squark#live" />
      <NavLink code="destination"  intern="docs/reference/charm-squark#destination" />
      <NavLink code="title"        intern="docs/reference/charm-squark#title" />
      <NavLink code="description"  intern="docs/reference/charm-squark#description" />
      <NavLink code="heading"      intern="docs/reference/charm-squark#heading" />
      <NavLink code="caption"      intern="docs/reference/charm-squark#caption" />
      <NavLink code="tags"         intern="docs/reference/charm-squark#tags" />
      <NavLink code="release-date" intern="docs/reference/charm-squark#release-date" />
      <NavLink code="last-update"  intern="docs/reference/charm-squark#last-update" />
      <NavLink code="cleanse"      intern="docs/reference/charm-squark#cleanse" />
    </NavLink>
    <NavLink text="Squarks" intern="docs/reference/squarks">
      <NavLink code="slash" intern="docs/reference/squarks#slash" />
      <NavLink code="only" intern="docs/reference/squarks#only" />
      <NavLink code="leave" intern="docs/reference/squarks#leave" />
    </NavLink>
    <!-- <NavLink text="site data" intern="docs/reference/site-data" /> -->
    <NavLink text="CLI" intern="docs/reference/cli">
    </NavLink>
  </NavSection>

  <!-- <NavSection title="Features" intern="features">
    <NavLink text="Link Rewriting" intern="features/link-rewriting" />
    <NavLink text="Markdown Sanitisation" intern="features/cleanse" />
    <NavLink text="Assets" intern="features/assets" />
    <NavLink text="Fonts" intern="features/fonts" />
  </NavSection> -->

  <NavSection title="Schemas" intern="squarkup-schemas" open={false}>
    <NavLink text="latest (v{Schema.version})" intern="squarkup-schema/latest.json" />
    <NavLink text="v5.0.7" intern="squarkup-schema/v5.0.7.json" />
  </NavSection>

  <NavSection title="Info" intern="info/synopsis">
    <NavLink text="Synopsis" intern="info/synopsis" />
    <NavLink text="Rationale" intern="info/rationale" />
    <NavLink text="Decoded" intern="info/decoded" />
    <NavLink text="Licence" intern="info/licence" />
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


$base-width: max(15rem, 20vw);

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
  border-radius: calc($size / 2);
}

button#show-hide {
  width: $size;
  height: $size;
  @include font-code;
  font-size: 150%;
  background: light-dark(white, black);
  border: none;
  border-radius: calc($size / 2);
  box-shadow: 0 1px 3px $col-shadow;
  transition: background 0.1s ease-out;

  @include nav-interact($col-hover: black);
}

</style>
