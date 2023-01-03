use std::{io::{Error, ErrorKind, self}, process::Command, fs};

pub fn run_command(command: &str) -> Result<String, Error> {
    let output =  match Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
    {
        Ok(p) => p,
        Err(e) => return Err(Error::new(
            ErrorKind::Other, 
            format!("[OS HELPER] Something went wrong with running command: {}", e))
        ),
    };
    match output.status.success() {
        true => Ok(String::from_utf8_lossy(&output.stdout).to_string()),
        false => Err(Error::new(
            ErrorKind::Other, 
            format!("[OS HELPER] Something went wrong with running command: {}", String::from_utf8_lossy(&output.stderr)))
        ),
    }
}

pub fn create_workdir() -> Result<String, Error>  {
    println!("[OS HELPER] Creating working directory!");
    run_command("mkdir rp_workspace")
}

pub fn folder_names(path: &str) -> Result<Vec<String>, io::Error> {
    Ok(fs::read_dir(path)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().unwrap().is_dir())
        .map(|entry| entry.file_name().into_string().unwrap())
        .collect::<Vec<String>>())
}