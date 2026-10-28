use super::{ RenderCtx as Ctx };
use crate::config::*;
use crate::types::*;
use crate::utils::log;
use crate::utils;
use crate::utils::colours::*;
use crate::utils::macros::*;

use aho_corasick::AhoCorasick;
use map_or_original::MapOrOriginal;
use path_clean::PathClean;
use path_macro::path;
use pulldown_cmark::{
	self as pd,
	Event,
	Event::{ Start, End },
	Tag, TagEnd,
};
use pulldown_cmark_to_cmark as cmark;
use regex::regex;
use time::UtcDateTime;

use std::collections::{ VecDeque };
use std::fmt::{ Write as _ };
use std::fs::{ self, File };
use std::io;
use std::iter::{ Peekable };
use std::ops::{ Range };
use std::path::{ Path, PathBuf };
use std::sync::{ LazyLock };


/// A span in the source text returned by [`pd::Parser`](pulldown_cmark::Parser).
type ParserRange = Range<usize>;


/// Options for parsing with `pulldown-cmark`.
pub static PARSER_OPTIONS: LazyLock<pd::Options>
	= LazyLock::new(||
		  pd::Options::ENABLE_GFM
		| pd::Options::ENABLE_STRIKETHROUGH
		| pd::Options::ENABLE_TABLES
		| pd::Options::ENABLE_MATH
		| pd::Options::ENABLE_FOOTNOTES
		| pd::Options::ENABLE_TASKLISTS
	);

/// Options for rendering with `pulldown-cmark-to-cmark`.
pub static RENDER_OPTIONS: LazyLock<cmark::Options<'static>>
	= LazyLock::new(||
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

	/// The Markdown source text to parse and render.
	pub(super) source: String,

	// == MUTABLE == //

	/// The list of byte boundaries where a new line starts.
	/// 
	/// Each number is an index into `self.source`. The character accessed at that index is the _first_ character in a new line.
	/// 
	/// For instance, `[0, 4, 9, 25]` means bytes 0–3 are line 1 in the source text, 4–8 are line 2, and 9–24 are line 3.
	/// 
	/// If `errors.debug = false`, this is empty. Otherwise, this is always guaranteed to be non-empty, because `0` is the first element.
	pub(super) line_boundaries: Vec<usize>,
	
	/// The parsing context stack.
	pub(super) ctx: ContextStack<Ctx>,

	/// Errors accumulated during rendering.
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
			source: str!(),
			line_boundaries: vec![],
			errors: SquarkError::multiple(str!()),
			dest_file: path!(page.destination / config.out.file_name),
			dest_folder: page.destination.clone(),
			ctx: ContextStack::new(),
		}
	}

	/// Run the renderer to completion, writing the whole `+page.svx` file with all injections.
	pub(super) fn render_page_svx(mut self,
		wrote: &mut bool,
		dry_run: bool,
	) -> SquarkResult
	{
		if self.dest_file.exists() {
			self.err_exists(&self.dest_file)?;
		}

		let (base, folder) = utils::split_path_from(&self.dest_folder, &self.config.paths.root);
		
		log::info!(
			"{} to: {base}/{FF}{folder}{F}/{}",
			if dry_run {"would render"} else {"rendering"},
			self.config.out.file_name,
		);

		let source = fs::read_to_string(&self.page.filepath)?;
		let output = self.render_from(&source)?;

		if !self.ctx.is_empty() {
			self.errors.push(SquarkError::Recoverable {
				msg: str!("unclosed context"),
				hint: fmt!("did you forget to close a {W}{}{G} context?", self.ctx.current()),
				debug: self.ctx.printed(),
			});
		}

		if self.errors.is_fine() || self.config.errors.on_error == ErrorAction::WARN {
			if dry_run {
				self.write_page_svx(io::sink(), &[])?;
			} else {
				self.write_page_svx(File::create(&self.dest_file)?, output.as_bytes())?;
			}
			*wrote = true;
		}

		self.errors.or(())
	}

	fn write_page_svx(&self, mut f: impl io::Write, content: &[u8]) -> SquarkResult
	{
		self.inject_timestamp(&mut f)?;
		self.inject_head(&mut f)?;
		f.write_all(content)?;
		f.flush()?;
		Ok(())
	}

	/// (out-of-place) Render the given Markdown `source`, without surrounding injection.
	pub(super) fn render_from(&mut self, source: &str) -> SquarkResult<String>
	{
		if self.config.errors.debug {
			self.update_source(source);
		}

		let mut parser =
			pd::Parser::new_ext(source, *PARSER_OPTIONS)
			.into_offset_iter()
			.peekable()
		;

		self.skip_heading(&mut parser);
		self.skip_charm_squark(&mut parser);

		let externalise_links = self.config.format.externalise_links;

		let parser =
			pd::TextMergeWithOffset::new(parser)
			.inspect(|(e, _)| if DEBUG { println!("{G}›{F} {e:?}"); })
			.flat_map(Self::split_comments)
		;

		let parser = Self::externalise_links(parser, externalise_links);

		let parser = parser
			.inspect(|(e, _)| if DEBUG { println!("{G}›{W} {e:?}"); })
			.filter_map(|(e, range)| self.process_event(e, range))
		;

		let mut out = str!();

		cmark::cmark_with_options(parser, &mut out, RENDER_OPTIONS.clone())?;

		Ok(out)
	}
}

/// Parser abuse
impl Renderer<'_>
{
	/// Advance `parser` to skip over the events that produce the initial page heading.
	fn skip_heading(&self, parser: &mut Peekable<pd::OffsetIter>)
	{
		if !self.config.format.preserve_heading
		&& let Some((Start(Tag::Heading{ level, .. }), _)) = parser.peek()
		{
			let level = *level;

			for (event, _range) in parser.by_ref() {
				if let End(TagEnd::Heading(lv)) = event
				&& lv == level {
					break;
				}
			}
		}
	}

	/// Advance `parser` to skip over the events that produce the charm squark.
	fn skip_charm_squark(&self, parser: &mut Peekable<pd::OffsetIter>)
	{
		/* NOTE:
		
			Since `parser` is an iterator and `Peekable` only provides a 1-element lookahead, we can't fully verify the upcoming events are actually the charm squark.

			However, any active page must have had a charm squark, either under or in place of the heading, so this should be safe.
		*/

		if !matches!(
			parser.peek(),
			Some((Start(Tag::HtmlBlock), _))
		) {
			return;
		}

		for (event, _range) in parser.by_ref() {
			if matches!(event, End(TagEnd::HtmlBlock)) {
				break;
			}
		}
	}

