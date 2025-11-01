# Rationale
<!-- #SQUARK live!
| dest = info/rationale
| capt = What did I create Squarkdown for?
| update = 2025 November 1
-->

Like any sane developer, I created *Squarkdown* because I needed a tool to do something, but couldn’t find any that could.[^find-tool]

[^find-tool]: Uhh, or maybe I just didn’t look and decided it’d be easier (and cooler) to make my own. :P

The situation which led me to create Squarkdown should actually pretty clearly illustrate what it’s for. (I hope.)

So I store my personal wiki, [Assort](https://sup2point0.github.io/Assort), as a [repo on GitHub](https://github.com/Sup2point0/Assort). It’s filled with hundreds of `.md` files stored in a meshwork of folders. You can browse through them right on GitHub, and GitHub renders a lovely preview with [GitHub-Flavoured Markdown]() for each file. This is certainly my preferred way of reading their content!

At the same time, I also wanted to share the content with non-techy people who probably wouldn’t be accustomed to browsing content through a GitHub repo. In this case, a website would be ideal.

> How and why I chose Svelte/Kit is irrelevant, but it’s my web dev framework of choice.

However, extracting the content from all those Markdown files to the Svelte site was... a little nontrivial, considering they were literally everywhere. Deeply nested folders, stored alongside other files, some hidden files that I didn’t want to be deployed... I wanted a huge degree of control over their processing.

What Squarkdown does is exactly that – it automates the process of collecting all the relevant `.md` files in the repo, processing and extracting their metadata, and generating their equivalent pages in the site. It means I can write content in Markdown as usual, then simply configure a few options in [squarks](walkthrough/squarkdown-flavoured-markdown.md), and have the peace of mind that it’ll all be automatically reflected in the site.
