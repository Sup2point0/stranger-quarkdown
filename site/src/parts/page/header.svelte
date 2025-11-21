<!-- @component PageHeader -->

<script lang="ts">

interface Props {
  page: object;
}

let { page }: Props = $props();

</script>


<header>
  <h1> {page.head} </h1>

  {#if page.update_display}
    <p> Last updated <span>{page.update_display}</span> </p>
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
