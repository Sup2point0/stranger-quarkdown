# Project Structure
<!-- #SQUARK live!
| dest = docs/walkthrough/project-structure
| update = 2026 July 11
-->

A project using Squarkdown looks like this:

```py
your-project/
   .squarkdown/
      squarkup.json

      content/
         example-file.md
         ...

   stranger-quarkdown/...

   site/
      src/
         app.html
         routes/...
      static/...
```

- `stranger-quarkdown/` is the Stranger Quarkdown repo, as a Git submodule.
  - This is how you run Squarkdown.
  - It may be preferable to alias this as `.stranger-quarkdown/` to distinguish it from your actual project files.
- `.squarkdown/` contains your project-specific configuration.
  - You configure Squarkdown’s behaviour with `squarkup.json`.
  - You can provide additional content files in `.squarkdown/content`, if you can’t find another suitable place for them.
- `site/` is your Svelte/Kit project.
  - Squarkdown will export files to directories inside this folder.
  - Your directory might be called something different from `site/`. Make sure you let Squarkdown know in `squarkup.json` if so!

Squarkdown recursively looks for `.md` from `project/`, preprocesses active files it finds, and outputs to `site/src/routes/`.

Of course, your project may require a different structure, so these paths are [all configurable](../reference/squarkup-json.md) in `squarkup.json`.
