use super::*;
use crate::{
	SquarkupConfig, PageData, SquarkResult, SquarkError,
	log,
	colours::*,
	macros::*,
};

use kiam::when;
use lazy_static::lazy_static;
use pulldown_cmark as pd;
use pulldown_cmark_to_cmark as cmark;
use regex::Regex;

use std::fs::{ self, File };
use std::io::{ Read, Write };


lazy_static!
{
	/// Options for parsing with `pulldown_cmark`.
	pub static ref PARSER_OPTIONS: pd::Options
		= pd::Options::from(
			  pd::Options::ENABLE_GFM
			| pd::Options::ENABLE_TABLES
			| pd::Options::ENABLE_FOOTNOTES
		);

	/// Options for rendering with `pulldown_cmark_to_cmark`.
	pub static ref RENDER_OPTIONS: cmark::Options<'static>
		= cmark::Options {
			code_block_token_count: 3,
			list_token: '-',
			..cmark::Options::default()
		};

	/// The RegEx pattern for twin squarks.
	/// 
	/// - Group 1 is the squark (`leave`, `slash`)
	/// - Group 2 is either `?` (open) or `.` (close).
	/// - Group 3, if present, is an alphanumeric identifier for the section.
	pub static ref TWIN_SQUARK: Regex
		= Regex::new(
			r"#(?:squark|SQUARK)\s+([a-zA-Z]+)(\?|\.)(?:\s+\[(\w+)\])?"
		).unwrap();
}


pub struct Renderer
{
	/// The parsing context stack.
	pub(super) ctx: ContextStack,

	/// Accumulated errors during rendering.
	errors: Vec<SquarkError>,
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

		let output = self.render_from(source, page, config);

		let mut target = File::create(dest)?;
		target.write_all(output.as_bytes())?;

		if self.errors.is_empty() {
			Ok(())
		} else {
			Err(SquarkError::Multiple { errs: self.errors.drain(..).collect() })
		}
	}

	pub(super) fn render_from(&mut self,
		mut source: String,
		page: &PageData,
		config: &SquarkupConfig,
	) -> String
	{
		source = Self::expand_only(source);

		// TODO maybe `flat_map` to support context-tracking `only`?
		let parser =
			pd::Parser::new_ext(&source, PARSER_OPTIONS.clone())
				.inspect(|e| { dbg!(e); })
				.filter_map(|e| self.process_event(e, page, config))
		;

		let mut out = str!();
		cmark::cmark_with_options(parser, &mut out, RENDER_OPTIONS.clone()).unwrap();

		out
	}

	fn expand_only(source: String) -> String
	{
		Regex::new(
			r"(?is)<!--\s*#SQUARK\s+ONLY\?\s+(?-i)(.*?)(?i)#SQUARK\s+ONLY\.\s*-->"
		).unwrap()
		.replace_all(&source, "$1")
		.to_string()
	}
}

impl Renderer
{
	fn process_event<'e>(&mut self,
		event: pd::Event<'e>,
		page: &PageData,
		config: &SquarkupConfig,
	) -> Option<pd::Event<'e>>
	{
		match event {
			pd::Event::Start(pd::Tag::CodeBlock(..)) => { self.ctx.push(Ctx::CODE); }
			pd::Event::End(pd::TagEnd::CodeBlock) => { self.ctx.try_pop(Ctx::CODE); }
			_ => (),
		};

		match event {
			| pd::Event::Html(ref html)
			| pd::Event::InlineHtml(ref html)
			=> {
				let html = html.trim();
				
				if html.starts_with("<!--") && html.ends_with("-->") {
					return when! {
						self.process_comment(html) || self.ctx.is_slash()      => None,
						config.format.preserve_comments || self.ctx.is_leave() => Some(event),
						_ => None,
					}
				}
				else if !self.ctx.is_slash() {
					if html.starts_with("<!--") {
						self.ctx.push(Ctx::COMMENT);
						return config.format.preserve_comments.then_some(event)
					}
					else if html.ends_with("-->") {
						self.ctx.force_pop(Ctx::COMMENT);
						return config.format.preserve_comments.then_some(event)
					}
				}
				self.process_markdown(event, config)
			}
			_ => self.process_markdown(event, config),
		}
	}

	fn process_markdown<'e>(&mut self,
		event: pd::Event<'e>,
		config: &SquarkupConfig,
	) -> Option<pd::Event<'e>>
	{
		dbg!(self.ctx.stack());

		match self.ctx.current()
		{
			Ctx::COMMENT => config.format.preserve_comments.then_some(event),
			Ctx::SLASH{..} => None,
			_ => Some(event),
		}
	}

	/// Attempt to process squarks inside `html`, returning `true` if a squark was matched (and so the comment should be removed).
	fn process_comment(&mut self,
		html: &str,
	) -> bool
	{
		if let Some(captures) = TWIN_SQUARK.captures(html) {
			let key = captures.get(3).map(|k| k.as_str().to_owned());

			let squark = match captures.get(1) {
				Some(m) => match m.as_str().to_ascii_uppercase().as_str() {
					"LEAVE" => Ctx::LEAVE { key },
					"SLASH" => Ctx::SLASH { key },

					s => {
						self.errors.push(SquarkError::Recoverable {
							msg: fmt!("unknown twin squark: {W}{s}"),
							hint: fmt!("valid twin squarks are {W}leave{G}, {W}slash{G}, {W}only"),
							debug: vec![
								fmt!("context stack: {:?}", self.ctx.stack())
							],
						});
						return false;
					}
				}
				None => unreachable!(),
			};

			if matches!(self.ctx.current(), Ctx::LEAVE{..})
			&& !matches!(squark, Ctx::LEAVE{..})
			{
				return false;
			}

			match captures.get(2) {
				Some(m) => match m.as_str() {
					"?" => self.ctx.push(squark),
					"." => {
						let did_pop = self.ctx.force_pop(squark);

						if !did_pop {
							self.errors.push(SquarkError::Recoverable {
								msg: fmt!("unpaired closing squark: {W}{html}"),
								hint: fmt!(""),
								debug: vec![
									fmt!("context stack: {:?}", self.ctx)
								],
							});
						}
					}
					_ => unreachable!(),
				}
				None => unreachable!(),
			}

			return true;
		}
		else if html.contains("#squark") || html.contains("#SQUARK") {
			self.errors.push(SquarkError::Recoverable {
				msg: fmt!("unknown squark pattern: {W}{html}"),
				hint: fmt!("use squarks like this: {W}<!-- #SQUARK leave? -->"),
				debug: vec![],
			});
		}

		false
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
		test_expect(&[
			"# Heading\nThe quick brown fox jumps over the lazy dog",
			"# Heading\n\nThe quick brown fox jumps over the lazy dog",
			"# Heading\n\n\nThe quick brown fox jumps over the lazy dog",
		], "# Heading\n\nThe quick brown fox jumps over the lazy dog");
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
		test_expected(&[
			("```code```",   "`code`"),
			("```code\n```", "```code\n```"),
			("``````",       "```\n```"),
			("``` ```",      "` `"),
			("```\n```",     "```\n```"),
		])
	}
}