	/// Split up text containing `<!--` xor `-->` into 2 separate `pd::Event`s with offsets.
	/// 
	/// For instance:
	/// 
	/// - We split `some <!-- comment` into `Text("some ")` and `InlineHtml("<!-- comment")`.
	/// - We split `closing --> comment` into `InlineHtml("closing -->")` and `Text(" comment")`.
	/// 
	/// This transformation is necessary because `pulldown-cmark` does not handle comments crossing multiple lines very well, so for instance this:
	/// 
	/// ```md
	/// A <!-- perhaps
	/// 
	/// strange --> comment
	/// ```
	/// 
	/// Gets parsed to `[Text("a <!-- perhaps"), Text("strange --> comment")]` instead of the more structurally correct `[Text("A"), InlineHtml("<!-- perhaps\n\nstrange -->"), Text("comment")]`.
	fn split_comments((event, range): (Event, ParserRange))
		-> impl Iterator<Item = (Event, ParserRange)>
	{
		if let Event::Text(text)
				| Event::Html(text)
				| Event::InlineHtml(text)
				= &event
			&& text.trim() != "<!--"
			&& text.trim() != "-->"
		{
			let mut idx = 0;
			/* NOTE: `Rule` is a temporary placeholder, we never read it */
			let mut text_left = Event::Rule;
			let mut text_right = Event::Rule;

			// For an opening comment attach the `<!--` to the right fragment and turn the right fragment into HTML
			/* NOTE: `<!--` at the start of a line shouldn't be split */
			if let Some(i) = text.find("<!--")
				&& i > 0
			{
				idx = i;
				let (l, r) = text.split_at(idx);
				text_left = Event::Text(l.to_owned().into());
				text_right = Event::InlineHtml(r.to_owned().into());
			}
			// Do the opposite for a closing comment
			/* NOTE: `-->` at the end of a line shouldn't be split */
			else if let Some(i) = text.find("-->")
				&& i < text.len()
			{
				idx = i + "-->".len();
				let (l, r) = text.split_at(idx);
				text_left = Event::InlineHtml(l.to_owned().into());
				text_right = Event::Text(r.to_owned().into());
			}

			if idx != 0 {
				let left = Some((text_left, range.clone())).into_iter();
				let right = Some((text_right, range)).into_iter();
				return left.chain(right)
			}
		}
		Some((event, range)).into_iter().chain(None)
	}

	/// Transform the sequence of events for a `[link<sup>↗</sup>](linK)` into events that would render to `<a target="_blank" href="link">link</a>`.
	/// 
	/// This is an iterator adapter that wraps `parser` and returns a new iterator.
	/// 
	/// If `enabled` is not `true`, this is a no-op – the returned iterator is equivalent to `.map(id)`.
	fn externalise_links<'e>(
		mut parser: impl Iterator<Item = (Event<'e>, ParserRange)>,
		enabled: bool,
	) -> impl Iterator<Item = (Event<'e>, ParserRange)>
	{
		/*
			Links arrive as multiple events, e.g. [Start(Link), Text, Html, ..., End(Link)] so we won't know whether a link is external until many events down the line. That either requires mutating past iterator events or arbitrary-length lookahead, which ~~both suck~~ aren't feasible.
		
			This adapter is necessary so that we can store a buffer of pending events, and if we find that a link _is_ external, we can then transform that buffer appropriately before actually yielding those events.

			So in a way, I guess this is arbitrary-length lookahead ;P
		*/
		let mut buffer = VecDeque::new();

		std::iter::from_fn(move ||
		{
			if !enabled {
				return parser.next()
			}

			if buffer.is_empty() && let Some((event, range)) = parser.next()
			{
				let is_in_link = matches!(event, Start(Tag::Link{..}));

				/// The next fragment of `<sup>↗</sup>` we need to consume. `None` once we've found all of them.
				let mut next_frag = Some("<sup>");

				buffer.push_back((event, range));

				// when we encounter a `Start(Link)`, read ahead until we see `End(Link)`
				while is_in_link
				{
					let Some((event, range)) = parser.next() else { break };

					match event
					{
						End(TagEnd::Link) =>
						{
							// if this was not an external link, preserve the normal `End(Link)`
							if let Some(..) = next_frag {
								buffer.push_back((event, range));
								break;
							}

							/* When we reach the end of a link which contained <sup>↗</sup>, the only thing we need to touch is the opening `Start(Link)` event, turning into the raw `<a target="_blank" ...` HTML. */
							let Some((Start(Tag::Link{ dest_url, title, id, .. }), ..)) = buffer.get_mut(0)
								else {
									unreachable!("element 0 of `buffer` is always `Start(Link)` if `is_in_link`")
								};

							let mut html = str!("<a target=\"_blank\" ");
							if !id.is_empty()       { let _ = write!(html, "id={:?}", id.to_string()); }
							if !dest_url.is_empty() { let _ = write!(html, "href={:?}", dest_url.to_string()); }
							if !title.is_empty()    { let _ = write!(html, "title={:?}", title.to_string()); }
							let _ = write!(html, ">");

							buffer.front_mut().expect("literally just accessed").0
								= Event::InlineHtml(html.into());
							
							buffer.push_back(
								(Event::InlineHtml("</a>".into()), range)
							);

							break;
						}
						Event::InlineHtml(pd::CowStr::Borrowed("<sup>")) if next_frag == Some("<sup>") => {
							next_frag = Some("↗");
						}
						Event::Text(pd::CowStr::Borrowed("↗")) if next_frag == Some("↗") => {
							next_frag = Some("</sup>");
						}
						Event::InlineHtml(pd::CowStr::Borrowed("</sup>")) if next_frag == Some("</sup>") => {
							next_frag = None;  // done!
						}
						_ => {
							// events before `End(Link)` get queued into the `buffer``
							buffer.push_back((event, range));

							// if we did not find the next required fragment, and we're _still looking_, then reset
							if let Some(..) = next_frag {
								next_frag = Some("<sup>");
							}
						}
					}
				}
			}

			buffer.pop_front()
		})
	}
}

/// Markdown transforms
impl Renderer<'_>
{
	/// Transform a single `pulldown-cmark` event.
	fn process_event<'e>(&mut self,
		event: Event<'e>,
		range: ParserRange,
	) -> Option<Event<'e>>
	{
		match &event
		{
			Start(Tag::CodeBlock(..)) => {
				self.ctx.push(Ctx::CODE);
			}
			End(TagEnd::CodeBlock) => {
				catch!(self.errors => {
					self.ctx.try_pop_exact(&Ctx::CODE)?;
				});
			}
			_ => {}
		};

		let out = match event {
			/* We almost always need to process comments, to check for squarks that may _change_ the context. The exception is if we're in a code context. */
			| Event::Html(ref html)
			| Event::InlineHtml(ref html)
			if !self.ctx.is_code()
			=>
				match self.process_html(html, range.clone()) {
					ProcessAction::KEEP  => Some(event),
					ProcessAction::ERASE => None,
					ProcessAction::DEFER => self.process_ctx(event, range),
				}

			/* But for everything else, handling will depend on the current context */
			_ => self.process_ctx(event, range),
		};

		if DEBUG {
			println!(
				"\n{G}»{B} {B}{:?}{W}",
				self.ctx.prints_with_prefix("").rev().collect::<Vec<_>>()
			);
		}

		out
	}

