use super::*;
use crate::config::*;
use crate::types::*;
use crate::log;
use crate::utils;
use crate::colours::*;
use crate::macros::*;

use path_clean::PathClean;
use path_macro::path;
use pulldown_cmark as pd;
use pulldown_cmark_to_cmark as cmark;
use regex::regex;

use std::borrow::{ Cow };
use std::fs;
use std::path::{ Path, PathBuf };
use std::sync::{ LazyLock };


// == IMPLEMENTATION == //

/// Options for parsing with `pulldown-cmark`.
pub static PARSER_OPTIONS: LazyLock<pd::Options> = LazyLock::new(||
	  pd::Options::ENABLE_GFM
	| pd::Options::ENABLE_TABLES
	| pd::Options::ENABLE_FOOTNOTES
	| pd::Options::ENABLE_TASKLISTS
);

/// Options for rendering with `pulldown-cmark-to-cmark`.
pub static RENDER_OPTIONS: LazyLock<cmark::Options<'static>> = LazyLock::new(||
	cmark::Options {
		code_block_token_count: 3,
		list_token: '-',
		..cmark::Options::default()
	}
);


/// Mutable state for tracking rendering context and errors.
pub(super) struct Renderer<'d>
{
	// == IMMUTABLE == //

	pub(super) page: &'d PageData,

	pub(super) site: &'d SiteData,

	pub(super) config: &'d SquarkupConfig,

	/// The target file to render to.
	pub(super) dest_file: PathBuf,

	/// The target folder to render to.
	pub(super) dest_folder: PathBuf,

	// == MUTABLE == //
	
	/// The parsing context stack.
	pub(super) ctx: ContextStack<RenderCtx>,

	/// Accumulated errors during rendering.
	pub(super) errors: SquarkError,
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
			errors: SquarkError::multiple(str!()),
			dest_file: path!(page.destination / config.out.file_name),
			dest_folder: page.destination.clone(),
			ctx: ContextStack::new(),
		}
	}

	pub(super) fn render_page_svx(mut self) -> SquarkResult
	{
		if self.dest_file.exists() {
			self.err_exists(&self.dest_file)?;
		}
		
		log::info!(
			"rendering to: {GREY1}{}{GREY}/{}",
			utils::display_rel(&self.dest_folder, &self.config.paths.root),
			self.config.out.file_name,
		);

		let source = fs::read_to_string(&self.page.filepath)?;
		let output = self.render_from(&source);

		if !self.ctx.is_empty() {
			self.errors.push(SquarkError::Recoverable {
				msg: str!("warning: unterminated rendering context"),
				hint: str!("this may be a bug in the Squarkdown renderering engine!"),
				debug: self.ctx.printed(),
			});
		}

		if self.errors.is_fine() || self.config.errors.on_error == ErrorAction::WARN {
			fs::write(&self.dest_file, output)?;
		}

		self.errors.or(())
	}

	pub(super) fn render_from(&mut self, source: &str) -> String
	{
		let source = Self::expand_only(&source);

		let mut parser =
			pd::Parser::new_ext(&source, *PARSER_OPTIONS)
			.into_offset_iter()
			.peekable()
		;

		/* NOTE: Would love to extract these into their own isolated methods, but the required type annotations are too complex =( */

		// skip heading
		if !self.config.format.preserve_heading
		&& let Some((pd::Event::Start(pd::Tag::Heading{ level, .. }), _)) = parser.peek()
		{
			let level = *level;
			
			while parser.next_if(|(e, _range)| {
				if let pd::Event::End(pd::TagEnd::Heading(lv)) = e
				&& *lv == level {
					false
				} else {
					true
				}
			}).is_some()
			{
				continue;
			}

			// consume the `End(Heading)`
			parser.next();
		}

		// TODO maybe `flat_map` to support context-tracking `only`?
		let parser = parser
			// .inspect(|e| { dbg!(e); })
			.filter_map(|(e, range)| self.process_event(e, range))
		;

		let mut out = str!();
		cmark::cmark_with_options(parser, &mut out, RENDER_OPTIONS.clone()).unwrap();

		// strip charm squark
		match regex!(r"(?is)\A(#+.*?\n)?<!--\s*#SQUARK.*?\n-->").replace(&out, "$1") {
			Cow::Borrowed(..) => out,
			Cow::Owned(out) => out,
		}
	}

	/// Remove `<!-- #SQUARK only?` and `#SQUARK only. -->` to expose their content to the render pipeline.
	fn expand_only(source: &str) -> Cow<'_, str>
	{
		regex!(
			r"(?is)<!--\s*#SQUARK\s+ONLY\?\s+(?-i)(.*?)(?i)#SQUARK\s+ONLY\.\s*-->"
		)
		.replace_all(source, "$1")
	}
}

