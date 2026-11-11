# Repairing HTML
<!-- #SQUARK live!
| dest = features/repair
| date = 2026 November 4
| update = 2026 November 5
-->

Squarkdown will attempt to repair broken HTML that it finds while rendering.

Currently, this includes:

- [Repairing tables](#repairing-tables) by inserting missing `<tbody>`.


<br>


## Repairing Tables

*Opt in by setting [`format.repair-tables = true`](../reference/squarkup-config.md#repair-tables)*.

This is a common shorthand when handwriting HTML tables:

```html
<table>
   <tr>
      ...
   </tr>
</table>
```

However, it’s technically invalid – it should really be inside a `<tbody>` element:

```diff
  <table>
+    <tbody>
        <tr>
           ...
        </tr>
+    </tbody>
  </table>
```

As of SvelteKit 3 (I believe) the former is now rejected, so the latter is strictly necessary. If you still want to use the former (like me :P), Squarkdown will automatically insert the missing `<tbody>` for you.