	/// Transform content depending on the current context.
	fn process_ctx<'e>(&mut self,
		mut event: Event<'e>,
		range: ParserRange,
	) -> Option<Event<'e>>
	{
		/* NOTE: LEAVE has higher priority than SLASH, so doesn't matter if it's [SLASH, LEAVE] or [LEAVE, SLASH], the content should still be left untouched! */
		if self.ctx.has_leave() {
			return Some(event)
		}

		if self.ctx.has_slash() {
			return None
		}

		/* Keep comments only if `preserve_comments: true` */
		if matches!(self.ctx.current(), Ctx::COMMENT) {
			return self.config.format.preserve_comments.then_some(event)
		}

		match event
		{
			// rewrite link
			Start(Tag::Link{ ref mut dest_url, .. }) =>
			{
				self.ctx.push(Ctx::LINK);
				return self.process_link(dest_url, range).or(Some(event))
			}

			// pop link context
			End(TagEnd::Link)
				if self.ctx.try_pop_exact(&Ctx::LINK_REWRITE).is_ok() =>
			{
				return Some(Event::InlineHtml("</a>".into()))
			}

			End(TagEnd::Link) =>
			{
				catch!(self.errors => {
					self.ctx.try_pop_exact(&Ctx::LINK)?;
				});
				return Some(event)
			}

			// rewrite asset link
			Start(Tag::Image { ref mut dest_url, .. }) =>
			{
				self.ctx.push(Ctx::IMAGE_LINK);
				self.process_image(dest_url, range);
				return Some(event)
			}
			
			// pop asset context
			End(TagEnd::Image) =>
			{
				catch!(self.errors => {
					self.ctx.try_pop_exact(&Ctx::IMAGE_LINK)?;
				});
				return Some(event)
			}

			// sanitise text
			Event::Text(text) => Some(self.process_text(text)),

			_ => Some(event),
		}
	}

	/// Process HTML comments to check for `<!-- #SQUARK -->`s and/or strip comments from the output.
	fn process_html(&mut self, html: &str, range: ParserRange) -> ProcessAction
	{
		let html = html.trim();
		let preserve = self.config.format.preserve_comments;
		
		if html.starts_with("<!--") && html.ends_with("-->") {
			if self.process_comment(html, range) || self.ctx.has_slash() {
				return ProcessAction::ERASE
			}
			return ProcessAction::from(preserve || self.ctx.has_leave())
		}
		else if !self.ctx.has_slash() && !self.ctx.has_leave() {
			if html.starts_with("<!--") {
				// check `<!-- #SQUARK only?`
				if self.process_comment(html, range) {
					return ProcessAction::ERASE
				}
				self.ctx.push(Ctx::COMMENT);
				return ProcessAction::from(preserve)
			}
			else if html.ends_with("-->") {
				// check `#SQUARK only. -->`
				if self.process_comment(html, range) {
					return ProcessAction::ERASE
				}

				if self.ctx.try_pop_exact(&Ctx::COMMENT).is_ok() {
					return ProcessAction::from(preserve)
				} else {
					/* NOTE: If `-->` didn't terminate a comment context, it was just a stray `-->` which should be kept */
					return ProcessAction::KEEP
				}
			}
		}
		ProcessAction::DEFER
	}

	/// Attempt to process squarks inside `html`, returning `true` if the comment should be [`ERASE`](ProcessAction::ERASE)d.
	fn process_comment(&mut self, html: &str, range: ParserRange) -> bool
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
					msg: fmt!(
						"unknown squark pattern: {W}{BOLD}{}{UNBOLD}",
						html.trim_start_matches("<!--").trim_end_matches("-->").trim()
					),
					hint: fmt!("use squarks like this: {Y}<!-- #SQUARK leave? -->"),
					debug: [self.source_printed(range), self.ctx.printed()].concat(),
				});
			}
			return false
		};

		let m1 = &captures[1];
		let m2 = &captures[2];
		let key = captures.get(3).map(|k| k.as_str().to_owned());

		let squark = match m1 {
			s if s.eq_ignore_ascii_case("LEAVE") => Ctx::LEAVE { key },
			s if s.eq_ignore_ascii_case("SLASH") => Ctx::SLASH { key },
			s if s.eq_ignore_ascii_case("ONLY")  => Ctx::ONLY,
			s => {
				if !self.ctx.has_leave() {
					self.errors.push(SquarkError::Recoverable {
						msg: fmt!("unknown twin squark: {W}{BOLD}{s}{UNBOLD}"),
						hint: fmt!("valid twin squarks are {Y}leave{G} · {Y}slash{G} · {Y}only"),
						debug: [self.source_printed(range), self.ctx.printed()].concat(),
					});
				}
				return false
			}
		};

		if self.ctx.has_leave() && !matches!(squark, Ctx::LEAVE{..}) {
			return false
		}

		match m2 {
			"?" => self.ctx.push(squark),
			"." => {
				if self.ctx.try_pop_exact(&squark).is_err() {
					self.errors.push(SquarkError::Recoverable {
						msg: fmt!("unpaired closing squark"),
						hint: {
							if self.ctx.contains(&squark) {
								fmt!("did you forget to close a {} context?", self.ctx.current())
							} else {
								fmt!("did you forget to open a {W}{squark}?{G} context?")
							}
						},
						debug: [self.source_printed(range), self.ctx.printed()].concat(),
					});
				}
			}
			_ => unreachable!("RegEx pattern only allows ? and ."),
		}

		true
	}

	/// Rewrite an internal link that originally points to a Markdown file, such that it points to where *that* Markdown file exports to, using the site map in `self.site`.
	/// 
	/// For instance, suppose page P references `[q](extra.q.md)`. But `q` specifies in its charm squark that it should render to `/secrets/q`. Then we rewrite the `[q](extra/q.md)` link to `[q](/secrets/q)`, both redirecting the link and stripping the `.md` suffix.
	/// 
	/// If resolution fails, an error is added to `self.errors`, and as a best-effort fallback, we try to strip a `.md` suffix from the link.
	/// 
	/// If no rewriting was performed, this returns `None`.
	fn process_link<'e>(&mut self,
		dest_url: &mut pd::CowStr,
		range: ParserRange,
	) -> Option<Event<'e>>
	{
		/* We only rewrite relative links to Markdown files, or folders. We ignore external links and anchor links. */
		if dest_url.is_empty()
		|| dest_url.starts_with('#')
		|| dest_url.contains("://")
		|| dest_url.starts_with("mailto:") {
			return None
		}

		let mut anchor: Option<&str> = None;

		let their_file_name = {
			if let Some((left, right)) = dest_url.split_once(".md#") {
				anchor = Some(right);
				left.replace("%20", " ") + ".md"
			} else {
				dest_url.replace("%20", " ")
			}
		};

		// 1. find where the target file lives, relative to the current file
		let own_source_folder = self.page.filepath.parent()
			.expect("active files always have a parent folder");

		let their_source_path = path!(own_source_folder / their_file_name).clean();

		// if it's a file link, only accept `.md` files
		if let Some(ext) = their_source_path.extension() && ext != "md" {
			return None
		}

		let is_folder_link = their_source_path.extension().is_none();

		// 2. find where the target file will be exported to
		let mut shard = PageData::shard_for(&their_source_path, self.config);
		let mut lookup = self.site.get_page(&shard);

		// folder links resolve to the README of that folder
		for readme in ["readme.md", "README.md"] {
			if lookup.is_none() && is_folder_link {
				shard = PageData::shard_for(&path!(their_source_path / readme), self.config);
				lookup = self.site.get_page(&shard);
			}
		}

		if let Some(dest_page) = lookup
		{
			let own_dest_folder = self.page.destination.parent()
				.expect("destination always has a parent folder");

			let mut href = utils::display_rel(&dest_page.destination, own_dest_folder);

			if let Some(a) = anchor {
				href += "#";
				href += a;
			}

			*dest_url = href.into();
			return None
		}
		// 3. handle broken or inactive link
		else {
			let mut link_rewritten =
				their_file_name[..]
				.map_or_orig(|f| f.strip_suffix(".md"))
				.to_owned()
			;
			if let Some(a) = anchor {
				link_rewritten += "#";
				link_rewritten += a;
			}

			if !their_source_path.exists() {
				self.process_broken_link(
					self.config.errors.link_broken,
					dest_url, range, link_rewritten,
					fmt!("broken link to a nonexistent {}", if is_folder_link {"folder"} else {"file"}),
					str!(),
					slash!("resolved to: {}", their_source_path)
				)
			} else {
				self.process_broken_link(
					self.config.errors.link_inactive,
					dest_url, range, link_rewritten,
					str!("link to an inactive page"),
					fmt!("does the linked page have a {Y}<!-- #SQUARK live! -->{G}?"),
					fmt!("resolved to: {shard}")
				)
			}
		}
	}

	fn process_broken_link<'e>(&mut self,
		action: LinkRewriteAction,
		dest_url: &mut pd::CowStr,
		range: ParserRange,
		link_rewritten: String,
		msg: String,
		hint: String,
		resolved_to: String,
	) -> Option<Event<'e>>
	{
		match action
		{
			LinkRewriteAction::MARK_BROKEN =>
			{	
				log::error(SquarkError::Recoverable {
					msg: fmt!("found a {msg}, marking it as broken"),
					hint,
					debug: [
						self.source_printed(range),
						vec![resolved_to],
					].concat()
				});

				if self.ctx.try_pop_any(&Ctx::LINK).is_err() {
					log::bad!("internal bug: expected an existing link context to be pushed before popping in inactive link handler");
				};
				self.ctx.push(Ctx::LINK_REWRITE);
				
				let href = "<a class=\"broken-link\" href=\"\">";
				
				return Some(Event::InlineHtml(href.into()))
			}

			LinkRewriteAction::STRIP_EXTENSION =>
			{
				log::info!("found a {msg}, rewriting it: {FF}{dest_url}{F} ⟶ {FF}{link_rewritten}");

				*dest_url = link_rewritten.into();
			}

			LinkRewriteAction::LINK_TO_GITHUB =>
			{
				let href = fmt!(
					"https://github.com/{}/blob/main/{}",
					self.config.project.github,
					utils::display_rel(&self.page.filepath, &self.config.paths.root),
				);
				log::info!("found a {msg}, rewriting it: {FF}{dest_url}{F} ⟶ {FF}{href}");

				*dest_url = href.into();
			}

			LinkRewriteAction::ERROR =>
			{
				self.errors.push(SquarkError::Recoverable {
					msg: fmt!("found a {msg}"),
					hint,
					debug: [
						self.source_printed(range),
						vec![resolved_to],
					].concat()
				});
			}
		}
		None
	}

	fn process_image(&mut self, dest_url: &mut pd::CowStr, range: ParserRange)
	{
		if dest_url.contains("://")
		|| dest_url.starts_with("http") {
			return;
		}

		// 1. find where the asset file lives, relative to the current file
		let own_source_folder = self.page.filepath.parent()
			.expect("active files always have a parent folder");

		let their_source_path = path!(own_source_folder / **dest_url).clean();

		// 2. check it's an asset file
		if !their_source_path.exists() {
			self.errors.push(SquarkError::Recoverable {
				msg: fmt!("found a broken asset link"),
				hint: str!(),
				debug: [
					self.source_printed(range),
					vec![slash!("resolved to: {}", their_source_path)],
				].concat()
			});
			return;
		}

		if !self.config.assets.has_asset_extension(&their_source_path) {
			return;
		}

		// 3. resolve to site link
		let Some(their_path_rel) = self.config.assets.rel_path(&their_source_path)
			else {
				self.errors.push(SquarkError::Recoverable {
					msg: fmt!("found a link to an asset not processed by Squarkdown"),
					hint: fmt!("this asset isn't under {Y}assets.folder{G} or {Y}assets.site-assets.folder{G}, so Squarkdown doesn't know how to link to it"),
					debug: [
						self.source_printed(range),
						vec![slash!("resolved to: {}", their_source_path)],
					].concat()
				});
				return;
			};

		let dest = slash!("/{}", their_path_rel);

		*dest_url = pd::CowStr::Boxed(Box::from(dest));
	}

	/// Sanitise content in `text` for safety.
	/// 
	/// This includes:
	/// 
	/// - HTML entity processing
	fn process_text<'e>(&self, text: pd::CowStr) -> Event<'e>
	{
		let mut text = HTML_ENTITY_PATTERNS.replace_all(&text, &HTML_ENTITY_REPLACE_WITH);

		if self.ctx.is_link() {
			text = BRACKET_PATTERNS.replace_all(&text, &BRACKET_REPLACEMENTS);
		}

		Event::Text(text.into())
	}
}

