use super::*;
use crate::core::*;
use crate::config::*;
use crate::log;
use crate::colours::*;
use crate::macros::*;

use lazy_static::lazy_static;
use pulldown_cmark as pd;
use pulldown_cmark_to_cmark as cmark;
use regex::Regex;

use std::fs::{ self, File };
use std::io::{ Read, Write };
use std::path::{ PathBuf };


// == IMPLEMENTATION == //

lazy_static!
{
	/// Options for parsing with `pulldown-cmark`.
	pub static ref PARSER_OPTIONS: pd::Options
		= pd::Options::from(
			  pd::Options::ENABLE_GFM
			| pd::Options::ENABLE_TABLES
			| pd::Options::ENABLE_FOOTNOTES
		);

	/// Options for rendering with `pulldown-cmark-to-cmark`.
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


/// Render `page` to its `+page.svx` and/or `+page.js` files, applying `site` and `config` accordingly.
pub fn render(
	page: &PageData,
	site: &SiteData,
	config: &SquarkupConfig,
) -> SquarkResult
{
	Renderer::new(page, site, config)._render_()
}


/// Mutable state for tracking rendering context and errors.
pub(super) struct Renderer<'d>
{
	// == IMMUTABLE == //
	page: &'d PageData,

	site: &'d SiteData,

	config: &'d SquarkupConfig,

	/// File to render to.
	dest: PathBuf,

	// == MUTABLE == //
	/// The parsing context stack.
	pub(super) ctx: ContextStack,

	/// Accumulated errors during rendering.
	errors: Vec<SquarkError>,
}

/// Core interface.
impl<'d> Renderer<'d>
{
	pub fn new(
		page: &'d PageData,
		site: &'d SiteData,
		config: &'d SquarkupConfig,
	) -> Self
	{
		Self {
			page,
			site,
			config,
			ctx: ContextStack::new(),
			errors: vec![],
			dest: PathBuf::new(),
		}
	}

	fn _render_(&mut self) -> SquarkResult
	{
		self.dest =
			self.config.out.folder
			.join(&self.page.destination)
			.join(&self.config.out.file);
		
		debug_assert!(self.dest != PathBuf::new());

		log::info!(slash!(
			"rendering to: {GREY1}{}",
			self.dest.strip_prefix(&self.config.paths.root).unwrap().to_path_buf(),
		));

		let mut file = File::open(&self.page.filepath)?;

		let mut source = str!();
		file.read_to_string(&mut source)?;

		if let Some(folder) = self.dest.parent() {
			if !folder.exists() {
				fs::create_dir_all(folder)?;
			}
		}

		let output = self.render_from(source);

		if self.errors.is_empty() || self.config.errors.on_error == ErrorAction::WARN {
			let mut target = File::create(&self.dest)?;
			target.write_all(output.as_bytes())?;
		}

		if self.errors.is_empty() {
			Ok(())
		} else {
			Err(SquarkError::Multiple { errs: self.errors.drain(..).collect() })
		}
	}

	pub(super) fn render_from(&mut self, mut source: String) -> String
	{
		source = Self::expand_only(source);

		// TODO maybe `flat_map` to support context-tracking `only`?
		let parser =
			pd::Parser::new_ext(&source, PARSER_OPTIONS.clone())
				// .inspect(|e| { dbg!(e); })
				.filter_map(|e| self.process_event(e))
		;

		let mut out = str!();
		cmark::cmark_with_options(parser, &mut out, RENDER_OPTIONS.clone()).unwrap();

		out
	}

	/// Remove `<!-- #SQUARK only?` and `#SQUARK only. -->` to expose their content to the render pipeline.
	fn expand_only(source: String) -> String
	{
		Regex::new(
			r"(?is)<!--\s*#SQUARK\s+ONLY\?\s+(?-i)(.*?)(?i)#SQUARK\s+ONLY\.\s*-->"
		).unwrap()
		.replace_all(&source, "$1")
		.to_string()
	}
}

/// Specific transforms.
impl<'d> Renderer<'d>
{
	/// Transform a single `pulldown-cmark` event.
	fn process_event<'e>(&mut self, event: pd::Event<'e>) -> Option<pd::Event<'e>>
	{
		match event {
			pd::Event::Start(pd::Tag::CodeBlock(..)) => { self.ctx.push(Ctx::CODE); }
			pd::Event::End(pd::TagEnd::CodeBlock) => { self.ctx.try_pop(Ctx::CODE); }
			_ => (),
		};

		match event {
			/* We always need to process comments regardless of the current context, to check for squarks that may _change_ the context */
			| pd::Event::Html(ref html)
			| pd::Event::InlineHtml(ref html)
			=>
				match self.process_html(html) {
					Some(true) => Some(event),
					Some(false) => None,
					None => self.process_ctx(event)
				}

			/* But for everything else, handling will depend on the current context */
			_ => self.process_ctx(event)
		}
	}

	/// Transform content depending on the current context.
	fn process_ctx<'e>(&mut self, mut event: pd::Event<'e>) -> Option<pd::Event<'e>>
	{
		match self.ctx.current()
		{
			/* Don't transform anything */
			Ctx::LEAVE{..} => Some(event),

			/* Remove this content */
			Ctx::SLASH{..} => None,

			/* Keep comments only if `preserve_comments: true` */
			Ctx::COMMENT   => self.config.format.preserve_comments.then_some(event),

			_ => match event
			{
				pd::Event::Start(pd::Tag::Link{ ref mut dest_url, .. }) => {
					self.process_link(dest_url);
					Some(event)
				}

				_ => Some(event),
			}
		}
	}

	/// Process HTML content – specifically comments, to check for `<!-- #SQUARK -->`s.
	/// 
	/// Returns:
	/// - `Some(true)` if processing was performed, and the content should be kept.
	/// - `Some(false)` if processing was performed, and the content should be stripped from the output.
	/// - `None` if processing was NOT performed, and the caller should forward to another method.
	fn process_html<'e>(&mut self, html: &pd::CowStr<'e>) -> Option<bool>
	{
		let html = html.trim();
				
		if html.starts_with("<!--") && html.ends_with("-->") {
			return Some(
				if self.process_comment(html) || self.ctx.is_slash() {
					false
				} else {
					self.config.format.preserve_comments || self.ctx.is_leave()
				}
			)
		}
		else if !self.ctx.is_slash() {
			if html.starts_with("<!--") {
				self.ctx.push(Ctx::COMMENT);
				return Some(self.config.format.preserve_comments)
			}
			else if html.ends_with("-->") {
				self.ctx.force_pop(Ctx::COMMENT);
				return Some(self.config.format.preserve_comments)
			}
		}

		None
	}

	/// Attempt to process squarks inside `html`, returning `true` if a squark was matched (and so the comment should be removed).
	fn process_comment(&mut self, html: &str) -> bool
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

	/// Rewrite an internal link that originally points to a Markdown file, such that it points to where *that* Markdown file exports to, using the site map in `self.site`.
	/// 
	/// For instance, suppose page P references `[q](extra.q.md)`. But `q` specifies in its charm squark that it should render to `/secrets/q`. Then we rewrite the `[q](extra/q.md)` link to `[q](/secrets/q)`, both redirecting the link and stripping the `.md` suffix.
	/// 
	/// If resolution fails, an error is added to `self.errors`, and as a best-effort fallback, we try to strip a `.md` suffix from the link.
	fn process_link(&mut self, dest_url: &mut pd::CowStr)
	{
		// 1. find where the target file lives, relative to the current file
		let folder = self.page.filepath.parent().expect("active files are always inside a folder");
		let target_source = folder.join(dest_url.as_ref());

		if !target_source.exists() {
			self.errors.push(SquarkError::Recoverable {
				msg: fmt!("found invalid link: {dest_url}"),
				hint: str!(),
				debug: vec![
					// TODO add line number
					str!(slash!("in: {}", self.page.filepath)),
					str!(slash!("resolved to: {}", target_source)),
				]
			});
		}

		// 2. find where the target file will be exported to
		if let Some(dest_page) = self.site.pages.get(&target_source)
		{
			let href = pathdiff::diff_paths(&dest_page.destination, &self.page.destination)
				.expect("destinations of files always have ROOT as common ancestor");

			*dest_url = pd::CowStr::Boxed(Box::from(href.to_str().unwrap()));
			return;
		}

		match self.config.errors.linked_file_does_not_exist {
			LinkRewriteAction::STRIP_EXTENSION => {
				todo!("replace regex")
			}
			LinkRewriteAction::LINK_TO_GITHUB => {
				todo!("link to github")
			}
			LinkRewriteAction::ERROR => {
				self.errors.push(SquarkError::Recoverable {
					msg: fmt!("found link to inactive page: {dest_url}"),
					hint: str!(),
					debug: vec![
						// TODO add line number
						str!(slash!("in: {}", self.page.filepath)),
					]
				});
			}
		}
	}
}


// == TESTS == //

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

		#[test] fn nested() {
			// FIXME
			// test_expected(&[
			// 	("<!-- <!-- illegal --> comment", " comment"),
			// ]);
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
mod links {
	use super::*;

	mod rewrites {
		use super::*;

		#[test] fn easy() {
			test_expected(&[
				("[link](file.md)", "[link](./file)"),
				("[link](some-file.md)", "[link](./some-file)"),
			]);
		}
	}

	mod preserves {}
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
