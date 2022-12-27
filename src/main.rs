use std::env;

use controllers::workdir::{move_sources, extract_submissions_from_sources};
use models::student_project::StudentProjectSubmission;

use crate::controllers::git_handler::{clone_repos, check_structure, check_commits};
use crate::controllers::parser::parse_file_args;
use crate::controllers::workdir::setup_workdir;
use crate::models::file_path::FilePath;

mod controllers;
mod models;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_paths: Vec<FilePath> = parse_file_args(args);
    
    setup_workdir();
    move_sources(file_paths);
    let mut submissions: Vec<StudentProjectSubmission> = extract_submissions_from_sources();
    
    if submissions.is_empty() {
        println!("[MAIN] No valid submissions to check!");
        std::process::exit(0);
    }

    clone_repos(&mut submissions);
    check_structure(&mut submissions);
    check_commits(&mut submissions);
    println!("Submissions: {:#?}", submissions);
}

