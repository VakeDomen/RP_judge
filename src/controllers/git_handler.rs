use crate::models::student_project::StudentProjectSubmission;

use super::{validator::check_dir_exists, os_helper::run_command};


pub fn clone_repos(submissions: &mut Vec<StudentProjectSubmission>) {
    if let false = check_dir_exists("rp_workspace/sources") {
        println!("[GIT HANDLER] Error reading sources directory!");
        std::process::exit(1);
    }
    if let false = check_dir_exists("rp_workspace/repos") {
        println!("[GIT HANDLER] Error reading repos directory!");
        std::process::exit(1);
    }

    for submission in submissions.iter_mut() {
        let repo = match &submission.git_repo {
            Some(repo) => repo,
            None => {
                println!("[GIT HANDLER] No git repo detected: {:#?}",submission);
                continue;
            },
        };
        if let Err(e) = run_command(format!("git clone {} ./rp_workspace/repos/{}", repo, submission.student_folder).as_str()) {
            println!("[GIT HANDLER] Error cloning git repo({}):\n{:#?}",repo, e);
            std::process::exit(1);
        }; 
        submission.cloned = true;
    }
}
