#[derive(Debug)] 
pub struct StudentProjectSubmission {
    pub student_folder: String,
    pub git_repo: Option<String>,
    pub has_two_commits: Option<bool>,
    pub all_commits_compile: Option<bool>,
    pub cloned: bool,
}