/// Specific transforms
impl Renderer<'_>
{
	/// Transform a single `pulldown-cmark` event.
	fn process_event<'e>(&mut self,
		event: pd::Event<'e>,
		range: std::ops::Range<usize>,
	) -> Option<pd::Event<'e>>
	{
		match event {
			pd::Event::Start(pd::Tag::CodeBlock(..)) => { self.ctx.push(RenderCtx::CODE); }
			pd::Event::End(pd::TagEnd::CodeBlock) => { self.ctx.try_pop(RenderCtx::CODE).expect("contexts are always balanced"); }
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
			RenderCtx::LEAVE{..} => Some(event),

			/* Erase this content */
			RenderCtx::SLASH{..} => None,

			/* Keep comments only if `preserve_comments: true` */
			RenderCtx::COMMENT => self.config.format.preserve_comments.then_some(event),

			_ => match event
			{
				pd::Event::Start(pd::Tag::Link{ ref mut dest_url, .. }) => {
					self.process_link(dest_url);
					Some(event)
				}
				pd::Event::Start(pd::Tag::Image { ref mut dest_url, .. }) => {
					self.process_image(dest_url);
					Some(event)
				}
				_ => Some(event),
			}
		}
	}

	// TODO use `KEEP`, `ERASE`, `UNHANDLED` enum
	/// Process HTML content – specifically comments, to check for `<!-- #SQUARK -->`s.
	/// 
	/// Returns:
	/// - `Some(true)` if processing was performed, and the content should be kept.
	/// - `Some(false)` if processing was performed, and the content should be erased from the output.
	/// - `None` if processing was NOT performed, and the caller should forward to another method.
	fn process_html(&mut self, html: &str) -> Option<bool>
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
				self.ctx.push(RenderCtx::COMMENT);
				return Some(self.config.format.preserve_comments)
			}
			else if html.ends_with("-->") {
				self.ctx.try_pop(RenderCtx::COMMENT).expect("contexts are always balanced");
				return Some(self.config.format.preserve_comments)
			}
		}

		None
	}

	/// Attempt to process squarks inside `html`, returning `true` if a squark was matched (and so the comment should be removed).
	fn process_comment(&mut self, html: &str) -> bool
	{
		/// The RegEx pattern for twin squarks.
		/// 
		/// - Group 1 is the squark (`leave`, `slash`)
		/// - Group 2 is either `?` (open) or `.` (close).
		/// - Group 3, if present, is an alphanumeric identifier for the section.
		let Some(captures) =
			regex!(r"#(?:squark|SQUARK)\s+([a-zA-Z]+)(\?|\.)(?:\s+\[(\w+)\])?")
			.captures(html)
		else {
			if html.contains("#squark") || html.contains("#SQUARK") {
				self.errors.push(SquarkError::Recoverable {
					msg: fmt!("unknown squark pattern: {W}{html}"),
					hint: fmt!("use squarks like this: {W}<!-- #SQUARK leave? -->"),
					debug: vec![],
				});
			}
			return false;
		};

		let m1 = &captures[1];
		let m2 = &captures[2];
		let key = captures.get(3).map(|k| k.as_str().to_owned());

		let squark = match m1 {
			s if s.eq_ignore_ascii_case("LEAVE") => RenderCtx::LEAVE { key },
			s if s.eq_ignore_ascii_case("SLASH") => RenderCtx::SLASH { key },
			s => {
				if !self.ctx.is_leave() {
					self.errors.push(SquarkError::Recoverable {
						msg: fmt!("unknown twin squark: {W}{s}"),
						hint: fmt!("valid twin squarks are {W}leave{G}, {W}slash{G}, {W}only"),
						debug: self.ctx.printed(),
					});
				}
				return false;
			}
		};

		if self.ctx.is_leave() && !matches!(squark, RenderCtx::LEAVE{..}) {
			return false;
		}

		match m2 {
			"?" => self.ctx.push(squark),
			"." => {
				let r = self.ctx.try_pop(squark);

				if r.is_err() {
					self.errors.push(SquarkError::Recoverable {
						msg: fmt!("unpaired closing squark: {W}{html}"),
						hint: fmt!("did you mean to close a {:?} context?", self.ctx.current()),
						debug: self.ctx.printed(),
					});
				}
			}
			_ => unreachable!("pattern only allows ? and ."),
		}

		true
	}

	/// Rewrite an internal link that originally points to a Markdown file, such that it points to where *that* Markdown file exports to, using the site map in `self.site`.
	/// 
	/// For instance, suppose page P references `[q](extra.q.md)`. But `q` specifies in its charm squark that it should render to `/secrets/q`. Then we rewrite the `[q](extra/q.md)` link to `[q](/secrets/q)`, both redirecting the link and stripping the `.md` suffix.
	/// 
	/// If resolution fails, an error is added to `self.errors`, and as a best-effort fallback, we try to strip a `.md` suffix from the link.
	fn process_link(&mut self, dest_url: &mut pd::CowStr)
	{
		/* We only rewrite relative links to Markdown files */
		if !dest_url.contains(".md")
		|| dest_url.contains("://")
		|| dest_url.starts_with("http")
		|| dest_url.starts_with("mailto:") {
			return;
		}

		// TODO can avoid `.to_string()`?
		let mut their_file_name = dest_url.to_string();
		let mut anchor: Option<String> = None;

		if let Some((left, right)) = dest_url.split_once(".md#") {
			their_file_name = left.to_string() + ".md";
			anchor = Some(right.to_string());
		}

		// 1. find where the target file lives, relative to the current file
		let own_source_folder = self.page.filepath.parent()
			.expect("active files are always inside a folder");

		let their_source_path = path!(own_source_folder / their_file_name).clean();

		if !their_source_path.exists() {
			self.errors.push(SquarkError::Recoverable {
				msg: fmt!("found broken link: {W}({dest_url})"),
				hint: str!(),
				debug: vec![
					// TODO add line number
					slash!("resolved to: {GREY1}{}", their_source_path),
				]
			});
			return;
		}

		// 2. find where the target file will be exported to
		let key = utils::display_rel(&their_source_path, &self.config.paths.root);

		if let Some(dest_page) = self.site.get_page(&key)
		{
			let own_dest_folder = self.page.destination.parent()
				.expect("destination always has a parent folder");

			let href_path = pathdiff::diff_paths(&dest_page.destination, own_dest_folder)
				.expect("destinations of files always have ROOT as common ancestor");

			let href_path_slashed = path_slash::PathBufExt::to_slash(&href_path).unwrap();

			let href = match anchor {
				Some(a) => fmt!("{href_path_slashed}#{a}"),
				None => href_path_slashed.to_string(),
			};

			*dest_url = pd::CowStr::Boxed(Box::from(href));

			return;
		}

		match self.config.errors.inactive_link {
			LinkRewriteAction::STRIP_EXTENSION => {
				todo!("replace regex")
			}
			LinkRewriteAction::LINK_TO_GITHUB => {
				todo!("link to github")
			}
			LinkRewriteAction::ERROR => {
				self.errors.push(SquarkError::Recoverable {
					msg: fmt!("found link to inactive page: {W}({dest_url})"),
					hint: str!(),
					debug: vec![
						// TODO add line number
						fmt!("resolved to: {GREY1}{key}"),
					]
				});
			}
		}
	}

	fn process_image(&mut self, dest_url: &mut pd::CowStr)
	{
		let Some(assets_folder) = &self.config.assets.folder else { return };

		if dest_url.contains("://")
		|| dest_url.starts_with("http") {
			return;
		}

		// 1. find where the asset file lives, relative to the current file
		let own_source_folder = self.page.filepath.parent()
			.expect("active files are always inside a folder");

		let their_source_path = path!(own_source_folder / **dest_url).clean();

		// 2. check it's an asset file
		if !their_source_path.exists() {
			self.errors.push(SquarkError::Recoverable {
				msg: fmt!("found broken asset link: {W}({dest_url})"),
				hint: str!(),
				debug: vec![
					// TODO add line number
					slash!("resolved to: {GREY1}{}", their_source_path),
				]
			});
			return;
		}

		let Some(extension) = their_source_path.extension() else { return };

		if !self.config.assets.extensions.iter().any(|ext| **ext == *extension) {
			return;
		}

		// 3. resolve to site link
		let base = {
			if let Some(site_assets_folder) = &self.config.assets.site_assets_folder
			&& their_source_path.starts_with(site_assets_folder)
			{
				site_assets_folder
			} else {
				assets_folder
			}
		};

		let Ok(path_rel) = their_source_path.strip_prefix(base) else {
			self.errors.push(SquarkError::Recoverable {
				msg: fmt!("found link to non-exported asset: {W}({dest_url})"),
				hint: fmt!("this asset isn't under {Y}assets.folder{G} or {Y}assets.site-assets.folder{G}, so Squarkdown doesn't know how to link to it"),
				debug: vec![
					// TODO add line number
					slash!("resolved to: {GREY1}{}", their_source_path),
				]
			});
			return;
		};

		let dest = slash!("/{}", path_rel);

		*dest_url = pd::CowStr::Boxed(Box::from(dest));
	}
}

