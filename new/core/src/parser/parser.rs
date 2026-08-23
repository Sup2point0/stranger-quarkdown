use super::BufferedParser;


/// A parser for the 'squark charm' header of a file.
struct CharmParser
{
	config: &SquarkupConfig,
	
	/// The current character the parser is pointing to.
	current: Option<char>,
	
	/// Have we encountered a `<!-- #SQUARK live!` yet?
	///
	/// If so, this means the user intends for the file to be squarked up, and error checking should be stricter to catch mistakes on their end.
	is_live: bool,
	
	/// The backing buffer that reads from the target file.
	_reader: BufReader,
	
	/// The currently in-memory chunk to process.
	_chunk: String,
	
	/// An iterator over the characters of the current chunk.
	_chars: Iterator<char>,
}

impl CharmParser
{
	pub fn init(file: File, config: &SquarkupConfig) -> Self
	{
		let mut self = Self {
			config,
			current: None,
			_reader: BufReader::new(file),
			_chunk: String::new(),
			_char: Iterator::empty(),
		}
		
		self.load_line();
		
		self
	}
	
	/// Run the parser to completion, extracting the metadata from the squark charm (if present) of the target file.
	pub fn parse(&mut self) -> Option<FileHeader>
	{
		match self._parse() {
			Ok(r) => Some(r),
			Err(ParseError::NoMatch) => None,
			Err(ParseError(e)) => {
				Log.err(e.to_string());
				None,
			},
		}
	}
	
	fn _parse(&mut self) -> Result<FileHeader>
	{
		self.eat_spaces()?;
		
		let mut heading = None;
		
		if self.current == '#' {
			heading = Some(self.parse_heading()?);
		}
		
		self.parse_charm()?;
		
		FileHeader {
			heading,
		}
	}
	
	fn parse_heading()
	{
		this.eat("#")
	}
	
	fn parse_charm() -> ParseResult
	{
		
	}
}

impl BufferedParser for CharmParser
{
	/// Read the next line of the source text into memory.
	fn next_line(&mut self) -> ParseResult
	{
		self.reader.read_line(&mut self.chunk)?;
		self.chars = self.chunk.chars();
		self.current = self.chars().next();
	}

	/// Proceed to the next character in the source text.
	fn advance(&mut self) -> ParseResult
	{
		self.current = self.chars.next();
		
		if self.current == None {
			self.next_line();
		}
		
		Err(if this.is_live {
			ParseError::FatalEnd
		} else {
			ParseError::NoMatch
		})?
	}
	
	fn eat(&mut self, chars: impl Iterator<item = char>) -> ParseResult
	{
		loop {
			let Some(required) = chars.next() else { return };
			
			this.advance();
		}
	}
	
	fn eat_spaces(&mut self) -> ParseResult
	{
		while this.current == ' ' {
			this.advance()?;
		}
	}
}
