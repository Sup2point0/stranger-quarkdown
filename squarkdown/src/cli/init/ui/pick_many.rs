use super::raw::*;
use super::out::*;
use crate::colours::*;
use crate::utils::macros::*;

use crossterm::{
	cursor,
	event::{ self, Event, KeyEvent, KeyCode, KeyEventKind },
	terminal::{ Clear, ClearType },
	execute,
};

use std::fmt::Display;
use std::io;


/// `:: Result<Vec<String>>`
/// 
/// Allow the user to pick any number of entries from the given choices.
/// 
/// ```ignore
/// pick_many!(
///    ask: "Pick any number";
///    choices: {
///       [true]  choice1 => hint1;
///       [false] choice2 => hint2;
///    }
/// )?;
/// ```
/// 
/// # See Also
/// 
/// [`pick_many_or_skip`] to add a "Skip this question" choice.
macro_rules! pick_many
{
	(
		ask : $bef:literal $(, $($args1:tt)* )?;
		$($rest:tt)*
	) => {
		pick_many! {
			before: $bef $(, $($args1)* )?;
			after:  $bef $(, $($args1)* )?;
			$($rest)*
		}
	};

	(
		before  : $bef:literal $(, $($args1:tt)* )?;
		after   : $aft:literal $(, $($args2:tt)* )?;
		choices : {
			$(
				[$enabled_by_default:literal]
				$value:expr => $hint:literal;
			)+
		}
	) => {
		_pick_many_(
			format_args!($bef $(, $($args1)* )?),
			format_args!($aft $(, $($args2)* )?),
			vec![
				$(
					PickManyChoice {
						value: $value,
						enabled: $enabled_by_default,
						label: $value,
						hint: $hint,
					},
				)+
			]
		)
	};

	(
		before  : $bef:literal $(, $($args1:tt)* )?;
		after   : $aft:literal $(, $($args2:tt)* )?;
		choices : $choices:expr
	) => {
		_pick_many_(
			format_args!($bef $(, $($args1)* )?),
			format_args!($aft $(, $($args2)* )?),
			$choices.into_iter()
				.map(|(choice, hint)| PickManyChoice {
					label: format!("{choice}"),
					value: choice,
					enabled: false,
					hint,
				})
				.collect::<Vec<_>>()
		)
	};
}
pub(crate) use pick_many;


/// `:: Result<Option<Vec<String>>>`
/// 
/// Allow the user to pick any number of entries from the given choices, or skip (in which case `None` is returned).
/// 
/// ```ignore
/// pick_many_or_skip! {
///    [true]  choice1 => hint1;
///    [false] choice2 => hint2;
/// }?;
/// ```
/// 
/// # See Also
/// 
/// [`pick_many`] to remove the "Skip this question" choice.
macro_rules! pick_many_or_skip
{
	(
		ask : $bef:literal $(, $($args1:tt)* )?;
		$($rest:tt)*
	) => {
		pick_many_or_skip! {
			before: $bef $(, $($args1)* )?;
			after:  $bef $(, $($args1)* )?;
			$($rest)*
		}
	};

	(
		before  : $bef:literal $(, $($args1:tt)* )?;
		after   : $aft:literal $(, $($args2:tt)* )?;
		choices : {
			$(
				[$enabled_by_default:literal]
				$value:expr => $hint:literal;
			)+
		}
	) => {
		_pick_many_or_skip_(
			format_args!($bef $(, $($args1)* )?),
			format_args!($aft $(, $($args2)* )?),
			vec![
				$(
					PickManyChoice {
						value: $value,
						enabled: $enabled_by_default,
						label: $value,
						hint: $hint,
					},
				)+
			],
			true
		)
	};

	(
		before  : $bef:literal $(, $($args1:tt)* )?;
		after   : $aft:literal $(, $($args2:tt)* )?;
		choices : $choices:expr
	) => {
		_pick_many_or_skip_(
			format!($bef $(, $($args1)* )?),
			format!($aft $(, $($args2)* )?),
			$choices.into_iter()
				.map(|(choice, hint)| PickManyChoice {
					label: format!("{choice}"),
					value: choice,
					enabled: false,
					hint,
				})
				.collect::<Vec<_>>(),
			true
		)
	};
}
pub(crate) use pick_many_or_skip;


