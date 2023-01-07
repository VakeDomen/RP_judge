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
            let mut submission = StudentProjectSubmission::new(format!("jrdndj/AY 2022-2023/Student Works/{}", folder));
            submission.cloned = true;
            submissions.push(submission);
        }
    }
}

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
            }
            
            save_compilation_results_to_submission(submission, task, last_compile, overall_compile, successful_commits);
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

fn get_commits_from_submission(task: &str, submission: &mut StudentProjectSubmission) -> Option<Vec<String>> {
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
        submission.has_task2 = folders
            .iter()
            .find(|folder| accepted_folder_names_task2.contains(&folder.as_str()))
            .map(|name| name.to_string());

        if let Some(task) = &submission.has_task1{
            println!("./rp_workspace/repos/{}/{}/main.c", submission.student_folder, task);
        }
    }
}