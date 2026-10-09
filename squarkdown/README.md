# Stranger Quarkdown

[Docs](https://github.com/Sup2point0/stranger-quarkdown/tree/main/docs)&ensp;·&ensp;[Quickstart](https://github.com/Sup2point0/stranger-quarkdown/blob/main/docs/walkthrough/quickstart.md)&ensp;·&ensp;[FAQ](https://github.com/Sup2point0/stranger-quarkdown/blob/main/FAQ.md)&ensp;·&ensp;[Changelog](https://github.com/Sup2point0/stranger-quarkdown/blob/main/CHANGELOG.md)&ensp;·&ensp;[Site](https://sup2point0.github.io/stranger-quarkdown/docs)

Markdown preprocessing automation for Svelte/Kit projects.

Please visit [the GitHub](https://github.com/Sup2point0/stranger-quarkdown#readme) for the full README :]


<br>


## Quickstart

Run:

```bash
> squarkdown
```

With extras:

```bash
> squarkdown --assets --fonts
```

Remind yourself:

```bash
> squarkdown --help
```


<br>


## Squarkdown-Flavoured Markdown

```md
# Welcome to Squarkdown!
<!-- #SQUARK live!
| dest = walkthrough/welcome
| capt = An introduction to what Squarkdown can do
| date = 2024 July 1
-->

This link to [other-file.md](other-file.md) will have its `.md` extension stripped.

<!-- #SQUARK slash? -->
This content won’t be included in the output.
<!-- #SQUARK slash. -->

<!-- #SQUARK only?

This content only shows up in the output.

     #SQUARK only. -->
```
