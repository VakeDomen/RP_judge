use crate::models::student_project::StudentProjectSubmission;

use super::{validator::check_dir_exists, git_commit_handler::get_commits_from_submission, os_helper::run_command};


pub fn compile_commits(submissions: &mut [StudentProjectSubmission]) {
    if let false = check_dir_exists("rp_workspace/repos") {
        println!("[GIT HANDLER] Error reading repos directory!");
        std::process::exit(1);
    }

    
    for submission in submissions.iter_mut() {
        // check if submission has cloned a
        if !submission.cloned {
            println!("[GIT HANDLER] Submission was not cloned: Skipping!");
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
            let mut successful_commits = 0;
            let mut was_checked = false;
            for commit_string in commits.iter().rev() {
                // change git repo to sprcified commit
                if let Err(e) = run_command(format!("git -C ./rp_workspace/repos/{} checkout {}", student_folder.replace(" ", "\\ "), commit_string).as_str()) {
                    println!("[GIT HANDLER] Error switching commits on repo({}): {:#?}", student_folder.replace(" ", "\\ "), e);
                    continue;
                }; 


                // compile with gcc
                let standards = ["c99", "c90", "c89", "c11", "c17"];

                let mut command_output = "".to_string();
                
                for standard in standards.iter() {

                    match run_command(format!("gcc -std={} ./rp_workspace/repos/{}/{}/main.c", standard, student_folder.replace(" ", "\\ "), task).as_str()) {
                        Ok(t) => {
                            command_output = t;
                            if command_output.is_empty() {
                                submission.gcc_standard = Some(standard.to_string());
                                break;
                            }
                        },
                        Err(e) => command_output = e.to_string(),
                    }; 
                }

                // if no warrnings/errors => no output => successful compile
                if command_output.is_empty() {
                    last_compile = true;
                    successful_commits += 1;                    
                }
                // if warrnings or errors, compilation was not successful
                if !command_output.is_empty() {
                    last_compile = false;
                    overall_compile = false;
                }
                was_checked = true;

                //checkout back to latest if on jordan's repo
                if submission.jordan {
                    if let Err(e) = run_command(format!("git -C ./rp_workspace/repos/{} checkout master", student_folder.replace(" ", "\\ ")).as_str()) {
                        println!("[GIT HANDLER] Error switching commits on repo back to latest master ({}): {:#?}", student_folder.replace(" ", "\\ "), e);
                    };
                }
            }
            if was_checked {
                save_compilation_results_to_submission(submission, task, last_compile, overall_compile, successful_commits);
            }
        }
    }
}

fn save_compilation_results_to_submission(
    submission: &mut StudentProjectSubmission, 
    task: &str, 
    last_compile: bool, 
    overall_compile: bool,
    successful_commits: i32,
) {
    if let Some(task1) = submission.has_task1.clone() {
        if task1 == task {
            submission.all_commits_compile_task1 = Some(overall_compile); 
            submission.final_commit_compile_task1 = Some(last_compile);
            submission.successful_compiles_task1 = Some(successful_commits);
            return;
        }
    }

    if let Some(task2) = submission.has_task2.clone() {
        if task2 == task {
            submission.all_commits_compile_task2 = Some(overall_compile); 
            submission.final_commit_compile_task2 = Some(last_compile);
            submission.successful_compiles_task2 = Some(successful_commits);
        }
    }
}
