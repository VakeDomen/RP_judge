use chrono::{NaiveDateTime};

#[derive(Debug)] 
pub struct StudentProjectSubmission {
    pub student_folder: String,
    pub git_repo: Option<String>,
    pub cloned: bool,
    pub jordan: bool,

    pub gcc_standard: Option<String>,
    pub last_commit_date: Option<NaiveDateTime>,

    pub total_commits: Option<i32>,
    pub commits_task1: Option<Vec<String>>,
    pub commits_task2: Option<Vec<String>>,
    pub has_task1: Option<String>,
    pub has_task2: Option<String>,
    pub task1_main: Option<String>,
    pub task2_main: Option<String>,
    pub all_commits_compile_task1: Option<bool>,
    pub all_commits_compile_task2: Option<bool>,
    pub final_commit_compile_task1: Option<bool>,
    pub final_commit_compile_task2: Option<bool>,
    pub successful_compiles_task1: Option<i32>,
    pub successful_compiles_task2: Option<i32>,
}

impl StudentProjectSubmission {
    pub fn new(name: String, jordan: bool) -> Self {
        Self { 
            student_folder: name, 
            git_repo: None, 
            cloned: false, 
            jordan: jordan,
            gcc_standard: None,
            last_commit_date: None,
            total_commits: None,
            commits_task1: None, 
            commits_task2: None, 
            has_task1: None, 
            has_task2: None, 
            task1_main: None,
            task2_main: None,
            all_commits_compile_task1: None, 
            all_commits_compile_task2: None, 
            final_commit_compile_task1: None,
            final_commit_compile_task2: None,
            successful_compiles_task1: None,
            successful_compiles_task2: None,
        }
    }
}