/// HTML entities that [`pulldown_cmark`] parses as unicode, to transform back into literal entities mapping into [`HTML_ENTITY_REPLACE_WITH`].
pub static HTML_ENTITY_PATTERNS: LazyLock<AhoCorasick>
	= LazyLock::new(||
		AhoCorasick::new(["\u{2002}", "\u{2003}"]).unwrap()
	);

/// HTML entities that [`pulldown_cmark`] parses as unicode, mapped from [`HTML_ENTITY_PATTERNS`].
pub const HTML_ENTITY_REPLACE_WITH: [&str; 2]
	= ["&ensp;", "&emsp;"];

pub static BRACKET_PATTERNS: LazyLock<AhoCorasick> = LazyLock::new(||
	AhoCorasick::new(["[", "]"]).unwrap()
);
pub static BRACKET_REPLACEMENTS: [&str; 2]
	= ["&lbrack;", "&rbrack;"];


/// Miscellaneous injections
impl Renderer<'_>
{
	/// Write a timestamp to `file`.
	fn inject_timestamp(&self, file: &mut impl io::Write) -> SquarkResult
	{
		let now = UtcDateTime::now();

		write!(
			file,
			"<!-- rendered by Squarkdown on {YYYY} {M} {D} at {hh:02}:{mm:02} -->\n\n",
			YYYY = now.year(), M = now.month(), D = now.day(),
			hh = now.hour(), mm = now.minute(),
		)?;

		Ok(())
	}

