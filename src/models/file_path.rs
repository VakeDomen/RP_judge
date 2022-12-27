#[derive(Debug)]
pub enum FilePath {
    Zip(String),
    Folder(String)
}

impl FilePath {
    pub fn get_path(&self) -> String {
        match self {
            FilePath::Zip(s) => s.clone(),
            FilePath::Folder(s) => s.clone(),
        }
    }
}