#[cfg(test)]
mod tables {
	use super::*;

	#[test] fn easy() {
		test_preserves(&[
			indoc! {"
				|one|two|
				|---|---|
				|1|2|
				|3|4|
			"},
			indoc! {"
				|one|two|
				|:--|:--|
				|1|2|
				|3|4|
			"},
			indoc! {"
				|one|two|
				|--:|--:|
				|1|2|
				|3|4|
			"},
			indoc! {"
				|one|two|
				|:-:|:-:|
				|1|2|
				|3|4|
			"},
		]);
	}
}

#[cfg(test)]
mod comments {
	use super::*;
	
	mod erases {
		use super::*;

		#[test] fn easy() {
			test_expect(&[
				"erase <!--this--> this",
				"erase <!--this --> this",
				"erase <!-- this--> this",
				"erase <!-- this --> this",
			], "erase  this");
		}

		#[test] fn medium() {
			test_expect(&[
				"erase <!--this comment--> please",
				"erase <!--this comment --> please",
				"erase <!-- this comment--> please",
				"erase <!-- this comment --> please",
			], "erase  please");
		}

		#[test] fn hard() {
			test_expect(&[
				"erase\n<!-- this comment -->\nplease",
				"erase\n<!--\nthis comment\n-->\nplease",
				"erase\n<!--\nthis\ncomment\n-->\nplease",
			], "erase\n\n\nplease");
		}

		// FIXME
		#[test] fn nested() {
			test_expected(&[
				("<!-- <!-- illegal --> comment", " comment"),
			]);
		}
	}

	mod preserves {
		use super::*;

		#[test] fn easy() {
			test_expected_for(|c| c.format.preserve_comments = true, &[
				pair!("keep <!--this--> comment"),
				pair!("keep <!--this --> comment"),
				pair!("keep <!-- this--> comment"),
				pair!("keep <!-- this --> comment"),
			]);
		}

		#[test] fn medium() {
			test_expected_for(|c| c.format.preserve_comments = true, &[
				pair!("keep <!--this comment--> please"),
				pair!("keep <!--this comment --> please"),
				pair!("keep <!-- this comment--> please"),
				pair!("keep <!-- this comment --> please"),
			]);
		}

		#[test] fn hard() {
			test_expected_for(|c| c.format.preserve_comments = true, &[
				("keep\n<!-- this one -->\nplease",    "keep\n\n<!-- this one -->\n\nplease"),
				("keep\n<!--\nthis one\n-->\nplease",  "keep\n\n<!--\nthis one\n-->\n\nplease"),
				("keep\n<!--\nthis\none\n-->\nplease", "keep\n\n<!--\nthis\none\n-->\n\nplease"),
			]);
		}

		#[test] fn nested() {
			test_expected_for(|c| c.format.preserve_comments = true, &[
				pair!("<!-- <!-- illegal --> comment"),
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
				"erase\n\n\nplease"
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
			("Please <!-- #SQUARK only? show #SQUARK only. --> me", "Please show  me"),
			("Please <!-- #SQUARK only? do show #SQUARK only. --> me", "Please do show  me"),
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
			("x <!-- #SQUARK only? y #SQUARK only. --> z",  "x y  z"),
			("x <!-- #SQUARK only?  y #SQUARK only. --> z", "x y  z"),
			("x <!-- #SQUARK only? y #SQUARK only. -->  z", "x y   z"),
		]);
	}
}
