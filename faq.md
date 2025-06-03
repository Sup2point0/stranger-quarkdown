# FAQ
<!-- #SQUARK live!
| dest = info/faq
| head = Frequently Asked Questions
-->

### Why does Squarkdown only work with SvelteKit and MDsveX?
Yeah, Squarkdown was made for a very particular purpose. It’s totally geared towards the technologies I use, but that’s purely because I made it for myself :P

It’s happened to be perfect for many of my projects, such as [pyco:bytes](https://sup2point0.github.io/pycobytes), [Integrity](https://sup2point0.github.io/integrity), and ofc [*Assort*](https://sup2point0.github.io/Assort). It really does make development so much easier when I don’t have to worry about where I put everything. After having all the infrastructure set up, being able to just add comments to a `.md` file and have it automatically render to a webpage complete with metadata is pretty awesome. The best part is, you can’t even see any of it when previewing the Markdown file!

### Where does the name *Squarkdown* come from?
*Squarkdown* is an abbreviation of the full name *StrangerQuarkdown*. It’s the successor to my original tool [*Quarkdown*](https://github.com/Sup2point0/quarkdown) (now deprecated). You can find out more in [Synopsis](synopsis.md)!

### How fast is Squarkdown?
Eh, decently? It really depends how many files you have. The more Squarkdown has to search through and parse, the longer squarkup will take. This is why [setting your sources properly](~) is important!

For my personal wiki [Assort](https://github.com/Sup2point0/Assort), currently my largest project that uses Squarkdown, it ends up scanning 350+ files and 600+ assets during squarkup,[^lot] taking around 3 seconds. Execution time varies a lot depending on system speed and delays.

[^lot]: It’s unlikely you’ll ever have this much in a GitHub repo unless you’re tryna host a wiki too, lmao.

### Why Ruby?
It’s a cute language :3 I needed an excuse (or rather, a project) to use it, so I decided I’d give writing Squarkdown in Ruby. It went pretty swimmingly, to be honest, almost zero hitches. Ruby just works ^v^

### Was this website built with Squarkdown?
Of course! (Imagine if it weren’t!) The text you’re reading now is sourced from a `.md` file [in the Squarkdown repo](https://github.com/Sup2point0/stranger-quarkdown/blob/main/faq.md?plain=1) ;)

More details are available in [Synopsis](synopsis.md).
