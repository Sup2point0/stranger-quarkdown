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


macro_rules! pick_many
{
	(
		$(
			[$enabled_by_default:literal]
			$value:expr
			=>
			$hint:literal
			;
		)+
	) => {
		_pick_many_(
			&mut [
				$(
					PickManyChoice {
						value: $value,
						enabled: $enabled_by_default,
						label: $value,
						hint: $hint,
						is_singleton: false,
					},
				)+
			]
		)
	};

	// (
	// 	$($value:expr => ($label:literal, $hint:literal) ;)+
	// ) => {
	// 	_pick_many_(
	// 		&[ $(($value, $label, $hint),)+ ]
	// 	)
	// };

	// (
	// 	$choices:expr
	// ) => {
	// 	_pick_many_(
	// 		&$choices.iter().map(|choice| (choice, choice, "")).collect::<Vec<_>>()
	// 	)
	// };
}
pub(crate) use pick_many;


pub(crate) struct PickManyChoice<T, L: Display>
{
	pub value: T,
	pub enabled: bool,
	pub label: L,
	pub hint: &'static str,
	pub is_singleton: bool,
}


pub(crate) fn _pick_many_<T: Clone>(
	choices: &mut [PickManyChoice<T, impl Display>],
) -> io::Result<Vec<T>>
{
	let mut stdout = io::stdout();

	let mut idx = 0;

	let _raw_mode = RawMode::start()?;
	
	loop {
		for (
			i,
			PickManyChoice{ value, enabled, label, hint, is_singleton }
		) in choices.iter_mut().enumerate()
		{
			let is_selected = (i == idx);

			print!(" {GREY}│  ");

			if *enabled {
				print!("{C}●  ");
			} else {
				if is_selected {
					print!("{C}");
				}
				print!("○  ");
			}

			if is_selected {
				print!("{W}{label} {GREY}{ITAL}{hint}{UNITAL}");
			} else {
				print!("{GREY}{label}");
			}

			println!();
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
						idx -= 1;
					}
				}
				KeyCode::Char(' ') => {
					choices[idx].enabled = !choices[idx].enabled;
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

	let choices = choices.iter().filter(|c| c.enabled);

	let labels =
		choices.clone()
		.map(|c| c.label.to_string())
		.collect::<Vec<_>>()
		.join(" · ")
	;
	out!("{GREY}{labels}");

	Ok(
		choices.map(|c| c.value.clone()).collect()
	)
}
