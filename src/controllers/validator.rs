use std::fs;

use super::os_helper::{folder_names, run_command};



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
    let wd_meta = match fs::metadata(dir_name.clone()) {
        Ok(m) => m,
        Err(e) => {
            println!("Error parsing path ({}): {}", dir_name, e);
            return false;
        },
    };

    // check if path points to a directory
    wd_meta.is_dir()
}

pub fn validate_and_fix_task_names(dir_name: &str) {
    let folder_names: Vec<String> = match folder_names(dir_name) {
        Ok(s) => s,
        Err(e) => {
            println!("[WD] Error fetching folder names!\n{:#?}", e);
            return;
        }
    };
    // fix task 1
    let accepted_folder_names = ["task1", "1task", "1Task", "task_1", "Task_1"];
    if !folder_names.contains(&"Task1".to_string()) {
        for accepted_name in accepted_folder_names.iter() {
            if folder_names.contains(&accepted_name.to_string()) {
                // rename folder to Task1
                if let Err(e) = run_command(&format!("mv {}/{} {}/Task1",dir_name, accepted_name, dir_name)) {
                    println!("[WD] Error renaming folder {} -> Task1!\n{:#?}", accepted_name, e);
                    continue;
                };
            }
        }
    }

    // fix task 2
    let accepted_folder_names = ["task2", "2task", "2Task", "task_2", "Task_2"];
    if !folder_names.contains(&"Task2".to_string()) {
        for accepted_name in accepted_folder_names.iter() {
            if folder_names.contains(&accepted_name.to_string()) {
                // rename folder to Task1
                if let Err(e) = run_command(&format!("mv {}/{} {}/Task2",dir_name, accepted_name, dir_name)) {
                    println!("[WD] Error renaming folder {} -> Task2!\n{:#?}", accepted_name, e);
                    continue;
                };
            }
        }
    }
}