use super::raw::*;
use super::out::*;
use crate::colours::*;

use crossterm::{
	cursor,
	event::{ self, Event, KeyEvent, KeyCode, KeyEventKind },
	terminal::{ Clear, ClearType },
	execute,
};

use std::fmt::Display;
use std::io;


macro_rules! pick_one
{
	(
		ask : $msg:literal $(, $($args:tt)* )?;
		$($rest:tt)+
	) => {
		pick_one!(
			before: $msg $(, $($args)* )?;
			after:  $msg $(, $($args)* )?;
			$($rest)+
		)
	};

	(
		before  : $bef:literal $(, $($args1:tt)* )?;
		after   : $aft:literal $(, $($args2:tt)* )?;
		choices : {
			$(
				$value:expr => $hint:literal ;
			)+
		}
	) => {
		_pick_one_(
			format_args!($bef $(, $($args1)* )?),
			format_args!($aft $(, $($args2)* )?),
			&[ $(($value, $value, $hint),)+ ]
		)
	};

	(
		before  : $bef:literal $(, $($args1:tt)* )?;
		after   : $aft:literal $(, $($args2:tt)* )?;
		choices : {
			$(
				$value:expr => ($label:literal, $hint:literal) ;
			)+
		}
	) => {
		_pick_one_(
			format_args!($bef $(, $($args1)* )?),
			format_args!($aft $(, $($args2)* )?),
			&[ $(($value, $label, $hint),)+ ]
		)
	};

	(
		before  : $bef:literal $(, $($args1:tt)* )?;
		after   : $aft:literal $(, $($args2:tt)* )?;
		choices : $choices:expr
	) => {
		_pick_one_(
			format_args!($bef $(, $($args1)* )?),
			format_args!($aft $(, $($args2)* )?),
			&$choices.iter()
				.map(|choice| (choice, choice, ""))
				.collect::<Vec<_>>()
		)
	};
}
pub(crate) use pick_one;


pub(crate) fn _pick_one_<T: Clone>(
	before: impl Display,
	after: impl Display,
	choices: &[(T, impl Display, &str)],
) -> io::Result<T>
{
	let mut stdout = io::stdout();

	let mut idx = 0;

	let _raw_mode = RawMode::start()?;
	execute!(stdout, cursor::Hide)?;

	let mut refresh = |is_final: bool| {
		execute!(stdout,
			cursor::MoveUp(choices.len() as u16 + if is_final {1} else {0}),
			Clear(ClearType::FromCursorDown),
		)
	};

	ask!("{before}");
	
	loop {
		for (i, (_, label, hint)) in choices.iter().enumerate() {
			if i == idx {
				out!("{C}● {W}{label} {GREY}{ITAL}{hint}{UNITAL}");
			} else {
				out!("{GREY}○ {label}");
			}
		}

		if let Event::Key(KeyEvent{
			code,
			kind: KeyEventKind::Press,
			..
		}) = event::read()?
		{
			match code
			{
				KeyCode::Esc => {
					refresh(true)?;
					out!("{R}{ITAL}cancelled{UNITAL}");
					return Err(io::Error::from(io::ErrorKind::Interrupted));
				}
				KeyCode::Down | KeyCode::Right | KeyCode::Char(' ') => {
					idx += 1;
					if idx == choices.len() {
						idx = 0;
					}
				}
				KeyCode::Up | KeyCode::Left => {
					if idx == 0 {
						idx = choices.len() - 1;
					} else {
						idx -= 1;
					}
				}
				KeyCode::Home => {
					idx = 0;
				}
				KeyCode::End => {
					idx = choices.len() - 1;
				}
				KeyCode::Enter => break,
				_ => {}
			}
		}

		refresh(false)?;
	}

	refresh(true)?;
	out!("{C}✓" => "{after}");

	let (choice, label, _) = &choices[idx];
	out!("{GREY}{label}");

	execute!(stdout, cursor::Show)?;

	Ok(choice.clone())
}
