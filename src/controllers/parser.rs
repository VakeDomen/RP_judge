use crate::models::file_path::FilePath;
use std::{fs, path::Path};

pub fn parse_file_args(args: Vec<String>) -> Vec<FilePath> {
    let mut files = vec![];
    for arg in args.iter().skip(1) {
        // check if arg is jordan's repo (jordan)
        if arg == "jordan" {
            files.push(FilePath::GitHub("https://github.com/jrdndj/CP1preps.git".to_string()));
            continue;
        }

        // create metadata to determine file type
        let path_meta = match fs::metadata(arg) {
            Ok(m) => m,
            Err(e) => {
                println!("Error parsing path ({}): {}", arg, e);
                continue;
            },
        };


        // check if path points to a zip file
        if path_meta.is_file() {
            let path = Path::new(arg);
            if path.extension().unwrap_or_default() == "zip" {
                files.push(FilePath::Zip(arg.to_string()));
                continue;
            }
        }

        println!("Error parsing path ({}): Unsupported file type!", arg);
    }
    files
}

pub fn escape(path: &String) -> String {
    path
        .replace(" ", "\\ ")
        .replace("&", "\\&")
        .replace("|", "\\|")
}