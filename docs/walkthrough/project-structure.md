# Project Structure
<!-- #SQUARK live!
| dest = docs/walkthrough/project-structure
| update = 2025 November 1
-->

Generally, the project structure Squarkdown expects is as follows:

```
./
   .squarkdown/
      squarkup.json
      content/
         file.md
         ...

   stranger-quarkdown/...

   site/
      src/
         routes/...
         ...
      static/...
```

- `/stranger-quarkdown/` is the StrangerQuarkdown repo, as a Git submodule. This is how you run Squarkdown.
  - It may be preferable to alias this as `/.stranger-quarkdown/` to distinguish it from your actual project files.
- `/.squarkdown/` contains your project-specific files. This is how you configure Squarkdown.
  - You can provide additional content files to squarkup that you can’t find another suitable place for in `./squarkdown/content`.
- `/site/` is your Svelte/Kit project. Your directory could be called something different from `/site/`.
