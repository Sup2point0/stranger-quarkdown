<!-- @component NavSection

A section in the left navigation pane.
-->

<script lang="ts">

import { base } from "$app/paths";


interface Props {
  title: string;
  intern: string;
  children?: any;
}

let { title, intern, children }: Props = $props();

</script>


<section>
  <a href="{base}/{intern}">{title}</a>

  <div class="links">
    {@render children?.()}
  </div>
</section>


<style lang="scss">

@use 'nav.interact.scss' as *;


section {
  width: 100%;
  padding-bottom: 1.5rem;
}

a {
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

.links {
  padding-top: 0.4rem;
  display: flex;
  flex-flow: column nowrap;
  gap: 0.25rem;
}

</style>
