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


/// `:: str | (str, str) -> Map T str -> Result T`
/// 
/// Allow the user to pick exactly one entry from the given choices.
/// 
/// ```ignore
/// pick_one!(
///    ask: "Pick one option";
///    choices: {
///       choice1 => hint1;
///       choice2 => hint2;
///    }
/// )?;
/// ```
/// 
/// # See Also
/// 
/// [`pick_one_or_skip`] to add a "Skip this question" choice.
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
			vec![
				$(
					PickOneChoice {
						value: $value,
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
		choices : {
			$(
				$value:expr => ($label:literal, $hint:literal) ;
			)+
		}
	) => {
		_pick_one_(
			format_args!($bef $(, $($args1)* )?),
			format_args!($aft $(, $($args2)* )?),
			vec![
				$(
					PickOneChoice {
						value: $value,
						label: $label,
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
		_pick_one_(
			format_args!($bef $(, $($args1)* )?),
			format_args!($aft $(, $($args2)* )?),
			$choices.iter()
				.map(|choice| PickOneChoice {
					label: choice,
					value: choice,
					hint: "",
				})
				.collect::<Vec<_>>()
		)
	};
}
pub(crate) use pick_one;


/// `:: str | (str, str) -> Map T str -> Result<Option T>`
/// 
/// Allow the user to pick exactly one entry from the given choices, or skip (in which case `None` is returned).
/// 
/// ```ignore
/// pick_one!(
///    ask: "Pick one option";
///    choices: {
///       choice1 => hint1;
///       choice2 => hint2;
///    }
/// )?;
/// ```
/// 
/// # See Also
/// 
/// [`pick_one`] to remove the "Skip this question" choice.
macro_rules! pick_one_or_skip
{
	(
		ask : $msg:literal $(, $($args:tt)*)?;
		$($rest:tt)*
	) => {
		pick_one_or_skip! {
			before: $msg $(, $($args)*)?;
			after:  $msg $(, $($args)*)?;
			$($rest)*
		}
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
		_pick_one_or_skip_(
			format_args!($bef $(, $($args1)* )?),
			format_args!($aft $(, $($args2)* )?),
			vec![
				$(
					PickOneChoice {
						value: $value,
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
		choices : {
			$(
				$value:expr => ($label:literal, $hint:literal) ;
			)+
		}
	) => {
		_pick_one_or_skip_(
			format_args!($bef $(, $($args1)* )?),
			format_args!($aft $(, $($args2)* )?),
			vec![
				$(
					PickOneChoice {
						value: $value,
						label: $label,
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
		_pick_one_or_skip_(
			format_args!($bef $(, $($args1)* )?),
			format_args!($aft $(, $($args2)* )?),
			$choices.iter()
				.map(|(choice, hint)| PickOneChoice {
					label: choice,
					value: choice,
					hint,
				})
				.collect::<Vec<_>>(),
			true
		)
	};
}
pub(crate) use pick_one_or_skip;


pub(crate) struct PickOneChoice<T, L: Display>
{
	pub value: T,
	pub label: L,
	pub hint: &'static str,
}


/// # Pre-Conditions
/// 
/// - `choices` is non-empty (guaranteed by macro).
pub(crate) fn _pick_one_<T: Clone>(
	before: impl Display,
	after: impl Display,
	choices: Vec<PickOneChoice<T, impl Display>>,
) -> io::Result<T>
{
	_pick_one_or_skip_(before, after, choices, false)
		.map(|r| r.expect("Cannot return `None` if not `skipable`"))
}


pub(crate) fn _pick_one_or_skip_<T: Clone>(
	before: impl Display,
	after: impl Display,
	choices: Vec<PickOneChoice<T, impl Display>>,
	skippable: bool,
) -> io::Result<Option<T>>
{
	let mut stdout = io::stdout();

	let mut idx = 0;
	let max_idx = choices.len() - if skippable {0} else {1};

	let _raw_mode = RawMode::start()?;
	execute!(stdout, cursor::Hide)?;

	let mut refresh = |is_final: bool| {
		execute!(stdout,
			/* NOTE: +1 cuz max_idx = len() - 1 */
			/* NOTE: +2 for "ENTER to ..." hint */
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

		for (i, PickOneChoice{ label, hint, .. }) in choices.iter().enumerate() {
			if i == idx {
				out!("{C}● {W}{label} {GREY}{ITAL}{hint}{UNITAL}");
			} else {
				out!("{GREY}○ {label}");
			}
		}

		if skippable {
			print!(" {GREY}│  ");

			if is_skipping {
				print!("{C}● ");
			} else {
				print!("○ ");
			}

			println!("{}Skip this question", if is_skipping {W} else {GREY});
		}

		out!();
		out!("└" => "{GREY}ENTER to submit · ESC to exit");

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
				KeyCode::Down | KeyCode::Right | KeyCode::Char(' ') => {
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
				KeyCode::Enter => break,
				_ => {}
			}
		}

		refresh(false)?;
	}

	refresh(true)?;

	if is_skipping {
		out!("{R}×" => "{after}");
		out!("{R}{ITAL}skipped{UNITAL}");
		execute!(stdout, cursor::Show)?;
		return Ok(None);
	}

	out!("{C}✓" => "{after}");

	let PickOneChoice{ value, label, .. } = &choices[idx];
	out!("{GREY}{label}");

	execute!(stdout, cursor::Show)?;

	Ok(Some(value.clone()))
}
