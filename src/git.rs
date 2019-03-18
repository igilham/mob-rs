use std::process::{ Command, Output };
use std::io::Error as IOError;
use std::io::ErrorKind;
use std::str;

pub fn git(args: Vec<&str>) -> Result<String, IOError> {
    let output = call(args)?;
    unwrap_output(output)
}

fn unwrap_output(out: Output) -> Result<String, IOError> {
    let out_str_result = str::from_utf8(&out.stdout);
    return match out_str_result {
        Ok(out_str) => {
            Ok(String::from(out_str))
        },
        Err(err) => {
            Err(IOError::new(ErrorKind::Other, err))
        },
    };
}

fn call(args: Vec<&str>) -> Result<Output, IOError> {
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
