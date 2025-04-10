<!-- @component PageHeader -->

<script lang="ts">

interface Props {
  title: string;
}

let { title }: Props = $props();

</script>


<header>
  <h1> {title} </h1>
</header>


<style lang="scss">

header {
  padding: 2rem 0;
}

h1 {
  width: 80%;
  padding-bottom: 0.25em;
  @include font-ui;
  font-size: 250%;
  font-weight: 300;
  border-bottom: 1px solid $col-charm;
}

</style>
