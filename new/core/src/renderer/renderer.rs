use super::*;
use crate::{
	SquarkupConfig, PageData, SquarkResult, SquarkError,
	macros::*,
};

use std::fs::{ File };
use std::io::{ BufReader, BufWriter, Read, Write };
use std::path::{ PathBuf };
use std::debug_assert_matches;


pub struct Renderer<Source: Read = File, Target: Write = File>
{
	/* NOTE:
		The renderer architecture is very similar to `CharmParser`, because `Renderer` is technically a parser+emitter in one lmao

		However, the renderer reads in _chunks_ instead of _lines_, because unlike the parser, it handles arbitrary Markdown text that could be super short or super long. Reading in chunks means memory usage doesn't explode if a file has one diabolically long line!
		
		Not really worth extracting into common shared functionality, more hassle than it's worth without structural traits in Rust =(
	*/

	/// Have we reached the end of the source?
	pub done_reading: bool,

	pub errors: Vec<SquarkError>,

	pub(super) ctx: ContextStack,

	pub(super) line_number: usize,

	/// The location of the source file to read from.
	pub(super) source_filepath: PathBuf,

	/// The location of the target file to write to.
	pub(super) target_filepath: PathBuf,

	pub(super) _reader: BufReader<Source>,
	
	pub(super) _writer: BufWriter<Target>,

	/// The index in the current chunk the renderer is pointing to.
	pub(super) _index: usize,

	/// The characters of the currently in-memory chunk to process.
	/// 
	/// As the renderer reads from the source file, we append characters to this buffer. When it's safe to do so, we chop off characters that have already been processed to keep the buffer short (avoiding huge memory usage).
	pub(super) _window: Vec<char>,
}

impl<Source: Read, Target: Write>
	Renderer<Source, Target>
{
	/// Construct a renderer for rendering from `source` to `target`.
	pub fn init(
		source: Source,
		target: Target,
		source_filepath: PathBuf,
		target_filepath: PathBuf,
	) -> SquarkResult<Self>
	{
		let mut out = Self {
			done_reading: false,
			errors: vec![],
			ctx: ContextStack::new(),
			line_number: 0,
			_reader: BufReader::new(source),
			_writer: BufWriter::new(target),
			source_filepath,
			target_filepath,
			_index: 0,
			_window: vec![],
		};

		out.next_chunk()?;

		Ok(out)
	}

	pub fn render(&mut self, page: &PageData, config: &SquarkupConfig) -> SquarkResult
	{
		while !self.is_done() {
			self.render_next_chunk(page, config)?;
		}

		self._writer.flush().map_err(err!())?;

		if self.ctx.stack().len() > 0 {
			Err(SquarkError::Recoverable {
				msg: str!("unterminated renderer context"),
				hint: str!("this means you have an unclosed comment, bracket, code block, etc. somewhere"),
				debug: vec![
					slash!("in: {}", self.source_filepath),
					fmt!("context stack: {:?}", self.ctx.stack()),
				],
			})
		} else {
			Ok(())
		}
	}
}

impl<Source: Read, Target: Write>
	Renderer<Source, Target>
{
	fn render_next_chunk(&mut self, _page: &PageData, config: &SquarkupConfig) -> SquarkResult
	{
		self.cleanup_chunk();

		debug_assert_matches!(self.current(), Some(..));

		match self.ctx.current()
		{
			Ctx::MARKDOWN     => self.render_markdown(config),
			Ctx::CODE_INLINE  => self.render_code_inline(),
			Ctx::CODE_BLOCK   => self.render_code_block(),
			Ctx::COMMENT      => self.render_comment(config),
			Ctx::SQUARK_LEAVE => self.render_leave(config),
			Ctx::SQUARK_SLASH => self.render_slash(),
			Ctx::SQUARK_ONLY  => self.render_only(config),
			_ => unimplemented!(),
		}
	}

	/// Handle generic Markdown context openers, which may be shared between many different contexts.
	fn render_plain(&mut self, config: &SquarkupConfig) -> SquarkResult<bool>
	{
		if self.try_eat("<!--")? {
			self.eat_whitespace()?;
			self.ctx.push(Ctx::COMMENT);

			if config.format.preserve_comments {
				self.emit("<!--")?;
			}
			return Ok(true);
		}
		else if self.try_eat("```")? {
			self.emit("```")?;
			self.ctx.push(Ctx::CODE_BLOCK);
			return Ok(true);
		}
		else if self.try_eat("`")? {
			self.emit_char('`')?;
			self.ctx.push(Ctx::CODE_INLINE);
			return Ok(true);
		}
		Ok(false)
	}
}

