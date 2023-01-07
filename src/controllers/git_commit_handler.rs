use crate::models::student_project::StudentProjectSubmission;

use super::{validator::{check_dir_exists, find_main_file}, os_helper::{run_command, folder_names}};


pub fn get_commits_from_submission(task: &str, submission: &mut StudentProjectSubmission) -> Option<Vec<String>> {
    if let Some(task1) = submission.has_task1.clone() {
        if task1 == task {
            return submission.commits_task1.clone();
        }
    }

    if let Some(task2) = submission.has_task2.clone() {
        if task2 == task {
           return submission.commits_task2.clone();
        }
    }
    None
}

pub fn extract_commits(submissions: &mut [StudentProjectSubmission]) {
    if let false = check_dir_exists("rp_workspace/repos") {
        println!("[GIT HANDLER] Error reading repos directory!");
        std::process::exit(1);
    }
    for submission in submissions.iter_mut() {
        if !submission.cloned {
            continue;
        }


        let mut tasks_to_check = vec![];
        // find task 1 name
        if let Some(t1) = submission.has_task1.clone() {
            tasks_to_check.push(t1);
        }

        // find task 2 name
        if let Some(t2) = submission.has_task2.clone() {
            tasks_to_check.push(t2);
        }

        let command_output = match run_command(format!("git -C ./rp_workspace/repos/{}  --no-pager log --pretty=\"%h\"", submission.student_folder.replace(" ", "\\ ")).as_str()) {
            Ok(t) => t,
            Err(e) => {
                println!("[GIT HANDLER] Error checking commits for git repo({}):\n{:#?}",submission.student_folder.replace(" ", "\\ "), e);
                std::process::exit(1);
            },
        }; 
        submission.total_commits = Some(command_output
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect::<Vec<String>>().len() as i32
        );

        for task in tasks_to_check.iter() {
            // check commits for task 1
            let command_output = match run_command(format!("git -C ./rp_workspace/repos/{}  --no-pager log --pretty=\"%h\" -- {}", submission.student_folder.replace(" ", "\\ "), task).as_str()) {
                Ok(t) => t,
                Err(e) => {
                    println!("[GIT HANDLER] Error checking commits for git repo({}):\n{:#?}",submission.student_folder, e);
                    std::process::exit(1);
                },
            }; 
            let commits = Some(command_output
                .split('\n')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect()
            );
            save_commits_to_submission(submission, task, commits);
        }
    }
}

fn save_commits_to_submission(submission: &mut StudentProjectSubmission, task: &str, commits: Option<Vec<String>>) {
    if let Some(task1) = submission.has_task1.clone() {
        if task1 == task {
            submission.commits_task1 = commits;
            return;
        }
    }

    if let Some(task2) = submission.has_task2.clone() {
        if task2 == task {
            submission.commits_task2 = commits;
        }
    }
}

pub fn check_structure(submissions: &mut [StudentProjectSubmission]) {
    if let false = check_dir_exists("rp_workspace/repos") {
        println!("[GIT HANDLER] Error reading repos directory!");
        std::process::exit(1);
    }

    for submission in submissions.iter_mut() {
        if !submission.cloned {
            continue;
        }
        // validate_and_fix_task_names(&format!("./rp_workspace/repos/{}", submission.student_folder));
        let folders = match folder_names(&format!("./rp_workspace/repos/{}", submission.student_folder)) {
            Ok(f) => f,
            Err(e) => {
                println!("[WD] Error checking repository folder structure!\n{:#?}", e);
                std::process::exit(1);
            }
        };
        let accepted_folder_names_task1 = ["Task1", "task1", "1task", "1Task", "task_1", "Task_1", "Task 1", "task 1"];
        let accepted_folder_names_task2 = ["Task2", "task2", "2task", "2Task", "task_2", "Task_2", "Task 2", "task 2"];
    

        submission.has_task1 = folders
            .iter()
            .find(|folder| accepted_folder_names_task1.contains(&folder.as_str()))
            .map(|name| name.to_string());

        if let Some(task) = &submission.has_task1 {
            submission.task1_main = find_main_file(&format!("./rp_workspace/repos/{}/{}", submission.student_folder, task));
        }
        submission.has_task2 = folders
            .iter()
            .find(|folder| accepted_folder_names_task2.contains(&folder.as_str()))
            .map(|name| name.to_string());
        if let Some(task) = &submission.has_task2 {
            submission.task2_main = find_main_file(&format!("./rp_workspace/repos/{}/{}", submission.student_folder, task));
        }

        
        if let Some(task) = &submission.has_task1 {
            if let Some(main) = &submission.task1_main  {
                println!("./rp_workspace/repos/{}/{}/{}", submission.student_folder, task, main);
            }
        }
    }
}