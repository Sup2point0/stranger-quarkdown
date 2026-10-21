use crossterm::terminal;

use std::io;


pub struct RawMode;

impl RawMode
{
	pub fn start() -> io::Result<Self>
	{
		terminal::enable_raw_mode()?;
		Ok(Self)
	}
}

impl Drop for RawMode {
	fn drop(&mut self) {
		let _ = terminal::disable_raw_mode();
	}
}
