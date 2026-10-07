//! Here you’ll find the integration tests for Squarkdown.
//! 
//! They’re organised into folders following `<feature>/<type>`, where `<feature>` is some aspect of Squarkdown and `<type>` is a particular capability, setting, version, etc. of that feature.
//! 
//! Each `<feature>/` folder has a `mod.rs`, containing one test function for each of the `<type>/` folders.
//! 
//! Each `<type>/` folder represents a small independent project using Squarkdown, with their own `squarkup.toml` and a series of `.md` files to squarkup.
//! 
//! `test-project/` contains a SvelteKit site shared between all of the integration tests. When the rests run Squarkdown, they will output to their own independent folders under `src/routes/`.
//! 
//! The tests invoke the Squarkdown binary under their `<feature>/<type>/` folder, then check the outputs under `src/routes/<feature>/<type>/`.
//! 
//! Afterwards, we also use Playwright to check all the links in the site. This makes sure all pages were rendered to the correct place, Markdown was rendered correctly, and that links were rewritten correctly.


// == CONFIG == //

mod errors;


// == RESOLVE == //

mod sources;


// == RENDER == //

mod render;

mod fields;

mod links;


// == EXTRAS == //

mod data;

mod assets;

mod fonts;


mod utils;
pub use utils::*;
