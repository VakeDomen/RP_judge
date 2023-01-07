use std::{fs, path::Path};

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