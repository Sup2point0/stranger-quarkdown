use super::*;
use crate::*;
use crate::prelude::*;
use crate::resolver;
use crate::log;
use crate::colours::*;

use git2 as git;

use std::fs;
use std::path::{ Path, PathBuf };
use std::process::{ ExitCode };
use std::time::{ Instant };


/// Delete Squarkdown's output.
/// 
/// This reads the user's squarkup config to find the appropriate folders and files to check, and reconciles it with Git to avoid deleting tracked files that probably shouldn't be deleted.
pub fn clean(args: impl Iterator<Item = String>) -> ExitCode
{
	let mut force = false;
	let mut unknown_args = vec![];

	for arg in args {
		if arg.eq_ignore_ascii_case("--force")
		|| arg.eq_ignore_ascii_case("-f") {
			force = true;
		}
		else {
			unknown_args.push(arg);
		}
	}

	if !unknown_args.is_empty() {
		shared::unknown_args(&unknown_args);
		return ExitCode::FAILURE;
	}

	log::version();
	log::is!("cleaning Squarkdown output...");

	let t_init = Instant::now();
	let r = cleanup(force);
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

fn cleanup(force: bool) -> SquarkResult
{
	let project_root = resolver::resolve_project_root()?;
	log::ok!(slash!("found your project: {B}{}", project_root));

	let config = resolver::resolve_config(&project_root)?;
	log::ok!(slash!("found your output folder: {B}{}", &config.out.folder));

	log::is!("checking for Git repository...");
	let mut repo_info: Option<(git::Repository, PathBuf)> = None;

	match resolver::resolve_project_repo(&config.out.folder, &config.paths.root) {
		Ok(repo) => {
			log::ok!(slash!("found your Git repository: {B}{}", repo.path()));

			let repo_folder = repo.path().parent()
				.expect(".git folder always has a parent folder")
				.to_path_buf();

			repo_info = Some((repo, repo_folder));
		}
	   Err(mut e) => if force {
			log::bad!("no repository found, proceeding with {W}{BOLD}--force{UNBOLD}");
		} else {
			e.set_hint(fmt!("to cleanup without checking Git, pass {Y}{BOLD}--force{UNBOLD}"));
			return Err(e);
		}
	}

	log::info!(slash!("deleting files under: {}", &config.out.folder));

	let mut errs = SquarkError::multiple("cleaning files");

	/* NOTE: walkdir walker always returns `out.folder` itself, so we don't count that. */
	let mut checked_paths = -1;
	let mut deleted_paths = 0;
	let mut skipped_paths = vec![];

	let walker = walkdir::WalkDir::new(&config.out.folder)
		.contents_first(true)
		.into_iter()
	;

	for entry in walker {
		checked_paths += 1;

		catch!(errs =>
		{
			let entry = entry?;
			let path = entry.into_path();

			if let Some((_, root)) = &repo_info && path.strip_prefix(root).is_err() {
				return Err(SquarkError::Unrecoverable {
					msg: fmt!("critical error: received a path outside your project root"),
					hint: str!(),
					debug: vec![
						slash!("would have tried deleting: {GREY1}{}", path)
					],
				});
			}

			let path_rel = utils::display_rel(&path, &config.paths.root);

			// delete folder
			if path.is_dir() && fs::read_dir(&path)?.next().is_none()
			{
				let (parent, folder) = utils::split_path(&path_rel);
				log::info!("deleting: {GREY1}{parent}/{folder}");

				fs::remove_dir(path)?;
				deleted_paths += 1;
				return Ok(());
			}

			// delete file
			let Some(filename) = path.file_name() else {
				return Ok(());
			};
			if *filename != *config.out.file_name && *filename != *"+page.ts" {
				return Ok(());
			}

			let (folder, filename) = utils::split_path(&path_rel);
			let path_display = fmt!("{GREY1}{folder}{GREY}/{filename}");
			
			// check Git safety
			if let Some((repo, repo_folder)) = &repo_info
			{	
				if !is_safe_to_delete(&path, repo, &repo_folder)? {
					skipped_paths.push(fmt!("skipped: {path_display}"));
					return Ok(());
				}
			}

			log::info!("deleting: {path_display}");
			fs::remove_file(path)?;
			deleted_paths += 1;
		});
	}

	if checked_paths == 0 {
		log::bad!("nothing found to cleanup!");
	} else {
		log::ok!("deleted {}/{} children", deleted_paths, checked_paths);
	}

	if skipped_paths.len() > 0 {
		errs.push(SquarkError::Recoverable {
			msg: fmt!("skipped {} files that are not Git-ignored", skipped_paths.len()),
			hint: fmt!("to cleanup without checking Git, pass {Y}{BOLD}--force{UNBOLD}"),
			debug: skipped_paths,
		});
	}
	
	errs.or(())
}

/// Is `path` a fully uncommitted, untracked, Git-ignored file that is safe to delete?
fn is_safe_to_delete(
	path: &Path,
	repo: &git::Repository,
	repo_folder: &Path,
) -> SquarkResult<bool>
{
	let path_rel = path.strip_prefix(repo_folder)
		.expect("walkdir entries are guaranteed to be under `repo.path()`");

	let ignored = repo.is_path_ignored(&path)?;
	let tracked = repo.index()?.get_path(path_rel, 0).is_some();
	let modified = repo.status_file(path_rel)?.intersects(
		  git::Status::INDEX_NEW
		| git::Status::INDEX_MODIFIED
	);

	/* NOTE: A git-ignored file can still be tracked if it was committed before the `.gitignore` rules excluded it, so we need to check both. */
	Ok(
			ignored
		&& !tracked
		&& !modified
	)
}
