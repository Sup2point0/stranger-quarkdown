use crate::{
	SquarkupConfig, SquarkResult,
};

use std::fs::File;
use std::io::{ BufReader, BufWriter, Read, Write };


pub struct Renderer<Source: Read = File, Target: Write = File>
{
	_reader: BufReader<Source>,
	
	_writer: BufWriter<Target>,
}

impl<Source: Read, Target: Write> Renderer<Source, Target>
{
	pub fn init(source: Source, target: Target) -> Self
	{
		Self {
			_reader: BufReader::new(source),
			_writer: BufWriter::new(target),
		}
	}
	
	pub fn render(&mut self, config: &SquarkupConfig) -> SquarkResult
	{
		// TODO
		println!("rendering...");
		Ok(())
	}
}
