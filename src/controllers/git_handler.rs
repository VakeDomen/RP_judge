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

pub fn compile_commits(submissions: &mut Vec<StudentProjectSubmission>) {
    if let false = check_dir_exists("rp_workspace/repos") {
        println!("[GIT HANDLER] Error reading repos directory!");
        std::process::exit(1);
    }

    let tasks_to_check = ["Task1", "Task2"];
    for submission in submissions.iter_mut() {
        // check if submission has cloned a
        if !submission.cloned {
            println!("[GIT HANDLER] Submission was not cloned: Skipping!");
            continue;
        }

        for task in tasks_to_check.iter() {
            // clone string so we can borrow submission mutably
            let student_folder = submission.student_folder.clone();

            // fetch commits of submission
            // if there are none, continue to next task/submisssion
            let commits  =  match get_commits_from_submission(task, submission) {
                Some(c) => c,
                None => {
                    println!("[GIT HANDLER] Submission task ({}) does not have commits: Skipping!", task);
                    continue;
                },
            };

            // if no commits, skip submission
            if commits.is_empty() {
                continue;
            }
            
            // go trough commits one by one and check if they compile
            // go from oldest to newest
            let mut last_compile = false;
            let mut overall_compile = true;
            let mut successfull_commits = 0;
            for commit_string in commits.iter().rev() {
                println!("{}", format!("git checkout {}", commit_string));
                // change git repo to sprcified commit
                if let Err(e) = run_command(format!("git -C ./rp_workspace/repos/{} checkout {}", student_folder, commit_string).as_str()) {
                    println!("[GIT HANDLER] Error switching commits on repo({}): {:#?}", student_folder, e);
                    continue;
                }; 

                // compile with gcc
                let command_output = match run_command(format!("gcc ./rp_workspace/repos/{}/{}/main.c", student_folder, task).as_str()) {
                    Ok(t) => t,
                    Err(_) => {
                        overall_compile = false;
                        continue;
                    },
                }; 

                // if no warrnings/errors => no output => successfull compile
                if command_output.is_empty() {
                    last_compile = true;
                    successfull_commits += 1;                    
                }
                // if warrnings or errors, compilation was not successful
                if !command_output.is_empty() {
                    last_compile = false;
                    overall_compile = false;
                }
            }
            
            save_compilation_results_to_submission(submission, task, last_compile, overall_compile, successfull_commits);
        }
    }
}

fn save_compilation_results_to_submission(
    submission: &mut StudentProjectSubmission, 
    task: &str, 
    last_compile: bool, 
    overall_compile: bool,
    successfull_commits: i32,
) {
    match task {
        "Task1" => {
            submission.all_commits_compile_task1 = Some(overall_compile); 
            submission.final_commit_compile_task1 = Some(last_compile);
            submission.successfull_compiles_task1 = Some(successfull_commits);
        },
        "Task2" => {
            submission.all_commits_compile_task2 = Some(overall_compile); 
            submission.final_commit_compile_task2 = Some(last_compile);
            submission.successfull_compiles_task2 = Some(successfull_commits);
        },
        _ => (),
    }
}

fn get_commits_from_submission(task: &str, submission: &mut StudentProjectSubmission) -> Option<Vec<String>> {
    match task {
        "Task1" => submission.commits_task1.clone(),
        "Task2" => submission.commits_task2.clone(),
        _ => None,
    }
}

pub fn extract_commits(submissions: &mut Vec<StudentProjectSubmission>) {
    if let false = check_dir_exists("rp_workspace/repos") {
        println!("[GIT HANDLER] Error reading repos directory!");
        std::process::exit(1);
    }
    let tasks_to_check = ["Task1", "Task2"];
    for submission in submissions.iter_mut() {
        for task in tasks_to_check.iter() {
            // check commits for task 1
            let command_output = match run_command(format!("git -C ./rp_workspace/repos/{}  --no-pager log --pretty=\"%h\" -- {}", submission.student_folder, task).as_str()) {
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