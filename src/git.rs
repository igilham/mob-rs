use std::process::{ Command, Output };
use std::io::Error as IOError;

pub fn git(args: Vec<&str>) -> Result<Output, IOError> {
    if cfg!(target_os = "windows") {
        return Command::new("cmd")
            .arg("/C")
            .arg("git")
            .args(args)
            .output();
    }
    return Command::new("sh")
        .arg("-c")
        .arg("git")
        .args(args)
        .output();
}
