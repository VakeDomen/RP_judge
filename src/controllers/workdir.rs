use crate::models::file_path::FilePath;

use super::{validator::{check_workdir, check_dir_exists}, os_helper::{create_workdir, run_command}};

pub fn setup_workdir() {
    if let false = check_workdir() {
        if let Err(e) = create_workdir() {
            println!("[WD] Error creating working directory!\n{:#?}", e);
            std::process::exit(1);
        }
    }
}

pub fn move_sources(sources: Vec<FilePath>) {
    if sources.is_empty() {
        println!("[WD] No sources!\nPlease specify source file paths in arguments of the program. You can point to the *.zip file downloaded from Moodle or the folder that contains files and folders extracted from the zip.");
        std::process::exit(0);
    }

    // make sources folder
    if let false = check_dir_exists("rp_workspace/sources") {
        if let Err(e) = run_command("mkdir rp_workspace/sources") {
            println!("[WD] Error creating sources directory!\n{:#?}", e);
            std::process::exit(1);
        };
    }

    // clear sources folder
    if let Err(e) = run_command("rm -rf rp_workspace/sources/*") {
        println!("[WD] Error creating sources directory!\n{:#?}", e);
        std::process::exit(1);
    };

    for source in sources.iter() {
        // make sources folder
        if let Err(e) = run_command(&format!("cp \"{}\" rp_workspace/sources/", source.get_path())) {
            println!("[WD] Error copying source into sources directory!\n{:#?}", e);
            std::process::exit(1);
        };
    }
}