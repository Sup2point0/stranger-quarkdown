# Hacking the Mainframe
<!-- #SQUARK live!
| dest = info/synopsis
| title = Synopsis
| update = 2026 October 31
-->

*Stranger Quarkdown* is a successor to *Quarkdown*.

I say ‘a’ successor, because firstly it wasn’t indended to replace Quarkdown, but rather satisfy a different purpose – integrating with SvelteKit, rather than generating raw static websites – and secondly, there ofc might always be other child projects!

I affectionately call it *Squarkdown*. Weird name, lol – and ‘squark’ sounds like ‘squawk’, a bird. I find that kinda funny tho. ‘Stranger’ ofc comes from the [strange quark](https://wikipedia.org/wiki/Strange_quark).

> [!Important]
> Upon searching up the Wikipedia link for ‘strange quark’ I have immediately discovered that a **squark** is a real thing. As in, scientists have used it to name something.
>
> This is news to me, and it’s absolutely hilarious. I love this. I love this so much.
>
> I’m a genius.
> 
> (\s)


<br>


## Source

Stranger Quarkdown v4.0 and beyond is written in Rust. For reasons why, see [FAQ § Why the Rust rewrite?](docs/faq.md#why-the-rust-rewrite)

> Since this was called *stranger* Quarkdown, and I happened to be falling in love with Ruby at the time, I decided to give a shot at writing Squarkdown with it instead of Python. Might as well flex the ol’ programming muscles.
> 
> Turns out complex string manipulation and filepath handling in Ruby is just as straightforward in Python, with the same predictable methods and a wonderful `Pathname` class. No regrets here!


<br>


## Site

What would this project even be if it didn’t use itself to build its own website?!

The [Squarkdown site](https://sup2point0.github.io/stranger-quarkdown) is built with [Svelte<sup>↗</sup>](https://svelte.dev)/[Kit<sup>↗</sup>](https://svelte.dev/docs/kit), using [MDsveX<sup>↗</sup>](https://mdsvex.pngwn.io) for Markdown integration, Squarkdown for preprocessing, and some [NuShell<sup>↗</sup>](https://nushell.sh) scripts for automation of stuff like syncing the JSON Schemas.


<br>


## Publishing

Squarkdown is published to:

- [crates.io<sup>↗</sup>](https://crates.io/crates/squarkdown) as `squarkdown`
- [npm<sup>↗</sup>](https://www.npmjs.com/package/stranger-quarkdown) as `stranger-quarkdown`

Since we obviously want publishing between these registries to be in sync, publishing is all done via a `publish.nu` script (which is very specialised to my local machine and workflow!) that runs tests, syncs the versions, and publishes to both registries.


<br>


## Assets

Assets were lovingly crafted in PowerPoint 2016 (offline), as usual =)


<br>
