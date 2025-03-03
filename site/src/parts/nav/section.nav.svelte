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
  <h3> <a href="{base}/{intern}">{title}</a> </h3>

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

h3 {
  padding: 0.3em 0.4em;
  font-size: 120%;
  color: $col-squark;
  border-radius: 0.4rem;
  transition: background 0.1s ease-out;

  @include nav-interact;

  a {
    @include font-ui;
    font-weight: 450;
    color: inherit;
    text-decoration: none;
    transition: color 0.16s;
  }
}

.links {
  padding-top: 0.4rem;
  display: flex;
  flex-flow: column nowrap;
  gap: 0.25rem;
}

</style>
