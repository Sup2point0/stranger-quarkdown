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
/// Allow the user to pick any number (even 0) of entries from the given choices.
/// 
/// ```ignore
/// pick_many!(
///    ask: "Pick any options";
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
		ask : $msg:literal $(, $($args:tt)*)?;
		$($rest:tt)+
	) => {
		pick_many! {
			before: $msg $(, $($args)*)?;
			after:  $msg $(, $($args)*)?;
			$($rest)+
		}
	};

	(
		before  : $bef:literal $(, $($args1:tt)*)?;
		after   : $aft:literal $(, $($args2:tt)*)?;
		choices : {
			$(
				[$enabled_by_default:literal]
				$value:expr => $hint:literal ;
			)+
		}
	) => {
		_pick_many_(
			format_args!($bef $(, $($args1)*)?),
			format_args!($aft $(, $($args2)*)?),
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
		before  : $bef:literal $(, $($args1:tt)*)?;
		after   : $aft:literal $(, $($args2:tt)*)?;
		choices : $choices:expr
	) => {
		_pick_many_(
			format_args!($bef $(, $($args1)*)?),
			format_args!($aft $(, $($args2)*)?),
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
/// Allow the user to pick any number (even 0) of entries from the given choices, or skip (in which case `None` is returned).
/// 
/// ```ignore
/// pick_many_or_skip!(
///    ask: "Pick any options";
///    choices: {
///       [true]  choice1 => hint1;
///       [false] choice2 => hint2;
///    }
/// )?;
/// ```
/// 
/// # See Also
/// 
/// [`pick_many`] to remove the "Skip this question" choice.
macro_rules! pick_many_or_skip
{
	(
		ask : $msg:literal $(, $($args:tt)*)?;
		$($rest:tt)*
	) => {
		pick_many_or_skip! {
			before: $msg $(, $($args)*)?;
			after:  $msg $(, $($args)*)?;
			$($rest)*
		}
	};

	(
		before  : $bef:literal $(, $($args1:tt)*)?;
		after   : $aft:literal $(, $($args2:tt)*)?;
		choices : {
			$(
				[$enabled_by_default:literal]
				$value:expr => $hint:literal ;
			)+
		}
	) => {
		_pick_many_or_skip_(
			format_args!($bef $(, $($args1)*)?),
			format_args!($aft $(, $($args2)*)?),
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
		before  : $bef:literal $(, $($args1:tt)*)?;
		after   : $aft:literal $(, $($args2:tt)*)?;
		choices : $choices:expr
	) => {
		_pick_many_or_skip_(
			format!($bef $(, $($args1)*)?),
			format!($aft $(, $($args2)*)?),
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
/// - `choices` is non-empty (guaranteed by macro).
pub(crate) fn _pick_many_or_skip_<T: Clone>(
	before: impl Display,
	after: impl Display,
	mut choices: Vec<PickManyChoice<T, impl Display>>,
	skippable: bool,
) -> io::Result<Option<Vec<T>>>
{
	debug_assert!(!choices.is_empty());
	
	let mut stdout = io::stdout();

	let mut idx = 0;
	let max_idx = choices.len() - if skippable {0} else {1};

	let _raw_mode = RawMode::start()?;
	execute!(stdout, cursor::Hide)?;

	macro_rules! reset_cursor {
		(  finish : $is_final:expr  ) =>
		{
			if $is_final {
				execute!(stdout,
					/* NOTE: +1 cuz max_idx = len() - 1 */
					/* NOTE: +2 for "SPACE to ..." hint */
					/* NOTE: +1 if also clearing question line */
					cursor::MoveUp(max_idx as u16 + 4),
					Clear(ClearType::FromCursorDown),
				)
			} else {
				execute!(stdout,
					cursor::MoveUp(max_idx as u16 + 3),
					// Clearing on a normal refresh is left to each individual line
				)
			}
		};
	}

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
				print!("{}● ", if is_skipping {GREY} else {C});
			} else {
				print!("{}○ ", if is_selected {C} else {GREY});
			}

			if is_selected {
				print!("{W}{label} {GREY}{ITAL}{hint}{UNITAL}");
			} else if *enabled {
				print!("{GREY1}{label}");
			} else {
				print!("{GREY}{label}");
			}

			execute!(stdout, Clear(ClearType::UntilNewLine))?;
			println!();
		}

		if skippable {
			println!(
				" {GREY}│  {col1}{dot} {col2}Skip this question",
				col1 = if is_skipping {C} else {""},
				dot = if is_skipping {"●"} else {"○"},
				col2 = if is_skipping {W} else {GREY},
			);
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
					reset_cursor!(finish: true)?;
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
					if !is_skipping {
						choices[idx].enabled = !choices[idx].enabled;
					}
				}
				KeyCode::Enter => break,
				_ => {}
			}
		}

		reset_cursor!(finish: false)?;
	}

	reset_cursor!(finish: true)?;

	if is_skipping {
		out!("{R}×" => "{after}");
		out!("{R}{ITAL}skipped{UNITAL}");
		execute!(stdout, cursor::Show)?;
		return Ok(None);
	}
	
	out!("{C}✓" => "{after}");

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
