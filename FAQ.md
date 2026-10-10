# FAQ
<!-- #SQUARK live!
| dest = info/faq
| head = Frequently Asked Questions
| update = 2026 October 10
-->

### What is Squarkdown?
See [What is Squarkdown?](docs/walkthrough/what-is-squarkdown.md).

### What is Squarkdown-flavoured Markdown?
See [Squarkdown-flavoured Markdown](docs/walkthrough/squarkdown-flavoured-markdown.md).

### Can I use Squarkdown for my project?
See [Project Requirements](docs/walkthrough/project-requirements.md).

### Why does Squarkdown only work with SvelteKit and MDsveX?
Yeah so, Squarkdown was made for a very particular purpose in a very particular tech stack. It’s totally geared towards my use cases, since I made it primarily for myself :P

It’s happened to be perfect for many of my projects, such as [pyco:bytes](https://sup2point0.github.io/pycobytes), [Integrity](https://sup2point0.github.io/integrity), and ofc [*Assort*](https://sup2point0.github.io/Assort). It really does make development so much easier when I don’t have to worry about where I put everything. After having all the infrastructure set up, being able to just add comments to a `.md` file and have it automatically render to a webpage complete with metadata is pretty awesome. The best part is, you can’t even see any of it when previewing the Markdown file!

### How fast is Squarkdown?
With the Rust rewrite, fast enough. On my Windows machine the Squarkdown docs take under 100 ms to squarkup, but on GitHub Action’s Linux machine the whole thing finishes in under 10 ms. Execution time varies a lot depending on system speed and delays. Run it yourself and see, right?

### What is Squarkdown spending its time doing?
Squarkdown spends most of its time doing 3 things:

- Finding `.md` files to squarkup
- Parsing their [charm](docs/glossary.md#charm-squark) squarks for metadata
- Rendering each page to `+page.svx` and `+page.ts`

The last part takes the longest because Squarkdown is reading, parsing, processing and rendering your entire file. So the longer the file, the more work it has to do.

The first two parts are pretty fast, and Squarkdown avoids unnecessary work by skipping ignored directories and only reading files up to the end of their charm squark.

### Where does the name *Squarkdown* come from?
*Squarkdown* is an abbreviation of the full name *Stranger Quarkdown*. It’s the successor to my original tool [*Quarkdown*](https://github.com/Sup2point0/quarkdown) (now deprecated). You can find out more in [Synopsis](synopsis.md)!

### Why Ruby (originally)?
It’s a cute language :3

I needed an excuse (or rather, a project) to use it, so I decided I’d give writing Squarkdown in Ruby a shot. It went pretty swimmingly, to be honest, almost zero hitches. Ruby just works ^v^

### Why the Rust rewrite?
Unfortunately, as much as I loved Ruby, it... was just getting unmanageable. I love its syntax, but the lack of type annotations just made things incredibly painful. The whole system felt very fragile.

Why Rust, then? Well, one of my aims was to make Squarkdown much faster. So Python was out the window. I also wanted to make it easily distributable,[^dist] so TypeScript/JavaScript was the clear choice (especially since the tool is for SvelteKit projects, which themselves use TypeScript/JavaScript).

But TypeScript’s boringgg. I don’t want to write another TypeScript project. So Rust was the next-best option ;)

[^dist]: The previous method of using a Git submodule, while easy, was a little scuffed for versioning. Basically meant everyone would be using a nightly version of Squarkdown!!

Well, what can I say. It works great, feels amazing to build upon, and the test suite is blooming. So no regrets at all!

<!-- #SQUARK only?

### Was this website built with Squarkdown?
Of course! (Imagine if it weren’t!) The text you’re reading now is sourced from a `.md` file [in the Squarkdown repo](https://github.com/Sup2point0/stranger-quarkdown/blob/main/faq.md?plain=1) ;)

More details are available in [Synopsis](synopsis.md).

     #SQUARK only. -->


<br>
