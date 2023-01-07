use std::fs;
use regex::Regex;
use std::io::Read;

use crate::models::{file_path::FilePath, student_project::StudentProjectSubmission};

use super::{validator::{check_workdir, check_dir_exists}, os_helper::{create_workdir, run_command, folder_names}};

pub fn setup_workdir() {
    // create working directory
    if let false = check_workdir() {
        if let Err(e) = create_workdir() {
            println!("[WD] Error creating working directory!\n{:#?}", e);
            std::process::exit(1);
        }
    }

    // make sources folder
    if let false = check_dir_exists("rp_workspace/sources") {
        println!("[WD] Creating sources folder!");
        if let Err(e) = run_command("mkdir rp_workspace/sources") {
            println!("[WD] Error creating sources directory!\n{:#?}", e);
            std::process::exit(1);
        };
    }

    // clear sources folder
    if let Err(e) = run_command("rm -rf rp_workspace/sources/*") {
        println!("[WD] Error clearing sources directory!\n{:#?}", e);
        std::process::exit(1);
    };

    // make repos folder
    if let false = check_dir_exists("rp_workspace/repos") {
        println!("[WD] Creating repos folder!");
        if let Err(e) = run_command("mkdir rp_workspace/repos") {
            println!("[WD] Error creating repos directory!\n{:#?}", e);
            std::process::exit(1);
        };
    }

    // clear repos folder
    if let Err(e) = run_command("rm -rf rp_workspace/repos/*") {
        println!("[WD] Error clearing repos directory!\n{:#?}", e);
        std::process::exit(1);
    };
}

pub fn move_sources(sources: &Vec<FilePath>) {
    if sources.is_empty() {
        println!("[WD] No sources!\nPlease specify source file paths in arguments of the program. You can point to the *.zip file downloaded from Moodle or the folder that contains files and folders extracted from the zip.");
        std::process::exit(0);
    }
    // make temporary folder
    if let false = check_dir_exists("rp_workspace/tmp") {
        println!("[WD] Creatin repos folder!");
        if let Err(e) = run_command("mkdir rp_workspace/tmp") {
            println!("[WD] Error creating temporary directory!\n{:#?}", e);
            std::process::exit(1);
        };
    }

    // unzip source into sources folder one by one
    for source in sources.iter() {
        match source {
            FilePath::Zip(path) => {
                // clear tmp folder
                if let Err(e) = run_command("rm -rf rp_workspace/tmp/*") {
                    println!("[WD] Error clearing temporary directory!\n{:#?}", e);
                    std::process::exit(1);
                };

                // extract zip into tmp folder
                if let Err(e) = run_command(&format!("7z x -o./rp_workspace/tmp/ \"{}\"", path)) {
                    println!("[WD] Error extracting source into sources directory!\n{:#?}", e);
                    std::process::exit(1);
                };

                // get names of all extracted folders
                let folders = match folder_names("./rp_workspace/tmp/") {
                    Ok(f) => f,
                    Err(e) => {
                        println!("[WD] Error extracting sources in temporary directory!\n{:#?}", e);
                        std::process::exit(1);
                    }
                };
                // remove spaces from all extracted sources
                for folder_name in folders.iter() {
                    let no_space_path = folder_name.replace(' ', "_");
                    if let Err(e) = run_command(&format!("mv \"rp_workspace/tmp/{}\" rp_workspace/tmp/{}", folder_name, no_space_path)) {
                        println!("[WD] Error renaming source in sources directory to remove spaces!\n{:#?}", e);
                        std::process::exit(1);
                    };
                }

                // move extracted sourcs from tmp to sources folder
                if let Err(e) = run_command("mv rp_workspace/tmp/* rp_workspace/sources/") {
                    println!("[WD] Error moving extracted sources from temporary folder to sources folder!\n{:#?}", e);
                    std::process::exit(1);
                };
            },
            FilePath::GitHub(_) => (),
        }
    }
}

pub fn extract_submissions_from_sources() -> Vec<StudentProjectSubmission> {
    let folder_names: Vec<String> = match folder_names("./rp_workspace/sources") {
        Ok(s) => s,
        Err(e) => {
            println!("[WD] Error fetching folder names!\n{:#?}", e);
            std::process::exit(1);
        }
    };
    extract_repos_form_folders(folder_names)
}

fn extract_repos_form_folders(folder_names: Vec<String>) -> Vec<StudentProjectSubmission> {
    let mut submissions = vec![];
    let re = Regex::new(r#"(https?://(?:www\.)?(?:gitlab|github)\.com/[-a-zA-Z0-9@:%._\+~#=]{2,256}\b(?:[-a-zA-Z0-9@:%_\+.~#?&//=]*))"#).unwrap();
    for folder in folder_names.iter() {
        let mut subm = StudentProjectSubmission::new(folder.clone());

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
        let mut captured_link_fragments = capture[1].split("/-/tree");
        let captured_link = captured_link_fragments.nth(0);
        if let Some(link) = captured_link {
            subm.git_repo = Some(format!("{}.git", link.replace(".git", "")));
            submissions.push(subm);
        } else {
            println!("[GIT EXTRACTION] Error extracting and parsing link: {:#?}", captured_link);
        }
    }
    submissions
}
