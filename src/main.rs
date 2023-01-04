use std::env;

use controllers::workdir::{move_sources, extract_submissions_from_sources};
use models::student_project::StudentProjectSubmission;

use crate::controllers::exporter::export_to_xlsx;
use crate::controllers::git_handler::{clone_repos, check_structure, extract_commits, compile_commits};
use crate::controllers::parser::parse_file_args;
use crate::controllers::workdir::setup_workdir;
use crate::models::file_path::FilePath;

mod controllers;
mod models;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_paths: Vec<FilePath> = parse_file_args(args);
    
    print!("[MAIN] Setting up workspace...");
    setup_workdir();
    println!("\tDone!");

    print!("[MAIN] Moving sources to workspace...");
    move_sources(&file_paths);
    println!("\tDone!");

    print!("[MAIN] Extracting git repo links from submissions...");
    let mut submissions: Vec<StudentProjectSubmission> = extract_submissions_from_sources();
    println!("\tDone!");
    
    // if submissions.is_empty() {
    //     println!("[MAIN] No valid submissions to check!");
    //     std::process::exit(0);
    // }
    
    print!("[MAIN] Cloning git repos...");
    clone_repos(&mut submissions, &file_paths);
    println!("\tDone!");
    
    print!("[MAIN] Checking git repo structure...");
    check_structure(&mut submissions);
    println!("\tDone!");
    
    print!("[MAIN] Extracting commits...");
    extract_commits(&mut submissions);
    println!("\tDone!");

    print!("[MAIN] Compiling commits...");
    compile_commits(&mut submissions);
    println!("\tDone!");

    print!("[MAIN] Exporting submissions...");
    match export_to_xlsx(submissions, "./rp_workspace/results.xlsx") {
        Ok(_) => println!("\tDone!"),
        Err(e) => println!("[MAIN] Error! Something went wrong exporting results: {:#?}", e),
    };
    
    // println!("{:#?}", submissions);

}