pub(crate) struct PickManyChoice<T, L: Display>
{
	pub value: T,
	pub enabled: bool,
	pub label: L,
	pub hint: &'static str,
}


pub(crate) fn _pick_many_<T: Clone>(
	before: impl Display,
	after: impl Display,
	choices: Vec<PickManyChoice<T, impl Display>>,
) -> io::Result<Vec<T>>
{
	_pick_many_or_skip_(before, after, choices, false)
		.map(|r| r.expect("Cannot return `None` if not `skippable`"))
}

/// # Pre-Conditions
/// 
/// - `choices` is non-empty.
pub(crate) fn _pick_many_or_skip_<T: Clone>(
	before: impl Display,
	after: impl Display,
	mut choices: Vec<PickManyChoice<T, impl Display>>,
	skippable: bool,
) -> io::Result<Option<Vec<T>>>
{
	let mut stdout = io::stdout();

	let mut idx = 0;
	let max_idx = choices.len() - if skippable {0} else {1};

	let _raw_mode = RawMode::start()?;
	execute!(stdout, cursor::Hide)?;

	let mut refresh = |is_final: bool| {
		execute!(stdout,
			/* NOTE: +1 cuz max_idx = len() - 1 */
			/* NOTE: +2 for "SPACE to ..." hint */
			/* NOTE: +1 if also clearing question line */
			cursor::MoveUp(max_idx as u16 + if is_final {4} else {3}),
			Clear(ClearType::FromCursorDown),
		)
	};

	ask!("{before}");

	#[allow(unused_assignments)]
	let mut is_skipping = false;
	
	loop {
		is_skipping = (skippable && idx == max_idx);

		for (
			i,
			PickManyChoice{ enabled, label, hint, .. }
		) in choices.iter().enumerate()
		{
			let is_selected = (i == idx);

			print!(" {GREY}│  ");

			if *enabled {
				print!("{}●  ", if is_skipping {GREY} else {C});
			} else {
				print!("{}○  ", if is_selected {C} else {GREY});
			}

			if is_selected {
				print!("{W}{label} {GREY}{ITAL}{hint}{UNITAL}");
			} else {
				print!("{GREY}{label}");
			}

			println!();
		}

		if skippable {
			print!(" {GREY}│  ");

			if is_skipping {
				print!("{C}●  ");
			} else {
				print!("○  ");
			}

			println!("{}Skip this question", if is_skipping {W} else {GREY});
		}

		out!();
		out!("└" => "{GREY}SPACE to select · ENTER to submit · ESC to exit");

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
					out!("{after}");
					out!("{R}{ITAL}cancelled{UNITAL}");
					
					return Err(io::Error::from(io::ErrorKind::Interrupted));
				}
				KeyCode::Down | KeyCode::Right => {
					idx += 1;
					if idx > max_idx {
						idx = 0;
					}
				}
				KeyCode::Up | KeyCode::Left => {
					if idx == 0 {
						idx = max_idx;
					} else {
						idx -= 1;
					}
				}
				KeyCode::Home => {
					idx = 0;
				}
				KeyCode::End => {
					idx = max_idx;
				}
				KeyCode::Char(' ') => {
					choices[idx].enabled = !choices[idx].enabled;
				}
				KeyCode::Enter => break,
				_ => {}
			}
		}

		refresh(false)?;
	}

	refresh(true)?;
	out!("{after}");

	if is_skipping {
		out!("{R}{ITAL}skipped{UNITAL}");
		return Ok(None);
	}

	let choices = choices.iter().filter(|c| c.enabled);

	let mut labels =
		choices.clone()
		.map(|c| c.label.to_string())
		.collect::<Vec<_>>()
		.join(" · ")
	;
	if labels.is_empty() {
		labels = str!("none");
	}
	out!("{GREY}{labels}");

	execute!(stdout, cursor::Show)?;

	Ok(Some(
		choices.map(|c| c.value.clone()).collect()
	))
}
