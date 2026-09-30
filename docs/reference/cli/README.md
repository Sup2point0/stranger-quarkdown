# Squarkdown CLI
<!-- #SQUARK live!
| dest = docs/reference/cli
| update = 2026 September 8
-->

Squarkdown is a CLI tool! This page provides an overview of the different commands included in Squarkdown.

To run these commands, you’ll need to have [cloned the Stranger Quarkdown repo as a submodule](../../walkthrough/quickstart.md#add-squarkdown) into your project, and installed Ruby. Remember to also install Stranger Quarkdown’s dependencies before using it:

```bash
your-project/stranger-quarkdown> bundle install
```


<br>


<!-- #SQUARK slash? -->
## [`rake init`](init.md)
<!-- #SQUARK slash. -->

Setup Squarkdown in your project.

```bash
your-project/stranger-quarkdown> rake init

#  ┌  Welcome to Squarkdown!  ──────────────────────────────────────────
#  │
#  ▸  You’re running version 3.1.0
#  └  any key to continue
```

This automatically builds your `squarkup.json` for you, so you don’t need to remember all the setting keys. It will ask you a series of questions on your project’s needs, and construct `squarkup.json` according to your answers (just like Svelte’s CLI).


<br>


<!-- #SQUARK leave? -->
## [`rake squark`](squark.md)
<!-- #SQUARK leave. -->

Squarkup your project.

```bash
your-project/stranger-quarkdown> rake squark

#  >>> Squark / running Squarkdown v3.2.0
#             / locating routes...
#             ✓ found root = /your-project/stranger-quarkdown
#             ✓ found repo = /your-project
#  ...
```

This runs Squarkdown, using your configurations specified in `squarkup.json`.

To use [extended features of Squarkdown](../../walkthrough/further-features.md), first ensure you have configured the relevant settings in `squarkup.json`, then opt-in when running the command by supplying them as arguments:

```bash
your-project/stranger-quarkdown> rake squark[fonts,scss,assets]

# or any subset
/your-project/stranger-quarkdown> rake squark[fonts,assets]

# in any order
/your-project/stranger-quarkdown> rake squark[assets,scss]
```

Since these may not change very often, you can often skip preprocessing for them to speed up build times in development. You may find it helpful to configure 2 separate squarkup commands in your `package.json`, e.g.

```json
// package.json
{
  // ..
  "scripts": {
    "squark":  "cd stranger-quarkdown && rake squark",
    "squarkx": "cd stranger-quarkdown && rake squark[fonts,scss,assets]",
  },
  // ..
}
```


<br>
