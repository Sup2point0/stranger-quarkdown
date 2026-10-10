# Squarkdown CLI
<!-- #SQUARK live!
| dest = docs/reference/cli
| update = 2026 October 10
-->

Squarkdown is a command-line application with a small, focused set of commands.


<br>


## Help

```bash
squarkdown --help
squarkdown -h
```

This shows all of the Squarkdown commands available.


<br>


## Version

```bash
squarkdown --version
```

This shows which version of Squarkdown you have installed.


<br>


## Squarkup

```bash
squarkdown
squarkdown --assets
squarkdown --fonts
squarkdown --no-parallel
```

This runs the entire squarkup pipeline, (optionally) with assets and fonts preprocessing.

You must have a [squarkup config](squarkup-config.md) file to run this command, otherwise Squarkdown cannot find your project root.

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


## Init (in development)

```bash
squarkdown init
```

This initialises your [`squarkup.toml`](squarkup-toml.md) for you, like `npm init` or `sv create`.

You may find this helpful if you’re new to Squarkdown and are overwhelmed by all the configuration options!