/// Core utilities
impl Renderer<'_>
{
	pub(super) fn err_exists(&self, filepath: &Path) -> SquarkResult
	{
		match self.config.errors.file_already_exists
		{
			FileAction::OVERWRITE => Ok(()),
			FileAction::ERROR => Err(SquarkError::Recoverable {
				msg: slash!("cannot overwrite existing file: {W}{}", filepath),
				hint: fmt!("Squarkdown will not overwrite files since you set {Y}errors.file-already-exists{G} to {W}'error'"),
				debug: vec![
					slash!("while rendering: {}", self.page.filepath),
				],
			}),
			FileAction::SKIP => Err(SquarkError::ABANDON),
		}
	}
}


// == TESTS == //

#[cfg(test)] use super::test_utils::*;

#[cfg(test)] use indoc::indoc;


// #[test] fn playground() {
// 	test_preserves_for(|_| {

// 	}, &[
// 		indoc! {"
// 			![asset](./asset.png)
// 		"}
// 	]);
// }


#[cfg(test)]
mod plain {
	use super::*;

	#[test] fn easy() {
		test_preserves(&[
			"sup, world!",
			"sup,\nworld!",
		])
	}

	#[test] fn medium() {
		test_preserves(&[
			indoc! {"
				sup, world!

				## Section

				sup, world!
			"},
		]);
	}
}

