#[derive(Debug)]
pub enum FilePath {
    Zip(String),
    Folder(String)
}