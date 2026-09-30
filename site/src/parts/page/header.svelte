<!-- @component PageHeader -->

<script lang="ts">

import type { PageData } from "stranger-quarkdown";

interface Props {
  page: PageData;
}

let { page }: Props = $props();

</script>


<header>
  <h1> {@html page.heading} </h1>

  {#if page.last_update_raw}
    <p> Last updated <span>{page.last_update_raw}</span> </p>
  {/if}
</header>


<style lang="scss">

header {
  position: relative;
  padding: 2rem 0;
}

h1 {
  width: 72%;
  padding-bottom: 0.25em;
  @include font-ui;
  font-size: 250%;
  font-weight: 300;
  border-bottom: 1px solid $col-charm;
}

p {
  position: absolute;
  bottom: calc(2rem - 5px);
  right: 0;
  @include font-ui;
  color: $col-text-deut;
  text-align: right;

  span {
    padding-left: 0.2em;
    color: $col-down;
  }
}

</style>
