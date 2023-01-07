use crate::models::{student_project::StudentProjectSubmission, file_path::FilePath};
use super::{validator::{check_dir_exists}, os_helper::{run_command, folder_names}};


pub fn clone_repos(submissions: &mut Vec<StudentProjectSubmission>, sources: &Vec<FilePath>) {
    if let false = check_dir_exists("rp_workspace/sources") {
        println!("[GIT HANDLER] Error reading sources directory!");
        std::process::exit(1);
    }
    if let false = check_dir_exists("rp_workspace/repos") {
        println!("[GIT HANDLER] Error reading repos directory!");
        std::process::exit(1);
    }

    for submission in submissions.iter_mut() {
        // if submission has a repo
        if let Some(repo) = &submission.git_repo {
            // try to clone it
            if let Err(e) = run_command(format!("git clone {} ./rp_workspace/repos/{}", repo, submission.student_folder.replace(" ", "\\ ")).as_str()) {
                println!("[GIT HANDLER] Error cloning git repo({}):\n{:#?}",repo, e);
                continue;
            }; 
            submission.cloned = true;
        }
    }

    // hadle jordan source
    let mut jordan_link = "".to_string();
    for source in sources.iter() {
        match source {
            FilePath::GitHub(link) => jordan_link = link.to_string(),
            _ => (),
        };
    }


    if !jordan_link.is_empty() {
        // clone master repo
        if let Err(e) = run_command(format!("git clone {} ./rp_workspace/repos/jrdndj/", jordan_link).as_str()) {
            println!("[GIT HANDLER] Error cloning git repo({}):\n{:#?}",jordan_link, e);
        }; 

        // extract student folders
        let folders = match folder_names(&format!("./rp_workspace/repos/jrdndj/AY 2022-2023/Student Works")) {
            Ok(f) => f,
            Err(e) => {
                println!("[WD] Error checking repository folder structure!\n{:#?}", e);
                std::process::exit(1);
            }
        };
        for folder in folders.into_iter() {
            let mut submission = StudentProjectSubmission::new(
                format!("jrdndj/AY 2022-2023/Student Works/{}", folder), 
                true
            );
            submission.cloned = true;
            submissions.push(submission);
        }
    }
}