	fn inject_head(&self, file: &mut impl io::Write) -> SquarkResult
	{
		if self.page.title.is_empty() {
			return Ok(())
		}

		writeln!(file, "<svelte:head>")?;
		
		let title   = &self.page.title;
		let project = &self.config.project.name;
		
		if project.is_empty() {
			writeln!(file, "\t<title> {title} </title>")?;
		} else {
			writeln!(file, "\t<title> {title} · {project} </title>")?;
		}

		if !self.page.description.is_empty()
		{
			let content = self.page.description.replace('\"', "\\\"");
			writeln!(file, "\t<meta name=\"description\" content=\"{content}\" />")?;
		}

		writeln!(file, "</svelte:head>\n")?;

		Ok(())
	}
}

/// Debug
impl Renderer<'_>
{
	/// Update `self.source` and `self.line_boundaries` with `source` for debug information.
	fn update_source(&mut self, source: &str)
	{
		source.clone_into(&mut self.source);

		/* First character is the first character in a new line */
		self.line_boundaries = vec![0];

		self.line_boundaries.extend(
			/* The character _after_ `\n` is the first in a new line */
			source.match_indices('\n').map(|(i, _str)| i + 1)
		);

		/* NOTE: Line boundaries assume each line is `\n`-terminated, so making sure the last line has this too makes things a little nicer */
		if !self.source.ends_with('\n') {
			self.source.push('\n');
		}

		/* Terminating `\n` guaranteed by above, then the character _after_ is the first in a new line */
		self.line_boundaries.push(source.len() + 1);
	}

	/// Print a snapshot of the source text, focused around `range`.
	fn source_printed(&self, range: ParserRange) -> Vec<String>
	{
		let Some(LineInfo{
			line_number, line_range, prev_line_range, next_line_range,
		}) = self.line_info(range.clone())
			else { return vec![] };

		let mut out = Vec::with_capacity(3);
		out.push(str!(" "));

		// previous line
		if let Some(r) = prev_line_range {
			out.push(fmt!(
				"{FF}{:6} {F}│  {W}{}",
				line_number - 1,
				&self.source[r],
			));
		}

		// this line
		let focus = &self.source[range.clone()].trim_end_matches('\n');

		let r_before = line_range.start..range.start;
		let r_after  = range.end..line_range.end;
		let mut before = if r_before.is_empty() {""} else {&self.source[r_before]};
		let mut after  = if r_after.is_empty()  {""} else {&self.source[r_after]};
		
		const MAX_PREVIEW_CHARS: usize = 32;
		let mut ddd_before = "";
		let mut ddd_after = "";
		if let Some(before_clipped) = utils::truncated_start(before, MAX_PREVIEW_CHARS) {
			before = before_clipped;
			ddd_before = "...";
		}
		if let Some(after_clipped) = utils::truncated_end(after, MAX_PREVIEW_CHARS) {
			after = after_clipped;
			ddd_after = "...";
		}

		out.push(fmt!(
			"{FF}{line_number:6} {F}│  {ddd_before}{W}{before}{R}{BOLD}{focus}{UNBOLD}{W}{after}{F}{ddd_after}",
		));

		// next line
		if let Some(r) = next_line_range {
			out.push(fmt!(
				"{FF}{:6} {F}│  {W}{}",
				line_number + 1,
				&self.source[r],
			));
		}

		out.push(str!(" "));
		out
	}

	/// Find relevant information surrounding the line of `self.source` that `range` resides in.
	fn line_info(&self, mut range: impl Iterator<Item = usize>) -> Option<LineInfo>
	{
		let start = range.next()?;

		for i in 0..self.line_boundaries.len() {
			let upper_this = self.line_boundaries[i];

			if start >= upper_this {
				continue;
			}

			let lower_this = *self.line_boundaries.get(zero!(i - 1)).unwrap_or(&0);

			return Some(LineInfo {
				line_number: i,
				line_range: lower_this..(upper_this - 1),
				prev_line_range:
					if i < 2 { None } else {
						self.line_boundaries.get(i - 2).map(|&lower_prev| lower_prev..(lower_this - 1))
					},
				next_line_range:
					self.line_boundaries.get(i + 1).map(|&upper_next| upper_this..(upper_next - 1)),
			})
		}

		None
	}
}

struct LineInfo
{
	/// 1-indexed.
	line_number: usize,
	line_range: Range<usize>,
	prev_line_range: Option<Range<usize>>,
	next_line_range: Option<Range<usize>>,
}

/// Core utilities
impl Renderer<'_>
{
	/// Produce the appropriate [`SquarkResult`] for propagation for when the target file already exists, depending on `errors.file-already-exists`.
	pub(super) fn err_exists(&self, filepath: &Path) -> SquarkResult
	{
		match self.config.errors.file_already_exists
		{
			FileAction::OVERWRITE => Ok(()),
			FileAction::ERROR => Err(SquarkError::Recoverable {
				msg: slash!("cannot overwrite existing file: {W}{}", filepath),
				hint: fmt!("Squarkdown will not overwrite files since you set {Y}errors.file-already-exists{G} to {W}'error'"),
				debug: vec![],
			}),
			FileAction::SKIP => {
				log::error(SquarkError::Recoverable {
					msg: slash!("warning: not overwriting existing file: {W}{}", filepath),
					hint: fmt!("Squarkdown will not overwrite since you set {Y}errors.file-already-exists{G} to {W}'skip'"),
					debug: vec![],
				});
				Err(SquarkError::ABANDON)
			}
		}
	}
}


/// What a `process_*` function did with its input event.
enum ProcessAction
{
	/// The event was processed, and should be kept.
	KEEP,

	/// The event was processed, is to be removed from the output.
	ERASE,

	/// The event was _not_ processed, and should be deferred to a different `process_*` function.
	DEFER,
}

impl From<bool> for ProcessAction {
	fn from(value: bool) -> Self {
		if value { Self::KEEP } else { Self::ERASE }
	}
}


// == TESTS == //

#[cfg(test)] use super::test_utils::*;
#[cfg(test)] use crate::utils::testing::*;

#[cfg(test)] use assertables::*;
#[cfg(test)] use indoc::indoc;

/* NOTE: Test cases cannot _start_ immediately with HTML or a comment, because it will get stripped by `skip_charm_squark` (which assumes the input starts with a charm squark) */


const DEBUG: bool
// 	= true;

