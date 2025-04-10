# Configuring Squarkup for a Repository
<!-- #SQUARK dead!
| dest = docs/squarkup/config
| desc =
-->


<br>


## Options

> [!Note]
> This table is auto-generated from the exact specification as laid out by the [JSON schema](../squarkdown/resources/squarkup-schema.json).

| Option | Type | Values | Description | Notes |
| :----- | :--- | :----- | :---------- | :---- |
<!-- #SQUARK inject? -->
<!-- #SQUARK inject. -->


<br>


## Example

Feel free to copy this as a starting template!

```json
{
  "$schema": "https://sup2point0.github.io/stranger-quarkdown/squarkup-schema.json",

  "repo": "Stranger Quarkdown",
  "site": "site",
  "exclude": [
    "stranger-quarkdown/README.md"
  ],

  "fonts": [
    "Fira+Sans"
  ],

  "if-no-dir": ["warn", "create"]
}
```
