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

		However, the renderer reads in _chunks_ instead of _lines_, because unlike the parser, it handles arbitrary Markdown text that could be super short or super long.
		
		Not really worth extracting into common shared functionality, more hassle than it's worth without structural traits in Rust =(
	*/

	/// Have we reached the end of the source?
	pub is_done: bool,

	pub errors: Vec<SquarkError>,

	pub(super) ctx: ContextStack,

	/// The location of the source file to read from.
	pub(super) source_filepath: PathBuf,

	/// The location of the target file to write to.
	pub(super) target_filepath: PathBuf,

	pub(super) _reader: BufReader<Source>,
	
	pub(super) _writer: BufWriter<Target>,

	pub(super) _index: usize,

	/// The characters of the currently in-memory chunk to process.
	pub(super) _chunk: Vec<char>,
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
			is_done: false,
			errors: vec![],
			ctx: ContextStack::new(),
			_reader: BufReader::new(source),
			_writer: BufWriter::new(target),
			source_filepath,
			target_filepath,
			_index: 0,
			_chunk: vec![' '; CHUNK_SIZE],
		};

		out.next_chunk()?;

		Ok(out)
	}

	pub fn render(&mut self, page: &PageData, config: &SquarkupConfig) -> SquarkResult
	{
		while !self.is_done {
			self.render_next_chunk(page, config)?;
		}

		self._writer.flush().map_err(err!())?;

		if self.ctx.stack().len() > 0 {
			Err(SquarkError::Recoverable {
				msg: str!("unterminated renderer context"),
				hint: str!("this means you have an unclosed comment, bracket, code block, etc. somewhere"),
				debug: vec![
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
	fn render_next_chunk(&mut self, page: &PageData, config: &SquarkupConfig) -> SquarkResult
	{
		debug_assert_matches!(self.current(), Some(..));

		match self.ctx.current()
		{
			Ctx::MARKDOWN     => self.render_plain(config),
			Ctx::CODE_BLOCK   => self.render_code_block(),
			Ctx::COMMENT      => self.render_comment(config),
			Ctx::SQUARK_LEAVE => self.render_leave(config),
			Ctx::SQUARK_SLASH => self.render_slash(config),
			Ctx::SQUARK_ONLY  => todo!(), // self.render_only(),
			_ => unimplemented!(),
		}
	}
}

impl<Source: Read, Target: Write>
	Renderer<Source, Target>
{
	fn render_plain(&mut self, config: &SquarkupConfig) -> SquarkResult
	{
		if self.try_eat("<!--")? {
			if config.format.preserve_comments {
				self.emit("<!--")?;

				self.eat_whitespace()?;

			}
			self.ctx.push(Ctx::COMMENT);
		}
		else if self.try_eat("```")? {
			self.emit("```")?;
			self.ctx.push(Ctx::CODE_BLOCK);
		}
		else if let Some(c) = self.current() {
			self.emit_char(c)?;
			self.advance()?;
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
		else if self.try_eat_caseless("#SQUARK")? {
			let _ =
				self.try_eat_paired_squark("leave", Ctx::SQUARK_LEAVE, true, false)?
			|| self.try_eat_paired_squark("slash", Ctx::SQUARK_SLASH, true, false)?
			|| self.try_eat_paired_squark("only", Ctx::SQUARK_ONLY, true, false)?;
		}
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
			if config.format.preserve_comments {
				self.emit("<!-- ")?;
			}
			
			if self.try_eat_paired_squark("leave", Ctx::SQUARK_LEAVE, true, true)? {
				self.eat_whitespace()?;

				self.eat("-->",
					to!("terminate squark"),
					hints!("close a slashed section like `<!-- #SQUARK slash. -->`"),
				)?;
			}
		}
		else if let Some(c) = self.current() {
			if config.format.preserve_comments {
				self.emit_char(c)?;
			}
			self.advance()?;
		}
		Ok(())
	}

	fn render_slash(&mut self, config: &SquarkupConfig) -> SquarkResult
	{
		if self.try_eat("<!--")? {
			self.eat_whitespace()?;
			if config.format.preserve_comments {
				self.emit("<!-- ")?;
			}

			if self.try_eat_paired_squark("slash", Ctx::SQUARK_SLASH, true, true)? {
				self.eat_whitespace()?;

				self.eat("-->",
					to!("terminate squark"),
					hints!("close a slashed section like `<!-- #SQUARK slash. -->`"),
				)?;
			}
		}
		else {
			self.advance()?;
		}
		Ok(())
	}
}


#[cfg(test)]
mod test
{
	use super::*;
	
	#[test] fn render_plain()
	{
		test_exact(&[
			"sup, world!",
			"sup,\nworld!",
			"sup,\nworld!\n",
			"sup, \nworld!\n",
			"sup,\n world!\n",
			"sup, \n world!\n",
		]);
	}

	#[test] fn render_code_block()
	{
		test_exact(&[
			"This is some code\n\n```\nprint(\"hello world\")\n```",
		])
	}

	#[test] fn test_comment_strip()
	{
		test_expected(
			&vec![
				"Strip <!--this--> comment",
				"Strip <!--this --> comment",
				"Strip <!-- this--> comment",
				"Strip <!-- this --> comment",
			]
			.into_iter()
			.map(|case| (case, "Strip  comment"))
			.collect::<Vec<_>>()
		);
		
		test_expected(
			&vec![
				"Strip <!--this comment--> please",
				"Strip <!--this comment --> please",
				"Strip <!-- this comment--> please",
				"Strip <!-- this comment --> please",
			]
			.into_iter()
			.map(|case| (case, "Strip  please"))
			.collect::<Vec<_>>()
		);
		
		test_expected(
			&vec![
				"Strip\n<!-- this comment -->\nplease",
				"Strip\n<!--\nthis comment\n-->\nplease",
				"Strip\n<!--\nthis\ncomment\n-->\nplease",
			]
			.into_iter()
			.map(|case| (case, "Strip\n\nplease"))
			.collect::<Vec<_>>()
		);
	}

	#[test] fn test_slash()
	{
		test_expected(&[
			(
				"Remove <!-- #SQUARK slash? --> this <!-- #SQUARK slash. --> please",
				"Remove  please",
			),
		]);
	}

	#[test] fn test_leave()
	{
		test_expected(&[
			(
"Don't <!-- #SQUARK leave? -->
<!-- #SQUARK slash? --> touch <!-- #SQUARK slash. -->
<!-- #SQUARK leave. --> this",

"Don't 
<!-- #SQUARK slash? --> touch <!-- #SQUARK slash. -->
 this"
			),
		]);
	}
}
