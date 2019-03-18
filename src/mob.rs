use clap::{ ArgMatches };
use std::io::Error as IOError;
use std::io::ErrorKind;
use std::str;

use super::git::*;

pub const MOBBING_BRANCH_NAME: &str = "mob-session";

pub fn reset(_matches: &ArgMatches) -> Result<(), IOError> {
    git(vec!("fetch", "--prune"))?;
    git(vec!("checkout", "master"))?;
    if has_mobbing_branch()? {
        git(vec!("branch", "-D", MOBBING_BRANCH_NAME))?;
    }
    if has_mobbing_branch_origin()? {
        git(vec!("push", "origin", "--delete", MOBBING_BRANCH_NAME))?;
    }

    Ok(())
}

pub fn start(_matches: &ArgMatches) -> Result<(), IOError> {
    Err(IOError::from(ErrorKind::Other))
}

pub fn next(_matches: &ArgMatches) -> Result<(), IOError> {
    Err(IOError::from(ErrorKind::Other))
}

pub fn status(_matches: &ArgMatches) -> Result<(), IOError> {
    Err(IOError::from(ErrorKind::Other))
}

pub fn done(_matches: &ArgMatches) -> Result<(), IOError> {
    Err(IOError::from(ErrorKind::Other))
}

fn has_mobbing_branch() -> Result<bool, IOError> {
    let output = git(vec!("branch"))?;

	return if output.contains(&format!("  {}", MOBBING_BRANCH_NAME)) || output.contains(&format!("* {}", MOBBING_BRANCH_NAME)) {
        Ok(true)
    } else {
        Ok(false)
    }
}

fn has_mobbing_branch_origin() -> Result<bool, IOError> {
    let output = git(vec!("branch", "--remotes"))?;

	return if output.contains(&format!("  origin/{}", MOBBING_BRANCH_NAME)) || output.contains(&format!("* origin/{}", MOBBING_BRANCH_NAME)) {
        Ok(true)
    } else {
        Ok(false)
    }
}
