use std::{fs, io};
use regex::Regex;
use std::io::Read;

use crate::models::{file_path::FilePath, student_project::StudentProjectSubmission};

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

    // unzip source into sources folder one by one
    for source in sources.iter() {
        match source {
            FilePath::Zip(path) => {
                if let Err(e) = run_command(&format!("unzip \"{}\" -d rp_workspace/sources/", path)) {
                    println!("[WD] Error extracting source into sources directory!\n{:#?}", e);
                    std::process::exit(1);
                };
            },
            _ => continue,
        }
    }
}

pub fn extract_submissions_from_sources() -> Vec<StudentProjectSubmission> {
    let folder_names: Vec<String> = match folder_names("./rp_workspace/sources") {
        Ok(s) => s,
        Err(e) => {
            println!("[WD] Error creating sources directory!\n{:#?}", e);
            std::process::exit(1);
        }
    };
    extract_repos_form_folders(folder_names)
}

fn extract_repos_form_folders(folder_names: Vec<String>) -> Vec<StudentProjectSubmission> {
    let mut submissions = vec![];
    let re = Regex::new(r#"<a href="(https://(github|gitlab)\.com/[^"]+)">"#).unwrap();
    for folder in folder_names.iter() {
        let mut subm = StudentProjectSubmission { 
            student_folder: folder.clone(), 
            git_repo: None ,
            has_two_commits: None,
            all_commits_compile: None,
        };
        // Construct the path to the HTML file
        let path = format!("./rp_workspace/sources/{}/onlinetext.html", folder);
        // Open the HTML file
        let mut file = match fs::File::open(path) {
            Ok(file) => file,
            Err(err) => {
                println!("Error opening file: {}", err);
                continue;
            }
        };
        // Read the contents of the HTML file into a string
        let mut html = String::new();
        if let Err(err) = file.read_to_string(&mut html) {
            println!("Error reading file: {}", err);
            continue;
        };
        // Extract the repository URL from the HTML file
        let capture = match re.captures(&html) {
            Some(capture) => capture,
            None => {
                println!("No repository URL found in HTML file");
                submissions.push(subm);
                continue;
            }
        };
        subm.git_repo = Some(format!("{}.git", capture[1].to_string()));
        submissions.push(subm);
    }
    submissions
}

fn folder_names(path: &str) -> Result<Vec<String>, io::Error> {
    Ok(fs::read_dir(path)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().unwrap().is_dir())
        .map(|entry| entry.file_name().into_string().unwrap())
        .collect::<Vec<String>>())
}