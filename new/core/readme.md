# Dev Notes

## Explaining Weird Conventions

- Unit tests aren't in `#[cfg(test)] mod test`, but rooted.
  - This avoids an extra layer of indentation for `test` in the IDE's Test Explorer UI. Small wins!
