use crate::models::student_project::StudentProjectSubmission;

use super::{validator::check_dir_exists, os_helper::{run_command, folder_names}};


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
        // if submission has a repo
        if let Some(repo) = &submission.git_repo {
            // try to clone it
            if let Err(e) = run_command(format!("git clone {} ./rp_workspace/repos/{}", repo, submission.student_folder).as_str()) {
                println!("[GIT HANDLER] Error cloning git repo({}):\n{:#?}",repo, e);
                continue;
            }; 
            submission.cloned = true;
        }
    }
}

// git -C ./rp_workspace/repos/Luka_Ur┼бi─Н_205159_assignsubmission_onlinetext_ --no-pager log --pretty="%h %s" -- Task2

pub fn extract_commits(submissions: &mut Vec<StudentProjectSubmission>) {
    if let false = check_dir_exists("rp_workspace/repos") {
        println!("[GIT HANDLER] Error reading repos directory!");
        std::process::exit(1);
    }
    let tasks_to_check = ["Task1", "Task2"];
    for submission in submissions.iter_mut() {
        for task in tasks_to_check.iter() {
            // check commits for task 1
            let command_output = match run_command(format!("git -C ./rp_workspace/repos/{}  --no-pager log --pretty=\"%h|||%s\" -- {}", submission.student_folder, task).as_str()) {
                Ok(t) => t,
                Err(e) => {
                    println!("[GIT HANDLER] Error checking commits for git repo({}):\n{:#?}",submission.student_folder, e);
                    std::process::exit(1);
                },
            }; 
            let commits = Some(command_output
                .split("\n")
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect()
            );
            save_commits_to_submission(submission, task, commits);
        }
    }

}

fn save_commits_to_submission(submission: &mut StudentProjectSubmission, task: &str, commits: Option<Vec<String>>) {
    match task {
        "Task1" => submission.commits_task1 = commits,
        "Task2" => submission.commits_task2 = commits,
        _ => (),
    }
}

pub fn check_structure(submissions: &mut Vec<StudentProjectSubmission>) {
    if let false = check_dir_exists("rp_workspace/repos") {
        println!("[GIT HANDLER] Error reading repos directory!");
        std::process::exit(1);
    }

    for submission in submissions.iter_mut() {
        if !submission.cloned {
            continue;
        }
        let folders = match folder_names(&format!("./rp_workspace/repos/{}", submission.student_folder)) {
            Ok(f) => f,
            Err(e) => {
                println!("[WD] Error checking repository folder structure!\n{:#?}", e);
                std::process::exit(1);
            }
        };
        submission.has_task1 = Some(folders.contains(&"Task1".to_string()));
        submission.has_task2 = Some(folders.contains(&"Task2".to_string()));
    }
}