// #[test] fn playground()
// {
// 	let source = indoc! {"
// 		[link<sup>↗</sup>](test)
// 	"};
// 	// let source = indoc! {"
// 	// 	<a target=\"_blank\" href=\"test\">link<sup>↗</sup></a>
// 	// "};

// 	let mut config = TEST_CONFIG.clone();
// 	config.errors.strict = true;
// 	config.errors.strict = false;
// 	config.errors.link_inactive = LinkRewriteAction::MARK_BROKEN;
// 	config.format.externalise_links = true;
// 	let mut renderer = Renderer::new(&TEST_PAGE, &TEST_SITE_DATA, &config);
// 	let out = renderer.render_from(source.trim()).unwrap();
// 	log::error(renderer.errors);
// 	println!("{C}{out}{W}");
// 	log::line();
// }

// const _: bool
	= false;


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
			indoc! {"
				sup, world!

				## Section

				sup, world!
			"},
		]);
	}

	mod edge {
		use super::*;

		#[test] fn html_entities() {
			test_preserves(&[
				"&ensp;",
				"&emsp;",
				"x&ensp;y",
				"x&ensp; y",
				"x &ensp;y",
				"x &ensp; y",
			]);
		}

		#[test] fn ensp_with_emphasis() {
			test_preserves(&[
				"*&ensp;*",
				"**&ensp;**",
				"***&ensp;***",
			]);
		}
	}
}

#[cfg(test)]
mod html {
	use super::*;

	#[test] fn easy() {
		test_preserves(&[
			"Test\n\n<div>hi</div>",
			"Test\n\n<div>hello there</div>",
		]);
	}

	#[test] fn medium() {
		test_preserves(&[
			indoc! {"
				Test

				<div class=\"sup\">

				|a|table|
				|-|-----|
				|x|y|

				</div>
			"},
		]);
	}

	#[test] fn hard() {
		test_preserves(&[
			indoc! {"
				Test

				<div class=\"sup\">
				<div class=\"suppety\">

				|a|table|
				|-|-----|
				|x|y|

				</div>
				</div>
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

	#[test] fn paragraph() {
		test_preserves(&[
			"`x`",
			"`code`",
		]);
	}

	#[test] fn double() {
		test_preserves(&[
			"`` ` ``",
			"`` x` ``",
			"`` `x ``",
			"`` `x` ``",
		]);
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
		]);
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
		]);
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
			indoc! {"
				```md

				<!-- #SQUARK slash? -->
				test
				<!-- #SQUARK slash. -->

				```
			"},
			indoc! {"
				```md
				<!-- #SQUARK only?

				This is dangerous

				     #SQUARK only. -->
				```
			"},
		]);
	}

	#[test] fn huge_block() {
		test_preserves(&[
			indoc! {"
				```md
				# What is Squarkdown-flavoured Markdown?
				<!-- #SQUARK live! feat!
				| dest = tests/sq-fl-mk
				| capt = It’s Markdown with wacky comments.
				| tags = tests / demos / no-deploy
				| date = 2025 March 14
				-->

				<!-- #SQUARK leave? -->
				Squarks won’t change your Markdown preview in any way.
				<!-- #SQUARK leave. -->

				<!-- #SQUARK only?
				But they can influence how it’s processed when rendered to HTML!
					#SQUARK only. -->
				```
			"},
		]);
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
		]);
	}

	#[test] fn edge() {
		test_expected(&[
			("```code```",   "`code`"),
			("```code\n```", "```code\n```"),
			("``````",       "```\n```"),
			("``` ```",      "` `"),
			("```\n```",     "```\n```"),
		]);
	}
}

/// Here we test basic link preservation, not resolution and rewriting. Since these require proper filepaths and multiple pages, that functionality is left for the integration tests.
/// 
/// Links here have to be external, otherwise they trigger link rewriting. Unfortunately this makes the cases a lot more verbose, but ah well.
#[cfg(test)]
mod links {
	use super::*;

	#[test] fn emphasis() {
		test_preserves(&[
			"[*link*](https://github.com)",
			"[**link**](https://github.com)",
			"[***link***](https://github.com)",
			"[~~link~~](https://github.com)",
		]);
	}

	#[test] fn nested_brackets() {
		test_expected(&[
			(
				"[[link]](https://github.com)",
				"[&lbrack;link&rbrack;](https://github.com)"
			), (
				"[[nested] bracketed](https://github.com)",
				"[&lbrack;nested&rbrack; bracketed](https://github.com)"
			), (
				"[a [nested]](https://github.com)",
				"[a &lbrack;nested&rbrack;](https://github.com)"
			), (
				"[a [nested] bracketed](https://github.com)",
				"[a &lbrack;nested&rbrack; bracketed](https://github.com)"
			), (
				"[[[link]]](https://github.com)",
				"[&lbrack;&lbrack;link&rbrack;&rbrack;](https://github.com)"
			),
		]);
	}
}

#[cfg(test)]
mod footnotes {
	use super::*;

