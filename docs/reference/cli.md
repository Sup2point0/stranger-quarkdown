# Squarkdown CLI
<!-- #SQUARK live!
| dest = docs/reference/cli
| update = 2026 October 31
-->

Squarkdown is a command-line application with a small, focused set of commands.

> [!Tip]
> You’ll notice a lot of options are shared between commands. This should make things a lot more intuitive and familiar!


<br>


## Help

```bash
squarkdown help
squarkdown --help
squarkdown -h
```

This shows the core Squarkdown commands available.[^variants]

[^variants]: Why so many variants for the same command? Well, when you want help there’s nothing more frustrating than *not being able to run the help command* ;)


<br>


## Version

```bash
squarkdown --version
```

This shows which version of Squarkdown you have installed, in plaintext.


<br>


## Setup

```bash
squarkdown init
```

This initialises your [squarkup.toml](squarkup-toml.md) for you, much like `npm init` or `sv create`.

You may find this helpful if you’re new to Squarkdown and are overwhelmed by all the configuration options!



<br>


## Squarkup

```bash
squarkdown
squarkdown --assets
squarkdown --fonts
squarkdown --quiet    # Suppress ‘info’-level messages (grey)
squarkdown --dry-run  # Run Squarkdown without actually _writing_ any files.
```

This runs the entire squarkup pipeline, (optionally) with assets and fonts preprocessing.

You can run this from any directory in your project, since it searches parent directories for your [squarkup config](squarkup-config.md). If you don’t have one, Squarkdown won’t be able to find your project root, and this command won’t work.

> [!Tip]
> Since assets are unlikely to change very often, you may want to leave out `--assets` for local development builds. I usually configure 2 separate npm scripts:
>
> ```json
> "scripts": {
>    "prep": "squarkdown",
>    "prepx": "squarkdown --assets --fonts"
> }
> ```
>
> Then local development uses `prep` while production builds use `prepx`.


<br>


## Clean

```bash
squarkdown clean
squarkdown clean --force    # Run even with no Git repository found
squarkdown clean --dry-run  # Do not actually delete any files
```

This (safely) deletes all of Squarkdown’s generated output. Currently, this is `+page.svx` and `+page.ts` files – support for assets, site data, etc. is coming.

This requires a Git repository to run, since it only deletes uncommitted and `.gitignore`d files. If you want to bypass this requirement, pass `--force`.

> [!Tip]
> If you want to check the safety of this, use `squarkdown clean --dry-run --force` to view which files would be deleted.

This command always checks before every delete that the file it’s about to delete is below your project root, so that it can never touch files outside your project.[^delete-folder]

[^delete-folder]: Why not just delete the whole folder for Squarkdown output? Well, what if you wanted to put committed files in that folder too? Then Squarkdown would be deleting your committed files, we don’t want.


<br>
