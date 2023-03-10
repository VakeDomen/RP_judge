use std::{fs, path::Path};

use crate::models::student_project::StudentProjectSubmission;

use super::{os_helper::folder_names, parser::escape};

pub fn check_workdir() -> bool {
    let wd_meta = match fs::metadata("./rp_workspace") {
        Ok(m) => m,
        Err(e) => {
            println!("Error parsing path (./rp_workspace): {}", e);
            return false;
        },
    };

    // check if path points to a directory
    wd_meta.is_dir()
}

pub fn check_dir_exists(dir_name: &str) -> bool {
    let wd_meta = match fs::metadata(dir_name) {
        Ok(m) => m,
        Err(e) => {
            println!("Error parsing path ({}): {}", dir_name, e);
            return false;
        },
    };

    // check if path points to a directory
    wd_meta.is_dir()
}

pub fn find_main_file(folder_name: &str) -> Option<String> {
    let path = Path::new(folder_name);
    if !path.is_dir() {
        return None;
    }

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let file_name = entry.file_name().into_string().unwrap();
        if file_name == "main.c" {
            return Some(file_name);
        }
    }

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let file_name = entry.file_name().into_string().unwrap();
        if file_name.ends_with(".c") {
            return Some(file_name);
        }
    }

    None
}

pub fn find_accepted_folder(folder_name: &str, accepted_names: &[&str]) -> Option<String> {
    let folders = match folder_names(folder_name) {
        Ok(f) => f,
        Err(_) => return None,
    };

    for folder in folders {
        if accepted_names.contains(&folder.as_str()) {
            return Some(folder);
        }

        if let Some(found_folder) = find_accepted_folder(&format!("{}/{}", folder_name, folder), accepted_names) {
            let path = format!("{}/{}", folder, found_folder);
            return Some(path);
        }
    }

    None
}

pub fn tasks_to_check(submission: &StudentProjectSubmission) -> Vec<String> {
    let mut names = vec![];
    // find task 1 name
    if let Some(t1) = submission.has_task1.clone() {
        names.push(t1);
    }
    // find task 2 name
    if let Some(t2) = submission.has_task2.clone() {
        names.push(t2);
    }
    names
}