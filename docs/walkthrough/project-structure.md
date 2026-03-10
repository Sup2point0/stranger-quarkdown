# Project Structure
<!-- #SQUARK live!
| dest = docs/walkthrough/project-structure
| update = 2026 March 10
-->

Generally, the project structure Squarkdown expects is as follows:

```py
project/
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

- `/stranger-quarkdown/` is the StrangerQuarkdown repo, as a Git submodule.
  - This is how you run Squarkdown.
  - It may be preferable to alias this as `/.stranger-quarkdown/` to distinguish it from your actual project files.
- `/.squarkdown/` contains your project-specific configuration files.
  - You configure Squarkdown’s behaviour with `squarkup.json`.
  - You can provide additional content files in `./squarkdown/content` to squarkup, if you can’t find another suitable place for them.
- `/site/` is your Svelte/Kit project.
  - Squarkdown will export files to directories inside this folder.
  - Your directory might be called something different from `/site/`. Make sure you let Squarkdown know in `squarkup.json` if so!
