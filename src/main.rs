use std::env;

use controllers::workdir::{move_sources, extract_submissions_from_sources};
use models::student_project::StudentProjectSubmission;

use crate::controllers::exporter::export_to_xlsx;
use crate::controllers::git_clone_handler::clone_repos;
use crate::controllers::git_commit_handler::{extract_commits, check_structure, check_latest_commit_date};
use crate::controllers::git_compilation_handler::compile_commits;
use crate::controllers::moss_handler::setup_moss_folders;
use crate::controllers::parser::{parse_file_args, escape};
use crate::controllers::workdir::setup_workdir;
use crate::models::file_path::FilePath;

mod controllers;
mod models;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_paths: Vec<FilePath> = parse_file_args(args);
    
    println!("[MAIN] Setting up workspace...");
    setup_workdir();
    println!("\tDone!");

    println!("[MAIN] Moving sources to workspace...");
    move_sources(&file_paths);
    println!("\tDone!");

    println!("[MAIN] Extracting git repo links from submissions...");
    let mut submissions: Vec<StudentProjectSubmission> = extract_submissions_from_sources();
    println!("\tDone!");
        
    println!("[MAIN] Cloning git repos...");
    clone_repos(&mut submissions, &file_paths);
    println!("\tDone!");
    
    println!("[MAIN] Checking git repo structure...");
    check_structure(&mut submissions);
    println!("\tDone!");
    
    println!("[MAIN] Extracting commits...");
    extract_commits(&mut submissions);
    println!("\tDone!");

    println!("[MAIN] Extracting latest commit date...");
    check_latest_commit_date(&mut submissions);
    println!("\tDone!");

    println!("[MAIN] Compiling commits...");
    compile_commits(&mut submissions);
    println!("\tDone!");

    println!("[MAIN] Preparing moss submission folders");
    setup_moss_folders(&submissions);
    println!("\nDone!");

    println!("[MAIN] Exporting submissions...");
    match export_to_xlsx(submissions, "./rp_workspace/results.xlsx") {
        Ok(_) => println!("\tDone!"),
        Err(e) => println!("[MAIN] Error! Something went wrong exporting results: {:#?}", e),
    };
}

