#[derive(Debug)]
pub enum FilePath {
    Zip(String),
    GitHub(String),
}