	#[test] fn easy() {
		test_preserves(&[
			"[^footnote]\n\n[^footnote]: footnote",
			"[^footnote]\n\n[^footnote]: This is a footnote",
		]);
	}

	#[test] fn medium() {
		test_preserves(&[
			"This ends with a[^footnote]\n\n[^footnote]: footnote",
			"This ends with a[^footnote]\n\n[^footnote]: foot note",
			"This ends with a[^footnote]\n\n[^footnote]: footnote\n\nnext line",
			"This ends with a[^footnote]\n\n[^footnote]: foot note\n\nnext line",
			"This ends with a[^footnote]\n\n[^footnote]: This is a footnote\n\nnext line",
			"This has a[^note] here\n\n[^note]: note",
			"This has a[^note] here\n\n[^note]: foot note",
		]);
	}

	#[test] fn with_punctuation_before() {
		test_preserves(&[
			"x[^footnote]\n\n[^footnote]: a footnote",
			"x,[^footnote]\n\n[^footnote]: a footnote",
			"x.[^footnote]\n\n[^footnote]: a footnote",
			"x?[^footnote]\n\n[^footnote]: a footnote",
			"x [^footnote]\n\n[^footnote]: a footnote",
			"x, [^footnote]\n\n[^footnote]: a footnote",
			"x. [^footnote]\n\n[^footnote]: a footnote",
			"x! [^footnote]\n\n[^footnote]: a footnote",
			"x? [^footnote]\n\n[^footnote]: a footnote"
		]);
	}

	#[test] fn with_hyphen() {
		test_preserves(&[
			"[^a-footnote]\n\n[^a-footnote]: sup",
			"[^a-footnote]\n\n[^a-footnote]: sup sup",
			"[^a-footnote]\n\n[^a-footnote]: sup\n\nnext line",
			"[^a-footnote]\n\n[^a-footnote]: sup sup\n\nnext line",
			"This ends with a[^a-footnote]\n\n[^a-footnote]: sup",
			"This ends with a,[^a-footnote]\n\n[^a-footnote]: sup",
			"This ends with a.[^a-footnote]\n\n[^a-footnote]: sup",
			"This ends with a.[^a-footnote]\n\n[^a-footnote]: sup sup",
			"This ends with a.[^a-footnote]\n\n[^a-footnote]: sup sup\n\nnext line",
			"[^a-longer-footnote]\n\n[^a-longer-footnote]: sup",
		]);
	}

	#[test] fn with_link() {
		test_preserves(&[
			"[^footnote]\n\n[^footnote]: With a [link](https://github.com) inside",
			"[^footnote]\n\n[^footnote]: [this](https://github.com) is a link",
			"[^footnote]\n\n[^footnote]: this has a [link](https://github.com)",
		]);
	}

	#[test] fn weird_case_with_other_formatting() {
		test_preserves(&[
			indoc! {"
				Here's a [^note] but *now* some **formatting**.

				[^note]: This is a footnote.
			"},
		]);
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

	#[test] fn medium() {
		test_expected(&[
			(
				indoc! {"
					| one | two | three |
					| --- | --- | ----- |
					| 1   | 2   | 3     |
				"},
				indoc! {"
					|one|two|three|
					|---|---|-----|
					|1|2|3|
				"},
			),
		]);
	}

	#[test] fn empty_heading() {
		test_preserves(&[
			indoc! {"
				|one||
				|---|-|
				|1|2|
			"},
			indoc! {"
				||two|
				|-|---|
				|1|2|
			"},
			indoc! {"
				|||
				|-|-|
				|1|2|
			"},
			indoc! {"
				|||
				|-|-|
				|1|2|
			"},
		]);
	}
}

#[cfg(test)]
mod maths_inline {
	use super::*;

	#[test] fn easy() {
		test_preserves(&[
			"the $x$ variable",
			"the $x+y$ variable",
			"the $x + y$ variable",
		]);
	}

	#[test] fn medium() {
		test_preserves(&[
			"$[1, 2, 3]$",
			"$[1,\\ 2,\\ 3]$",
			"$\\{ 1, 2, 3 \\}$",
		]);
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
			indoc! {"
				```math
				A(X, x_{1}, x_{2}) = X[\\max(x_{1}, x_{2})]
				```
			"},
		]);
	}
}

#[cfg(test)]
mod comments {
	use super::*;
	
	mod erases {
		use super::*;

		#[test] fn basic() {
			test_expect(&[
				"erase <!--this--> please",
				"erase <!--this --> please",
				"erase <!-- this--> please",
				"erase <!-- this --> please",
				"erase <!--this comment--> please",
				"erase <!--this comment --> please",
				"erase <!-- this comment--> please",
				"erase <!-- this comment --> please",
			], "erase  please");
		}

		#[test] fn many_in_one_line() {
			test_expected(&[
				("erase <!-- all --> of <!-- this --> please", "erase  of  please"),
			]);
		}

		#[test] fn with_breaks_in_comment() {
			test_expected(&[
				("erase\n<!-- this comment -->\nplease",    "erase\n\n\n\nplease"),
				("erase\n<!--\nthis comment\n-->\nplease",  "erase\n\n\nplease"),
				("erase\n<!--\nthis\ncomment\n-->\nplease", "erase\n\n\nplease"),
			]);
		}

		#[test] fn multi_line_from_line_start() {
			test_expect(&[
				"erase\n<!-- this\ncomment -->\nplease",
				"erase\n<!-- this\n\ncomment -->\nplease",
			], "erase\n\n\n\nplease");
		}

		#[test] fn multi_line_from_mid_line() {
			test_expect(&[
				"erase <!--this\ncomment--> please",
				"erase <!-- this\ncomment --> please",
				"erase <!-- this\n\ncomment --> please",
			], "erase  please");
		}

		#[test] fn multi_line_with_comment_before() {
			test_expect(&[
				"erase <!-- all --> of <!-- these\ncomments --> please",
				"erase <!-- all --> of <!-- these\n\ncomments --> please",
			], "erase  of  please");
		}

		#[test] fn nested() {
			test_expected(&[
				("a <!-- <!-- weird --> comment", "a  comment"),
				("a \n <!-- <!-- weirder --> comment", "a\n\n  comment"),
				("a <!-- <!-- weird --> comment <!-- another --> one", "a  comment  one"),
			]);
		}
	}

	mod preserves {
		use super::*;

		#[test] fn easy() {
			test_preserves_for(|c| c.format.preserve_comments = true, &[
				"keep <!--this--> comment",
				"keep <!--this --> comment",
				"keep <!-- this--> comment",
				"keep <!-- this --> comment",
			]);
		}

		#[test] fn medium() {
			test_preserves_for(|c| c.format.preserve_comments = true, &[
				"keep <!--this comment--> please",
				"keep <!--this comment --> please",
				"keep <!-- this comment--> please",
				"keep <!-- this comment --> please",
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
				pair!("a <!-- <!-- weird --> comment"),
			]);
		}

		#[test] fn unopened() {
			test_preserves(&[
				"not -->",
				"not --> a comment",
				"not -->\na comment",
				"\\-->",
				"--> a comment",
			]);
		}

		#[test] fn inside_inline_code() {
			test_preserves(&[
				"`<!-- just some code -->`",
			]);
		}

		#[test] fn inside_code_block() {
			test_preserves(&[
				"```\n<!-- just some code -->\n```",
			]);
		}
	}
}

#[cfg(test)]
mod slash {
	use super::*;

