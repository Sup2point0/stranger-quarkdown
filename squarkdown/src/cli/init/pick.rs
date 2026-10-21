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


macro_rules! choices
{
	(  $variant:ident $(, $rest:ident)* $(,)?  ) =>
	{
		#[derive(Clone, Copy, Debug, PartialEq, Eq, enum_display::EnumDisplay)]
		#[enum_display(case = "Kebab")]
		enum _Choice {
			$variant,
			$($rest,)*
		}

		use _Choice::$variant;
		$(use _Choice::$rest;)*
	}
}
pub(super) use choices;


macro_rules! pick_one
{
	(
		$($value:expr => $hint:literal ;)+
	) => {
		_pick_(
			&[ $(($value, $value, $hint),)+ ],
			false
		)
	};

	(
		$($value:expr => ($label:literal, $hint:literal) ;)+
	) => {
		_pick_(
			&[ $(($value, $label, $hint),)+ ],
			false
		)
	};

	(
		$choices:expr
	) => {
		_pick_(
			&$choices.iter().map(|choice| (choice, choice, "")).collect::<Vec<_>>(),
			false
		)
	};
}
pub(super) use pick_one;


pub(super) fn _pick_<T: Clone>(
	choices: &[(T, impl Display, &str)],
	allow_multiple: bool,
) -> io::Result<T>
{
	let mut stdout = io::stdout();

	let mut idx = 0;

	loop {
		for (i, (_, label, hint)) in choices.iter().enumerate() {
			if i == idx {
				out!("{C}● {W}{label} {GREY}{ITAL}{hint}{UNITAL}");
			} else {
				out!("{GREY}○ {label}");
			}
		}

		let e = event::read()?;

		if let Event::Key(KeyEvent{
			code,
			kind: KeyEventKind::Press,
			..
		}) = e
		{
			match code
			{
				KeyCode::Esc => {
					execute!(stdout,
						cursor::MoveUp(choices.len() as u16),
						Clear(ClearType::FromCursorDown),
					)?;

					out!("{R}cancelled");
					return Err(io::Error::from(io::ErrorKind::Interrupted));
				}
				KeyCode::Down | KeyCode::Right => {
					idx += 1;
					if idx == choices.len() {
						idx = 0;
					}
				}
				KeyCode::Up | KeyCode::Left => {
					if idx == 0 {
						idx = choices.len() - 1;
					} else {
						idx -= 1
					}
				}
				KeyCode::Enter => break,
				_ => {}
			}
		}

		execute!(stdout,
			cursor::MoveUp(choices.len() as u16),
			Clear(ClearType::FromCursorDown),
		)?;
	}

	execute!(stdout,
		cursor::MoveUp(choices.len() as u16),
		Clear(ClearType::FromCursorDown),
	)?;

	let (choice, label, _) = &choices[idx];
	out!("{GREY}{label}");

	Ok(choice.clone())
}