#[cfg(test)]
mod heading {
	use super::*;

	mod erases {
		use super::*;

		#[test] fn easy() {
			test_expect(&[
				"# Heading\nThe quick brown fox jumps over the lazy dog",
				"# Heading\n\nThe quick brown fox jumps over the lazy dog",
				"# Heading\n\n\nThe quick brown fox jumps over the lazy dog",
			], "The quick brown fox jumps over the lazy dog")
		}

		#[test] fn medium() {
			test_expect(&[
				"## Level 2\nsup",
				"### Level 3\nsup",
				"# Break\n\nsup",
				"# Space \n\nsup",
				"#  Space\n\nsup",
				"#  Space \n\nsup",
			], "sup")
		}
	}

	mod preserves {
		use super::*;

		#[test] fn easy() {
			test_expect_for(|c| c.format.preserve_heading = true, &[
				"# Heading\nThe quick brown fox jumps over the lazy dog",
				"# Heading\n\nThe quick brown fox jumps over the lazy dog",
				"# Heading\n\n\nThe quick brown fox jumps over the lazy dog",
			], "# Heading\n\nThe quick brown fox jumps over the lazy dog");
		}

		#[test] fn medium() {
			test_expected_for(|c| c.format.preserve_heading = true, &[
				("## Level 2\nsup",  "## Level 2\n\nsup",),
				("### Level 3\nsup", "### Level 3\n\nsup"),
				("# Break\n\nsup",   "# Break\n\nsup", ),
				("# Space \n\nsup",  "# Space\n\nsup",),
				("#  Space\n\nsup",  "# Space\n\nsup",),
				("#  Space \n\nsup", "# Space\n\nsup"),
			]);
		}

		#[test] fn hard() {
			test_preserves_for(|c| c.format.preserve_heading = true, &[
				indoc! {"
					sup, world!

					## Not a Heading

					sup, world!
				"}
			]);
		}
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
		])
	}

	#[test] fn medium() {
		test_expected(&[
			pair!("`x` `y`"),
				  ("` 1 ` ` 2 `", "`1` `2`"),
			pair!("`x y` `z`"),
			pair!("`x`y`z`"),
		])
	}

	#[test] fn unclosed() {
		test_expected(&[
			("`1\n2", "\\`1\n2"),
		])
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
			// FIXME track only context
			// indoc! {"
			// 	```md
			// 	<!-- #SQUARK only?

			// 	This is dangerous

			// 	     #SQUARK only. -->
			// 	```
			// "},
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
		])
	}
}

