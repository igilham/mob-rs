use clap::{ ArgMatches };
use std::io::Error as IOError;
use std::io::ErrorKind;
use std::str;

use super::git::*;

pub const MOBBING_BRANCH: &str = "mob-session";
pub const MASTER_BRANCH: &str = "master";
pub const COMMIT_MESSAGE: &str = "\"Mob Session DONE [ci-skip]\"";

pub fn reset(_matches: &ArgMatches) -> Result<(), IOError> {
    git(vec!("fetch", "--prune"))?;
    git(vec!("checkout", "master"))?;
    if has_mobbing_branch()? {
        git(vec!("branch", "-D", MOBBING_BRANCH))?;
    }
    if has_mobbing_branch_origin()? {
        git(vec!("push", "origin", "--delete", MOBBING_BRANCH))?;
    }

    Ok(())
}

pub fn start(_matches: &ArgMatches) -> Result<(), IOError> {
    if !is_nothing_to_commit()? {
		println!("uncommitted changes");
		return Ok(());
	}

	git(vec!("fetch", "--prune"))?;

	if has_mobbing_branch()? && has_mobbing_branch_origin()? {
		println!("rejoining mob session");
		git(vec!("checkout", MOBBING_BRANCH))?;
		git(vec!("merge", format!("origin/{}", MOBBING_BRANCH).as_str(), "--ff-only"))?;
		git(vec!("branch", format!("--set-upstream-to=origin/{}", MOBBING_BRANCH).as_str(), MOBBING_BRANCH))?;
	} else if !has_mobbing_branch()? && !has_mobbing_branch_origin()? {
		println!("create {} from master", MOBBING_BRANCH);
		git(vec!("checkout", MASTER_BRANCH))?;
		git(vec!("merge", "origin/master", "--ff-only"))?;
		git(vec!("branch", MOBBING_BRANCH))?;
		git(vec!("checkout", MOBBING_BRANCH))?;
		git(vec!("push", "--set-upstream", "origin", MOBBING_BRANCH))?;
	} else if !has_mobbing_branch()? && has_mobbing_branch_origin()? {
		println!("joining mob session");
		git(vec!("checkout", MOBBING_BRANCH))?;
	} else {
		println!("purging local branch and start new {} branch from {}", MOBBING_BRANCH, MASTER_BRANCH);
		git(vec!("branch", "-D", MOBBING_BRANCH))?; // check if unmerged commits

		git(vec!("checkout", MASTER_BRANCH))?;
		git(vec!("merge", "origin/master", "--ff-only"))?;
		git(vec!("branch", MOBBING_BRANCH))?;
		git(vec!("checkout", MOBBING_BRANCH))?;
		git(vec!("push", "--set-upstream", "origin", MOBBING_BRANCH))?;
	}
    Ok(())
}

pub fn next(_matches: &ArgMatches) -> Result<(), IOError> {
    if !is_mobbing()? {
		eprintln!("You aren't mobbing");
		return Err(IOError::from(ErrorKind::Other));
	}

	if is_nothing_to_commit()? {
		println!("Nothing was done, so nothing to commit");
	} else {
		git(vec!("add", "--all"))?;
		git(vec!("commit", "--message", "\"chore: wip in mob session [ci-skip]\""))?;
		git(vec!("push", "origin", MOBBING_BRANCH))?;
	}

	git(vec!("checkout", MASTER_BRANCH))?;
    Ok(())
}

pub fn status(_matches: &ArgMatches) -> Result<(), IOError> {
    if is_mobbing()? {
		println!("Mobbing in progress");

		let output = git(vec!("--no-pager", "log", format!("{}..{}", MASTER_BRANCH, MOBBING_BRANCH).as_str(), "--pretty=format:%h %cr <%an>", "--abbrev-commit"))?;
		println!("{}", output);
	} else {
		println!("You aren't mobbing right now");
	}
    Ok(())
}

pub fn done(_matches: &ArgMatches) -> Result<(), IOError> {
    if !is_mobbing()? {
		eprintln!("You aren't mobbing");
		return Ok(());
	}

	git(vec!("fetch", "--prune"))?;

	if has_mobbing_branch_origin()? {
		if !is_nothing_to_commit()? {
			git(vec!("add", "--all"))?;
			git(vec!("commit", "--message", COMMIT_MESSAGE))?;
		}
		git(vec!("push", "origin", MOBBING_BRANCH))?;

		git(vec!("checkout", MASTER_BRANCH))?;
		git(vec!("merge", "--squash", MOBBING_BRANCH))?;

		git(vec!("branch", "-D", MOBBING_BRANCH))?;
		git(vec!("push", "origin", "--delete", MOBBING_BRANCH))?;

		println!("TODO: git commit -m 'describe the changes'");
	} else {
		git(vec!("checkout", MASTER_BRANCH))?;
		git(vec!("branch", "-D", MASTER_BRANCH))?;
		println!("Someone else already ended your mob session");
	}
    Ok(())
}

fn has_mobbing_branch() -> Result<bool, IOError> {
    let output = git(vec!("branch"))?;

	return if output.contains(&format!("  {}", MOBBING_BRANCH)) || output.contains(&format!("* {}", MOBBING_BRANCH)) {
        Ok(true)
    } else {
        Ok(false)
    }
}

fn has_mobbing_branch_origin() -> Result<bool, IOError> {
    let output = git(vec!("branch", "--remotes"))?;

	Ok(output.contains(&format!("  origin/{}", MOBBING_BRANCH)) || output.contains(&format!("* origin/{}", MOBBING_BRANCH)))
}

fn is_mobbing() -> Result<bool, IOError> {
    let output = git(vec!("branch"))?;

    Ok(output.contains(&format!("* {}", MOBBING_BRANCH)))
}

fn is_nothing_to_commit() -> Result<bool, IOError> {
    let output = git(vec!("status", "--short"))?;

    Ok(output.trim().len() == 0)
}
