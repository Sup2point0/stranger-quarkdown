# Squarkup Schemas
<!-- #SQUARK live!
| dest = schemas
| desc = JSON Schemas for Configuring Stranger Quarkdown
-->

<div class="quicklinks" align="center">

[Latest](https://sup2point0.github.io/stranger-quarkdown/schemas/latest.json)

</div>

Stranger Quarkdown provides a [JSON Schema<sup>↗</sup>](https://json-schema.org) to validate your [squarkup.toml](../../docs/reference/squarkup-toml.md)[^toml] or squarkup.json.

[^toml]: JSON Schema can validate TOML too!

To use it, link a schema to your config:

```toml
#:schema https://sup2point0.github.io/stranger-quarkdown/schemas/latest.json

[project]
name = "Schemafied!"
# ...
```

This will enable error-checking, autocomplete, and helpful hover tooltips in IDEs.


## Schemas

The [latest](https://sup2point0.github.io/stranger-quarkdown/schemas/latest.json) version of the schema is always up-to-date with the latest version of Squarkdown.

If you’d like to pin your schema to your particular version, you can access past versions of the schema below. The schema’s version is synced with Squarkdown, so e.g. `v4.4` of the schema is compatible with Squarkdown `v4.4`.

### Supported
<!-- #SQUARK leave? -->
- [v4.7](4.7.json) (latest)
- [v4.4](4.4.json)
<!-- #SQUARK leave. -->

### Archived
Pre-Squarkdown v4.0, the schema had its own versioning separate from Squarkdown.[^version] This ended up being too confusing and pointless, so post-v4.0 the schema is released alongside Squarkdown even if no changes were made.

[^version]: Stranger Quarkdown is a fairly old project of mine at this point – back then I was much less disciplined with versions :P

<!-- #SQUARK leave? -->
- [v5.0.10](../squarkup-schema/v5.0.10.json)
- [v5.0.9](../squarkup-schema/v5.0.9.json)
- [v5.0.7](../squarkup-schema/v5.0.7.json)
- [v5.0.6](../squarkup-schema/v5.0.6.json)
- [v5.0.5](../squarkup-schema/v5.0.5.json)
- [v5.0.4](../squarkup-schema/v5.0.4.json)
- [v5.0.1](../squarkup-schema/v5.0.1.json)
- [v5.0.0](../squarkup-schema/v5.0.0.json)
- [v4.2](../squarkup-schema/v4.2.json)
- [v4.0](../squarkup-schema/v4.0.json)
- [v3.6](../squarkup-schema/v3.6.json)
- [v3.5](../squarkup-schema/v3.5.json)
- [v3.3](../squarkup-schema/v3.3.json)
- [v3.2](../squarkup-schema/v3.2.json)
- [v3.1](../squarkup-schema/v3.1.json)
- [v3.0](../squarkup-schema/v3.0.json)
- [v2.2](../squarkup-schema/v2.2.json)
- [v2.1](../squarkup-schema/v2.1.json)
<!-- #SQUARK leave. -->
