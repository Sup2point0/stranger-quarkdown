<!-- @component NavSection

A section in the left navigation pane.
-->

<script lang="ts">

interface Props {
  title: string;
  children?: any;
}

let { title, children }: Props = $props();

</script>


<section>
  <h3> {title} </h3>

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
  @include font-ui;
  color: $col-squark;
  font-size: 120%;
  font-weight: 450;
  border-radius: 0.4rem;
  transition: background 0.1s ease-out;

  @include nav-interact;
}

.links {
  padding-top: 0.4rem;
  display: flex;
  flex-flow: column nowrap;
  gap: 0.25rem;
}

</style>
