use std::fs;



pub fn check_workdir() -> bool {
    let wd_meta = match fs::metadata("./rp_workspace") {
        Ok(m) => m,
        Err(e) => {
            println!("Error parsing path (./rp_workspace): {}", e);
            return false;
        },
    };

    // check if path points to a directory
    wd_meta.is_dir()
}

pub fn check_dir_exists(dir_name: &str) -> bool {
    let wd_meta = match fs::metadata(dir_name.clone()) {
        Ok(m) => m,
        Err(e) => {
            println!("Error parsing path ({}): {}", dir_name, e);
            return false;
        },
    };

    // check if path points to a directory
    wd_meta.is_dir()
}