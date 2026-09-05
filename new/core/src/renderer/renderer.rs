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

	pub(super) ctx: Vec<Ctx>,

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
			ctx: vec![],
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

		if !self.ctx.is_empty() {
			Err(SquarkError::Recoverable {
				msg: str!("unterminated renderer context"),
				hint: str!("this means you have an unclosed comment, bracket, code block, etc. somewhere"),
				debug: vec![],
			})
		} else {
			Ok(())
		}
	}

	fn render_next_chunk(&mut self, page: &PageData, config: &SquarkupConfig) -> SquarkResult
	{
		debug_assert_matches!(self.current(), Some(..));

		match self.ctx.last() {
			Some(..) => unimplemented!(),
			None => self.render_plain(page, config),
		}
	}
}

impl<Source: Read, Target: Write>
	Renderer<Source, Target>
{
	fn render_plain(&mut self, page: &PageData, config: &SquarkupConfig) -> SquarkResult
	{
		if self.try_eat("<!--")? {
			self.ctx.push(Ctx::COMMENT);
		}
		// else if self.try_eat("-->")? {
		// 	self.ctx.pop();
		// }
		else if let Some(c) = self.current() {
			self.emit_char(c)?;
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
		]);
	}
}
