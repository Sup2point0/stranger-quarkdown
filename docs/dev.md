# Engineering
<!-- #SQUARK live! dead!
| dest = TODO
| desc = Technicalities, challenges and decisions behind developing Stranger Quarkdown
| date = 2026 October 10
| update = 2026 October 10
-->

sup!

Here I’ll talk about some of the technicalities, challenges and decisions behind developing Squarkdown. I’ll touch on how things were done previously in the Ruby version, and how I changed them with the Rust rewrite.

If you’re interested in understanding how Squarkdown works, this is *not* the place to start! Instead, I would recommend reading the actual code, starting from [`squarkdown/lib.rs](../squarkdown/src/lib.rs). I’ve got plenty of documentation and comments explaining how everything works. You’ll also find quite a few comments noting down technical challenges and justifying strange design decisions.

If those don’t satisfy you, then this page may be of interest, as an elaboration upon those challenges, providing wider context and reasoning behind them. I haven’t paid too much attention to structure or narrative here, since it’s all just little tidbits here and there for other curious developers to enjoy.


<br>


## Introduction

Squarkdown is an executable build tool, so it has a linear pipeline that performs a series of actions. These are:

- Read the user’s config
- Find `.md` files
- Parse charm squarks
- Render


<br>


## Aggregating Errors


<br>


## Parse then render... but that means duplicate work?

In the Ruby version, parsing the charm squarkdown and rendering Markdown happened at the same time.

With the Rust rewrite, I changed this to first parsing every file, then rendering every file. Theoretically this is worse because we repeat work reading from each file twice, not once.

However, this was a necessary evil if we wanted link rewriting to be possible. Squarkdown necessarily *needs* a complete picture of every page in the site in order to resolve links properly.

It’s basically like the Assembler dilemma: 2-pass to scan for labels first, or 1-pass and fill in labels as you find them? The latter is technically more performant but a nightmare to implement.


<br>


## Why is loading a config such a nightmare?

In the Ruby version, Squarkdown loaded the user’s `squarkup.json` with a simple JSON parse and JSON Schema validate. It was dead easy.

With the Rust rewrite, loading the config now takes a full file with a 200+ line method `from_toml()` and numerous helper functions. The code is somewhat horrific. Why? Why not just serde it?

Many reasons. Some fields (especially the paths) *depend* on other fields, so they need to be deserialised in a particular order.

Also, I wanted better error messages. A huge focus of the Rust rewrite *was* to renovate Squarkdown with really good error messages. Configuration is probably one of the most annoying aspects of software engineering, and it’s very easy to get frustrated when you just can’t figure out how to configure things properly. So these were some of the highest-payoff errors to get right. But serde produces generic type errors that only minimally help the user. And it also bails at the first error.

So, it looked like the only alternative was to manually load and validate the config myself 😎. Which yes, means writing out the logic for every individual field. Which is long, for sure! But it means an incredible amount of control over how we load them, how we transform them, how they interact, how we validate them, all the works.

Most of the lines come from the error messages text, since each error comes with a message, hint, and debug information. Those really add up, so the code becomes very bulky. But it’s worth it.
