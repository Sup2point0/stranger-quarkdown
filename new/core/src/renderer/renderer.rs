use super::*;
use crate::{
	SquarkupConfig, PageData, SquarkResult, SquarkError,
};

use std::fs::File;
use std::io::{ BufReader, BufWriter, Read, Write };


macro_rules! ctx {
	($self:ident) => { $self.ctx.last().unwrap() }
}


pub struct Renderer<Source: Read = File, Target: Write = File>
{
	/// Have we reached the end of the source?
	pub(super) is_done: bool,

	pub(super) errors: Vec<SquarkError>,

	pub(super) ctx: Vec<Ctx>,

	pub(super) _reader: BufReader<Source>,
	
	pub(super) _writer: BufWriter<Target>,
}

impl<Source: Read, Target: Write>
	Renderer<Source, Target>
{
	/// Construct a renderer for rendering from `source` to `target`.
	pub fn init(source: Source, target: Target) -> Self
	{
		Self {
			is_done: false,
			errors: vec![],
			ctx: vec![],
			_reader: BufReader::new(source),
			_writer: BufWriter::new(target),
		}
	}

	fn render(&mut self, page_data: &PageData, config: &SquarkupConfig) -> SquarkResult
	{
		while !self.is_done {
			self.render_next_chunk(page_data, config)?;
		}
		
		Ok(())
	}

	fn render_next_chunk(&mut self, page_data: &PageData, config: &SquarkupConfig) -> SquarkResult
	{
		match (self.current(), self.peek()) {
			// ('<', '!') if self.try_eat("<!--") => {
			// 	self.push_ctx(Ctx::COMMENT);
				
			// },
			// ('-', '-') if ctx!(self) == Ctx::COMMENT && self.try_eat("-->") => {
			// 	self.ctx.pop();
			// },
		}
	}
}
