use crate::colours::*;

use crossterm::{
	cursor::{ self },
	event::{ self, Event, KeyCode, KeyEvent, KeyEventKind },
	terminal::{ Clear, ClearType },
	ExecutableCommand,
	execute,
};

use std::io::{ self, stdout, Write };


macro_rules! wait
{
	() => {
		_wait_(None)
	};

	(
		$msg:literal $(, $($args:tt)* )?
	) => {
		_wait_(Some(&format!($msg, $($($args)*)?)))
	}
}
pub(super) use wait;


pub(super) fn _wait_(status_message: Option<&str>) -> io::Result<()>
{
	if let Some(msg) = status_message {
		print!("{GREY}{msg}");
	} else {
		print!("{GREY}any key to continue, ESC to exit");
	}

	stdout().flush()?;

	loop {
		let e = event::read()?;

		if let Event::Key(KeyEvent{
			code,
			kind: KeyEventKind::Release,
			..
		}) = e
		{
			execute!(
				stdout(),
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
