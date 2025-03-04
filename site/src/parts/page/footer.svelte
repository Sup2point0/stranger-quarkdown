<!-- @component PageFoter -->

<footer>
  <div class="left">
    <p> See something that needs fixing? Drop an <a target="_blank" href="https://github.com/Sup2point0/stranger-quarkdown">issue on GitHub</a>! </p>
  </div>

  <div class="right">
    <p> Squarkdown by <a target="_blank" href="https://github.com/Sup2point0">Sup#2.0</a> </p>
  </div>
</footer>


<style lang="scss">

footer {
  padding: 1em 0 0;
  margin: 3em 0 0;
  display: flex;
  flex-flow: row wrap;
  justify-content: space-between;
  border-top: 1px solid $col-line;
}

p {
  display: inline-block;
  @include font-ui;
  color: $col-text-deut;

  a {
    @include underlined-link;
  }
}

</style>
