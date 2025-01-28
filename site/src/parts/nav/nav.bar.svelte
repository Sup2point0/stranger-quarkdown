<!-- @component NavBar

The top navigation bar.
-->

<script lang="ts">

</script>


<nav>
  <section class="left">
    <a class="nav-link" href="https://sup2point0.github.io/stranger-quarkdown">
      StrangerQuarkdown
    </a>
  </section>

  <section class="right">
    <a class="nav-link" target="_blank" href="https://github.com/Sup2point0/stranger-quarkdown">
      GitHub
    </a>
  </section>
</nav>


<style lang="scss">

nav {
  width: 100%;
  padding: 0.5rem 2rem;
  z-index: 2;
  display: flex;
  flex-flow: row;
  justify-content: space-between;
  box-shadow: 0 0 4px lightgrey;
}

section {
  display: flex;
  flex-flow: row nowrap;
}

.nav-link {
  display: block;
  padding: 0.5rem;
  @include font-ui;
  color: light-dark(black, white);
  text-decoration: none;

  @include link($col-squark);
  color: unset;
}

</style>
