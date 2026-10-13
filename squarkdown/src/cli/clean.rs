use super::*;
use crate::*;
use crate::prelude::*;
use crate::log;
use crate::colours::*;
use crate::macros::*;

use git2 as git;

use std::process::ExitCode;
use std::time::Instant;


/// Delete Squarkdown's output.
/// 
/// This reads the user's squarkup config to find the appropriate folders and files to check, and reconciles it with Git to avoid deleting tracked files that probably shouldn't be deleted.
pub fn clean(args: impl Iterator<Item = String>) -> ExitCode
{
	let mut force = false;
	let mut unknown_args = vec![];

	for arg in args {
		if arg.eq_ignore_ascii_case("--force") { force = true; }
		else {
			unknown_args.push(arg);
		}
	}

	if !unknown_args.is_empty() {
		shared::unknown_args(&unknown_args);
		return ExitCode::FAILURE;
	}

	log::version();
	log::is!("deleting Squarkdown output...");

	let t_init = Instant::now();
	let r = go(force);
	let t = t_init.elapsed();
	let perf = t.as_secs_f64() * 1000.0;

	match r
	{
		Ok(()) => {
			log::line();
			println!("{P}cleanup finished! {GREY}{perf:.1?} ms{W}");
			ExitCode::SUCCESS
		},
		Err(e) => {
			log::error(e);
			println!("{R}cleanup failed! {GREY}{perf:.1?} ms\n{W}");
			ExitCode::FAILURE
		},
	}
}

fn go(force: bool) -> SquarkResult
{
	let project_root = resolver::resolve_project_root()?;
	log::ok!(slash!("found your project: {B}{}", project_root));

	let config = resolver::resolve_config(&project_root)?;
	log::ok!(slash!("found your output folder: {B}{}", &config.out.folder));

	log::is!("checking for Git repository...");

	let _ = catch! {
		let repo = git::Repository::open(".")?;
		let statuses = repo.statuses(None)?;

		for entry in &statuses {
			dbg!(entry.path());
		}
	};

	Ok(())
}
