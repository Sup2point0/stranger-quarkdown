# Main
<!-- #SQUARK live!
| dest = .
-->

These image links should be rewritten to `/link.jpg`.

![one](.assets/link.jpg)

![two](./.assets/link.jpg)


These image links should be rewritten to `/site-1.jpg` without `.site/` since they're site assets.

![three](.assets/.site/site-1.jpg)

![four](./.assets/.site/site-1.jpg)


These image links should be rewritten to `/nested/site-2.jpg` without `.site/` since they're site assets.

![five](.assets/.site/nested/site-2.jpg)

![six](./.assets/.site/nested/site-2.jpg)
