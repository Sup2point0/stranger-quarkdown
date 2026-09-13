use super::*;
use crate::{
	SquarkupConfig, PageData, SquarkResult, SquarkError,
	log,
	colours::*,
	macros::*,
};

use pulldown_cmark as pulldown;
use pulldown_cmark_to_cmark as cmark;

use std::fs::{ self, File };
use std::io::{ Read, Write };


pub struct Renderer
{
	/// The parsing context stack.
	pub(super) ctx: ContextStack,

	/// Accumulated errors during rendering.
	pub(super) errors: Vec<SquarkError>,
}

impl Renderer
{
	/// Construct a renderer for rendering from `source` to `target`.
	pub fn new() -> Self
	{
		Self {
			ctx: ContextStack::new(),
			errors: vec![],
		}
	}

	pub fn render(&mut self,
		page: &PageData,
		config: &SquarkupConfig,
	) -> SquarkResult
	{
		let dest = config.out.folder.join(&page.destination).join(&config.out.file);

		log::info!(slash!(
			"rendering to: {GREY1}{}",
			dest.strip_prefix(&config.paths.root).unwrap().to_path_buf(),
		));

		let mut file = File::open(&page.filepath)?;

		let mut source = str!();
		file.read_to_string(&mut source)?;

		if let Some(folder) = dest.parent() {
			if !folder.exists() {
				fs::create_dir_all(folder)?;
			}
		}

		let output = self.render_from(&source, page, config)?;

		let mut target = File::create(dest)?;
		target.write_all(output.as_bytes())?;

		Ok(())
	}

	pub(super) fn render_from(&mut self,
		source: &str,
		page: &PageData,
		config: &SquarkupConfig,
	) -> SquarkResult<String>
	{
		let parser =
			pulldown::Parser::new(&source)
				.map(|e| match e {
					_ => e,
				})
		;

		let mut out = str!();

		cmark::cmark(parser.inspect(|e| { dbg!(e); }), &mut out).unwrap();

		Ok(out)
	}
}


#[cfg(test)]
use indoc::indoc;


#[cfg(test)]
mod plain {
	use super::*;

	#[test] fn easy() {
		test_preserves(&[
			"sup, world!",
			"sup,\nworld!",
		]);
	}

	#[test] fn medium() {
		test_preserves(&[
			"sup,\nworld!\n",
			"sup, \nworld!\n",
			"sup,\n world!\n",
			"sup, \n world!\n",
		]);
	}
}

#[cfg(test)]
mod code_inline {
	use super::*;

	#[test] fn easy() {
		test_expected(&[
			pair!("this `is` code"),
			pair!("this `is ` code"),
			pair!("this ` is` code"),
				  ("this ` is ` code", "this `is` code"),
			pair!("this `is` some `more` code"),
			pair!("`1` onto\nline `2`"),
			pair!("line `1` onto\nline `2`."),
		]);
	}

	#[test] fn medium() {
		test_expected(&[
			pair!("`x` `y`"),
				  ("` 1 ` ` 2 `", "`1` `2`"),
			pair!("`x y` `z`"),
			pair!("`x`y`z`"),
		]);
	}

	#[test] fn unclosed() {
		test_expected(&[
			("`1\n2", "\\`1\n2"),
		]);
	}

	#[test] fn edge_cases() {
		test_preserves(&[
			"`x`",
			"`code`",
		])
	}
}

#[cfg(test)]
mod code_blocks {
	use super::*;

	#[test] fn easy() {
		test_preserves(&[
			indoc! {"
				This is some code

				```
				print(\"hello world\")
				```
			"},
		])
	}

	#[test] fn medium() {
		test_preserves(&[
			indoc! {"
				```md
				<!-- #SQUARK slash? -->
				sup
				<!-- #SQUARK slash. -->
				```
			"},
		])
	}

	#[test] fn hard() {
		test_preserves(&[
			indoc! {"
				```md
				<!-- #SQUARK slash? -->
				```

				sup

				```py
				sup
				```

				```
				<!-- #SQUARK slash. -->
				```
			"},
		])
	}

	#[test] fn escaped() {
		test_preserves(&[
			indoc! {"
				```md
				\\```math
				y = x
				\\```
				```
			"},
		])
	}

	#[test] fn edge_cases() {
		test_preserves(&[
			"```code```",
			"```code\n```",
			"``````",
			"``` ```",
			"```\n```",
		])
	}
}

#[cfg(test)]
mod comments {
	use super::*;
	
