<script>

import "#styles/essence.scss";
import "#styles/prism-tokyo-night.css";

import NavTop from "#parts/nav/nav.bar.svelte";
import NavLeft from "#parts/nav/nav.pane.svelte";
import PageHeader from "#parts/page/header.svelte";
import PageFooter from "#parts/page/footer.svelte";

import { page } from "$app/state";


let { children } = $props();

</script>


<div class="ui-root-layout">
  <NavTop />

  <div class="ui-page-layout">
    <NavLeft />

    <main>
      <PageHeader title={page.data.head} />

      <article>
        {#if children}
          {@render children()}
        {:else}
          <p> Oops, something went wrong! </p>
        {/if}
      </article>

      <PageFooter />
    </main>
  </div>
</div>


<style lang="scss">

.ui-root-layout {
  width: 100%;
  height: 100vh;
  max-height: 100vh;
  display: flex;
  flex-flow: column nowrap;
}

.ui-page-layout {
  width: 100%;
  display: flex;
  flex-flow: row nowrap;
  flex-grow: 1;
  overflow: hidden;
}

main {
  width: 100%;
  padding: 1rem 4rem 3rem 2rem;
  overflow-y: scroll;
  background: light-dark(white, black);
}

</style>
