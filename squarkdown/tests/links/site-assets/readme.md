# Main
<!-- #SQUARK live!
| dest = .
-->

These image links should be rewritten to `/link.jpg`.

![one](.assets/link.jpg)

![two](./.assets/link.jpg)


These image links should be rewritten to `/linked.jpg` without `.site/` since they're site assets.

![three](.assets/.site/linked.jpg)

![four](./.assets/.site/linked.jpg)


These image links should be rewritten to `/nested/linking.jpg` without `.site/` since they're site assets.

![five](.assets/.site/nested/linking.jpg)

![six](./.assets/.site/nested/linking.jpg)
