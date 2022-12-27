use std::env;

use controllers::workdir::{move_sources, extract_submissions_from_sources};
use models::student_project::StudentProjectSubmission;

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
    let submissions: Vec<StudentProjectSubmission> = extract_submissions_from_sources();
    println!("Submissions: {:#?}", submissions);
}

