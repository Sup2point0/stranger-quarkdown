use super::raw::*;
use crate::colours::*;

use crossterm::{
	cursor::{ self },
	event::{ self, Event, KeyEvent, KeyCode, KeyEventKind },
	terminal::{ Clear, ClearType },
	execute,
};

use std::io::{ self, Write };


macro_rules! wait
{
	() => {
		_wait_(None)
	};

	(
		$msg:literal $(, $($args:tt)* )?
	) => {
		_wait_(Some(&format!($msg, $($($args)* )?)))
	}
}
pub(crate) use wait;


pub(crate) fn _wait_(status_message: Option<&str>) -> io::Result<()>
{
	let mut stdout = io::stdout();
	
	print!(" {GREY}└  ");
	execute!(stdout, cursor::SavePosition)?;

	print!("{}", status_message.unwrap_or(
		"any key to continue, ESC to exit"
	));

	stdout.flush()?;
	execute!(stdout, cursor::RestorePosition)?;

	let _raw_mode = RawMode::start()?;

	loop {
		let e = event::read()?;

		if let Event::Key(KeyEvent{
			code,
			kind: KeyEventKind::Press,
			..
		}) = e
		{
			execute!(
				stdout,
				Clear(ClearType::CurrentLine),
				cursor::MoveToColumn(0),
			)?;

			if code == KeyCode::Esc {
				return Err(io::Error::from(io::ErrorKind::Interrupted));
			} else {
				return Ok(());
			}
		}
	}
}
