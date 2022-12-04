use std::{fs, path::PathBuf};

pub fn write_to_file(path: PathBuf, content: String) {
    fs::write(path, content).unwrap_or_else(|_| {
        eprintln!("Something went wrong, please try again later");
        std::process::exit(1)
    });
}

pub fn create_file(file_path: &str) {
    match fs::File::create(file_path) {
        Ok(_) => {}
        Err(_) => {
            eprintln!("Error creating {file_path}");
        }
    };
}

pub fn create_dir_and_file(dir_path: &str, file_path: &str) {
    match fs::create_dir(dir_path) {
        Ok(_) => {}
        Err(_) => {
            eprintln!("./{dir_path} already exists");
        }
    };

    let new_file = format!("{dir_path}/{file_path}");

    match fs::File::create(new_file) {
        Ok(_) => {}
        Err(_) => {
            eprintln!("Error creating file");
        }
    };
}