	#[test] fn one_line() {
		test_expect(&[
			"erase <!-- #SQUARK slash? -->this<!-- #SQUARK slash. --> please",
			"erase <!-- #SQUARK slash? -->this <!-- #SQUARK slash. --> please",
			"erase <!-- #SQUARK slash? --> this<!-- #SQUARK slash. --> please",
			"erase <!-- #SQUARK slash? --> this <!-- #SQUARK slash. --> please",
			"erase <!-- #SQUARK slash? -->  this  <!-- #SQUARK slash. --> please",
		], "erase  please");
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
			(
				indoc! {"
					-
					<!-- #SQUARK slash? -->
					
					```ts
					console.log('sup');
					```

					<!-- #SQUARK slash. -->
				"},
				indoc! {"-"}
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

	#[test] fn with_slash() {
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
			(
				indoc! {"
					Please
					<!-- #SQUARK slash? -->
					<!-- #SQUARK leave? -->
					keep
					<!-- #SQUARK leave. -->
					<!-- #SQUARK slash. -->
					this
				"},
				indoc! {"
					Please



					keep



					this
				"}
			),
		]);
	}

	#[test] fn with_only() {
		test_expected(&[
			(
				indoc! {"
					Don't
					<!-- #SQUARK leave? -->
					<!-- #SQUARK only?
					touch
					     #SQUARK only. -->
					<!-- #SQUARK leave. -->
					this
				"},
				indoc! {"
					Don't



					<!-- #SQUARK only?
					touch
					     #SQUARK only. -->

					

					this
				"}
			),
		]);
	}

	#[test] fn nested() {
		test_expected_for(|c| c.format.preserve_comments = true, &[
			(
				indoc! {"
					1
					<!-- #SQUARK leave? -->
					<!-- #SQUARK leave? -->
					<!-- #SQUARK only
					2
					     #SQUARK only. -->
					<!-- #SQUARK leave. -->
					3
				"},
				indoc! {"
					1





					<!-- #SQUARK only
					2
					     #SQUARK only. -->

					
					
					3
				"},
			),
			(
				indoc! {"
					1
					<!-- #SQUARK leave? -->
					2
					<!-- #SQUARK leave? -->

					<!-- #SQUARK only

					3

					     #SQUARK only. -->
					
					<!-- #SQUARK leave. -->
					4
				"},
				indoc! {"
					1



					2



					<!-- #SQUARK only

					3

					     #SQUARK only. -->


					
					4
				"},
			),
		]);
	}
}

#[cfg(test)]
mod only {
	use super::*;

	#[test] fn easy() {
		test_expected(&[
			(
				"Please \n <!-- #SQUARK only? \n\n show \n\n #SQUARK only. --> me",
				"Please\n\n \n show \n\n me"
			),
			(
				"Please \n <!-- #SQUARK only? \n\n do show \n\n #SQUARK only. --> me",
				"Please\n\n \n do show \n\n me"
			),
		]);
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

	#[test] fn medium() {
		test_expected(&[
			(
				"Please <!-- #SQUARK only? \n\n show \n\n #SQUARK only. --> me",
				"Please \n\nshow\n\n&#32;me"
			),
			(
				"Please <!-- #SQUARK only? \n\n do show \n\n #SQUARK only. --> me",
				"Please \n\ndo show\n\n&#32;me"
			),
		]);
	}

	#[test] fn awkward_whitespace() {
		// FIXME squarks on one line
		// test_expected(&[
		// 	("x <!-- #SQUARK only? y #SQUARK only. --> z",  "x y  z"),
		// 	("x <!-- #SQUARK only?  y #SQUARK only. --> z", "x y  z"),
		// 	("x <!-- #SQUARK only? y #SQUARK only. -->  z", "x y   z"),
		// ]);
	}
}

#[cfg(test)]
mod source_snapshots {
	use super::*;

	#[test] fn line_boundaries() {
		let mut renderer = Renderer::new(&TEST_PAGE, &TEST_SITE_DATA, &TEST_CONFIG);
		renderer.update_source("012\n45\n78");
		assert_eq!( renderer.line_boundaries, vec![0, 4, 7, 10] );
		renderer.update_source("01234\n\n78");
		assert_eq!( renderer.line_boundaries, vec![0, 6, 7, 10] );
		renderer.update_source("\n123\n");
		assert_eq!( renderer.line_boundaries, vec![0, 1, 5, 6] );
	}

	#[test] fn line_number() {
		let mut renderer = Renderer::new(&TEST_PAGE, &TEST_SITE_DATA, &TEST_CONFIG);
		renderer.update_source("012\n45\n78");
		assert_eq!( renderer.line_info(0..).unwrap().line_number, 1 );
		assert_eq!( renderer.line_info(1..).unwrap().line_number, 1 );
		assert_eq!( renderer.line_info(2..).unwrap().line_number, 1 );
		assert_eq!( renderer.line_info(3..).unwrap().line_number, 1 );
		assert_eq!( renderer.line_info(4..).unwrap().line_number, 2 );
		assert_eq!( renderer.line_info(5..).unwrap().line_number, 2 );
		assert_eq!( renderer.line_info(6..).unwrap().line_number, 2 );
		assert_eq!( renderer.line_info(7..).unwrap().line_number, 3 );
		assert_eq!( renderer.line_info(8..).unwrap().line_number, 3 );
	}

	#[test] fn line_range() {
		let mut renderer = Renderer::new(&TEST_PAGE, &TEST_SITE_DATA, &TEST_CONFIG);
		renderer.update_source("012\n45\n78");
		assert_eq!( renderer.line_info(0..).unwrap().line_range, 0..3 );
		assert_eq!( renderer.line_info(1..).unwrap().line_range, 0..3 );
		assert_eq!( renderer.line_info(2..).unwrap().line_range, 0..3 );
		assert_eq!( renderer.line_info(3..).unwrap().line_range, 0..3 );
		assert_eq!( renderer.line_info(4..).unwrap().line_range, 4..6 );
		assert_eq!( renderer.line_info(5..).unwrap().line_range, 4..6 );
		assert_eq!( renderer.line_info(6..).unwrap().line_range, 4..6 );
		assert_eq!( renderer.line_info(7..).unwrap().line_range, 7..9 );
		assert_eq!( renderer.line_info(8..).unwrap().line_range, 7..9 );
	}

	#[test] fn source_printed() {
		let mut renderer = Renderer::new(&TEST_PAGE, &TEST_SITE_DATA, &TEST_CONFIG);
		let source = "line one\nline two\nline three";
		renderer.update_source(&source);

		let i = source.find("two").unwrap();
		let debug = renderer.source_printed(i..i + "two".len());
		assert_eq!( debug[0], " " );
		assert_contains!( debug[1], " 1 " );
		assert_contains!( debug[1], "line one" );
		assert_contains!( debug[2], " 2 " );
		assert_contains!( debug[2], &fmt!("line {R}{BOLD}two{UNBOLD}") );
		// check newlines did not leak (dangerous off-by-1)
		assert_not_contains!( debug[2], &fmt!("line {R}{BOLD}two{UNBOLD}\n") );
		assert_contains!( debug[3], " 3 " );
		assert_contains!( debug[3], "line three" );
		assert_eq!( debug[4], " " );

		let i = source.find("one").unwrap();
		let debug = renderer.source_printed(i..i + "one".len());
		assert_eq!( debug[0], " " );
		assert_contains!( debug[1], " 1 " );
		assert_contains!( debug[1], &fmt!("line {R}{BOLD}one{UNBOLD}") );
		assert_not_contains!( debug[1], &fmt!("line {R}{BOLD}one{UNBOLD}\n") );
		assert_contains!( debug[2], " 2 " );
		assert_contains!( debug[2], "line two" );
		assert_eq!( debug[3], " " );

		let i = source.find("three").unwrap();
		let debug = renderer.source_printed(i..i + "three".len());
		assert_eq!( debug[0], " " );
		assert_contains!( debug[1], " 2 " );
		assert_contains!( debug[1], "line two" );
		assert_contains!( debug[2], " 3 " );
		assert_contains!( debug[2], &fmt!("line {R}{BOLD}three{UNBOLD}") );
		assert_not_contains!( debug[2], &fmt!("line {R}{BOLD}three{UNBOLD}\n") );
		assert_eq!( debug[3], " " );
	}
}