#[cfg(test)]
mod maths_inline {
	use super::*;

	#[test] fn easy() {
		test_preserves(&[
			"the $x$ variable",
		])
	}
}

#[cfg(test)]
mod maths_block {
	use super::*;

	#[test] fn easy() {
		test_preserves(&[
			indoc! {"
				```math
				f(x) = x
				```
			"},
		])
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
mod slash {
	use super::*;

	#[test] fn one_line() {
		test_expected(&[
			(
				"erase <!-- #SQUARK slash? --> this <!-- #SQUARK slash. --> please",
				"erase  please",
			),
		])
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
		])
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
		])
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
		])
	}

	#[test] fn nested() {
		// FIXME
		// test_expected(&[
		// 	(
		// 		indoc! {"
		// 			1
		// 			<!-- #SQUARK leave? -->
		// 			<!-- #SQUARK leave? -->
		// 			2
		// 			<!-- #SQUARK leave. -->
		// 			<!-- #SQUARK leave. -->
		// 			3
		// 		"},
		// 		indoc! {"
		// 			1

		// 			<!-- #SQUARK leave? -->
		// 			2
		// 			<!-- #SQUARK leave. -->

		// 			3
		// 		"},
		// 	),
		// ])
	}
}

#[cfg(test)]
mod only {
	use super::*;

	#[test] fn easy() {
		test_expected(&[
			("Please <!-- #SQUARK only? show #SQUARK only. --> me", "Please show  me"),
			("Please <!-- #SQUARK only? do show #SQUARK only. --> me", "Please do show  me"),
		])
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
		])
	}

	#[test] fn awkward_whitespace() {
		test_expected(&[
			("x <!-- #SQUARK only? y #SQUARK only. --> z",  "x y  z"),
			("x <!-- #SQUARK only?  y #SQUARK only. --> z", "x y  z"),
			("x <!-- #SQUARK only? y #SQUARK only. -->  z", "x y   z"),
		])
	}
}
