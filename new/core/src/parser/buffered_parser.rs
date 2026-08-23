use super::ParseResult;


/// A lazy stateful parser with a backing buffer.
pub trait BufferedParser
{
	fn next_line(&mut self) -> ParseResult;
	fn advance(&mut self) -> ParseResult;
	fn eat(&mut self, chars: impl Iterator<Item = char>) -> ParseResult;
	fn eat_spaces(&mut self) -> ParseResult;
}
