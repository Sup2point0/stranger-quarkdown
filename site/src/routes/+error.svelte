<script lang="ts">

import "#styles/essence.scss";

import NavTop from "#parts/nav/nav.bar.svelte";

import { page } from "$app/state";

</script>


<svelte:head>
  <title> {page.data?.title} </title>
  <meta name="description" content={page.data?.description} />
</svelte:head>


<div class="ui-root-layout">
  <NavTop />

  <main>
    <div class="error">
      <h1> {page.status ? `${page.status}` : "ERROR"} </h1>
      <p> {page.status === 404 ? "This page doesn’t exist!" : "Oops, something went wrong!"} </p>
    </div>

    <button id="back" onclick={() => history.back()}>
      Return to previous page
    </button>
  </main>
</div>


<style lang="scss">

.ui-root-layout {
  width: 100%;
  height: 100vh;
  max-height: 100vh;
  display: flex;
  flex-flow: column nowrap;
}

main {
  padding-top: 10vh;
  display: flex;
  flex-flow: column;
  justify-content: center;
  align-items: center;
  @include font-ui;
}

.error {
  margin-bottom: 3rem;
  text-align: center;

  h1 {
    margin-bottom: -0.1em;
    font-size: 40vh;
    color: #ff5c8733;
  }

  p {
    color: $col-text-deut;
    font-size: 200%;
    font-weight: 350;
  }
}

button#back {
  margin-top: 2rem;
  @include font-ui;
  font-size: 100%;
  @include block-link;
}

</style>