impl<Source: Read, Target: Write>
	Renderer<Source, Target>
{
	fn render_markdown(&mut self, config: &SquarkupConfig) -> SquarkResult
	{
		if !self.render_plain(config)? && let Some(c) = self.current() {
			self.emit_char(c)?;
			self.advance()?;
			
			if c == '\\' && let Some(cc) = self.current() {
				self.emit_char(cc)?;
				self.advance()?;
			}
		}
		Ok(())
	}

	fn render_code_inline(&mut self) -> SquarkResult
	{
		if self.try_eat("```")? {
			self.emit("```")?;
		}
		else if let Some(c) = self.current() {
			self.emit_char(c)?;
			self.advance()?;

			if c == '`' || c == '\n' || self.out_of_bounds() {
				self.ctx.pop(Ctx::CODE_INLINE);
			}
		}
		Ok(())
	}

	fn render_code_block(&mut self) -> SquarkResult
	{
		if self.try_eat("```")? {
			self.emit("```")?;
			self.ctx.pop(Ctx::CODE_BLOCK);
		}
		else if let Some(c) = self.current() {
			self.emit_char(c)?;
			self.advance()?;
			
			if c == '\\' && let Some(c2) = self.current() {
				self.emit_char(c2)?;
				self.advance()?;
			}
		}
		Ok(())
	}

	fn render_comment(&mut self, config: &SquarkupConfig) -> SquarkResult
	{
		if self.try_eat("-->")? {
			self.ctx.pop(Ctx::COMMENT);

			if config.format.preserve_comments {
				self.emit("-->")?;
			}
		}
		else if
				self.try_open_squark("leave", Ctx::SQUARK_LEAVE)?
			|| self.try_open_squark("slash", Ctx::SQUARK_SLASH)?
			|| self.try_eat_twin_squark("only",  Ctx::SQUARK_ONLY, true, false, false)?
		{}
		else if let Some(c) = self.current() {
			if config.format.preserve_comments {
				self.emit_char(c)?;
			}
			self.advance()?;
		}
		Ok(())
	}

	fn render_leave(&mut self, config: &SquarkupConfig) -> SquarkResult
	{
		if self.try_eat("<!--")? {
			self.eat_whitespace()?;

			// FIXME leave should do best-effort tracking, but otherwise allow invalid syntax
			if self.try_close_squark("leave", Ctx::SQUARK_LEAVE)?
			{
				if self.ctx.current() == Ctx::SQUARK_LEAVE {
					self.emit("<!-- #SQUARK leave. -->")?;
				}
			}
			else if self.try_open_squark("leave", Ctx::SQUARK_LEAVE)? {
				self.emit("<!-- #SQUARK leave? -->")?;
			}
			else {
				self.emit("<!-- ")?;
			}
		}
		else if let Some(c) = self.current() {
			self.emit_char(c)?;
			self.advance()?;
		}
		Ok(())
	}

	fn render_slash(&mut self) -> SquarkResult
	{
		if self.try_eat("<!--")? {
			self.try_open_close_squark("slash", Ctx::SQUARK_SLASH)?;
		} else {
			self.advance()?;
		}
		Ok(())
	}

	fn render_only(&mut self, config: &SquarkupConfig) -> SquarkResult
	{
		if !self.render_plain(config)? {
			if self.try_close_squark("only", Ctx::SQUARK_ONLY)?
			{}
			else if let Some(c) = self.current() {
				self.emit_char(c)?;
				self.advance()?;
			}
		}
		Ok(())
	}
}


#[cfg(test)]
mod test {
	use indoc::indoc;

	use super::*;

mod plain {
	use super::*;

	#[test] fn easy() {
		test_preserves(&[
			"sup, world!",
			"sup,\nworld!",
			"sup,\nworld!\n",
			"sup, \nworld!\n",
			"sup,\n world!\n",
			"sup, \n world!\n",
		]);
	}
}

mod code_inline {
	use super::*;

	#[test] fn easy() {
		test_preserves(&[
			"this `is` code",
			"this `is ` code",
			"this ` is` code",
			"this ` is ` code",
			"this `is` some `more` code",
			"line `1` onto\nline `2`.",
		]);
	}

	#[test] fn medium() {
		test_preserves(&[
			"`1` onto\nline `2`",
			// "`x` `y`",
			// "` x ` ` y `",
			// "`x y` `z`",
			// "`x`y`z`",
		]);
	}

	#[test] fn unclosed() {
		test_preserves(&[
			"`x\ny",
		]);
	}

	#[test] fn edge_cases() {
		test_preserves(&[
			"`x`",
			"`code`",
		])
	}
}

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
	}

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

}
