# Dev Notes

## Explaining Weird Conventions

- `utils` contains both individual items (e.g. `utils::display_rel()`) and submodules (e.g. `utils::testing`)
- Unit tests aren't in `#[cfg(test)] mod test`, but rooted.
  - This avoids an extra layer of indentation for `test` in the IDE's Test Explorer UI. Small wins!
- `std` imports always have `{}`, to allow quick additions of new imports:
  
  ```rs
  use std::collections::{ HashMap };

  // instead of:
  use std::collections::HashMap;
  ```
