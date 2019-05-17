//! Bootstrap a new lorri project

use crate::ops::{ok, ok_msg, ExitError, OpResult};
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

fn create_if_missing(path: &Path, contents: &str) -> Result<(), io::Error> {
    if path.exists() {
        println!("- Skipping existing {}", path.display());
        Ok(())
    } else {
        let mut f = File::create(path)?;
        f.write_all(contents.as_bytes())?;
        println!("- Writing {}", path.display());
        Ok(())
    }
}

fn to_op(e: Result<(), io::Error>) -> OpResult {
    match e {
        Ok(_) => ok(),
        Err(e) => ExitError::errmsg(format!("{}", e)),
    }
}

/// See the documentation for lorri::cli::Command::Init for
/// more details
pub fn main(default_shell: &str, default_envrc: &str) -> OpResult {
    to_op(create_if_missing(Path::new("./shell.nix"), default_shell))?;
    to_op(create_if_missing(Path::new("./.envrc"), default_envrc))?;

    ok_msg(String::from("\nSetup done."))
}