	mod erases {
		use super::*;

		#[test] fn easy() {
			test_expected(&[
				("erase <!--this--> comment",   "erase  comment"),
				("erase <!--this --> comment",  "erase  comment"),
				("erase <!-- this--> comment",  "erase  comment"),
				("erase <!-- this --> comment", "erase  comment"),
			]);
			test_expected(&[
				("erase <!--this comment--> please",   "erase  please"),
				("erase <!--this comment --> please",  "erase  please"),
				("erase <!-- this comment--> please",  "erase  please"),
				("erase <!-- this comment --> please", "erase  please"),
			]);
			test_expected(&[
				("erase\n<!-- this comment -->\nplease",    "erase\n\nplease"),
				("erase\n<!--\nthis comment\n-->\nplease",  "erase\n\nplease"),
				("erase\n<!--\nthis\ncomment\n-->\nplease", "erase\n\nplease"),
			]);
		}

		#[test] fn nested() {
			test_expected(&[
				("<!-- <!-- illegal --> comment", " comment"),
			]);
		}
	}

	mod preserves {
		use super::*;

		#[test] fn easy() {
			test_preserves_with_comments(&[
				"keep <!--this--> comment",
				"keep <!--this --> comment",
				"keep <!-- this--> comment",
				"keep <!-- this --> comment",
			]);
			test_preserves_with_comments(&[
				"keep <!--this comment--> please",
				"keep <!--this comment --> please",
				"keep <!-- this comment--> please",
				"keep <!-- this comment --> please",
			]);
			test_preserves_with_comments(&[
				"keep\n<!-- this comment -->\nplease",
				"keep\n<!--\nthis comment\n-->\nplease",
				"keep\n<!--\nthis\ncomment\n-->\nplease",
			]);
		}

		#[test] fn nested() {
			test_preserves_with_comments(&[
				"<!-- <!-- illegal --> comment",
			]);
		}
	}
}

#[cfg(test)]
mod slash {
	use super::*;

	#[test] fn one_line() {
		test_expected(&[
			(
				"erase <!-- #SQUARK slash? --> this <!-- #SQUARK slash. --> please",
				"erase  please",
			),
		]);
	}

	#[test] fn multi_line() {
		test_expected(&[
			(
				indoc! {"
					erase
					<!-- #SQUARK slash? -->
					this
					<!-- #SQUARK slash. -->
					please
				"},
				indoc! {"
					erase

					please
				"},
			),
		]);
	}
}

#[cfg(test)]
mod leave {
	use super::*;

	#[test] fn easy() {
		test_expected(&[
			(
				"Don't <!-- #SQUARK leave? --> do <!-- #SQUARK leave. --> anything",
				"Don't  do  anything",
			),
		]);
	}

	#[test] fn standard() {
		test_expected(&[
			(
				indoc! {"
					Don't
					<!-- #SQUARK leave? -->
					<!-- #SQUARK slash? --> touch <!-- #SQUARK slash. -->
					<!-- #SQUARK leave. -->
					this
				"},
				indoc! {"
					Don't

					<!-- #SQUARK slash? --> touch <!-- #SQUARK slash. -->

					this
				"}
			),
		]);
	}

	#[test] fn nested() {
		test_expected(&[
			(
				indoc! {"
					1
					<!-- #SQUARK leave? -->
					<!-- #SQUARK leave? -->
					2
					<!-- #SQUARK leave. -->
					<!-- #SQUARK leave. -->
					3
				"},
				indoc! {"
					1

					<!-- #SQUARK leave? -->
					2
					<!-- #SQUARK leave. -->

					3
				"},
			),
		])
	}
}

#[cfg(test)]
mod only {
	use super::*;

	#[test] fn easy() {
		test_expected(&[
			("Please <!-- #SQUARK only? show #SQUARK only. --> me", "Please show me"),
		]);
	}

	#[test] fn medium() {
		test_expected(&[
			(
				indoc! {"
					Please

					<!-- #SQUARK only?

					show me!

							#SQUARK only. -->
				"},
				indoc! {"
					Please

					show me!
				"}
			),
		]);
	}

	#[test] fn awkward_whitespace() {
		test_expected(&[
			("x <!-- #SQUARK only? y #SQUARK only. --> z",  "x y z"),
			("x <!-- #SQUARK only?  y #SQUARK only. --> z", "x y z"),
			("x <!-- #SQUARK only? y #SQUARK only. -->  z", "x y  z"),
		]);
	}
}
