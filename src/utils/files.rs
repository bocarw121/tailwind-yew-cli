use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::PathBuf,
};

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

pub fn find_and_replace_file(target_file: &str, from: &str, to: &str) {
    // Package.json

    let is_real_target = is_real_file_or_dir(target_file);

    if !is_real_target {
        return;
    }

    let file = match fs::read_to_string(target_file) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("{target_file} not found in the root");
            // Maybe show a message with scripts to add
            return;
        }
    };

    let content = file.replace(from, to);

    match fs::write(target_file, content) {
        Ok(_) => {
            // println!("File has been replaced successfully")
        }
        Err(_) => {
            eprintln!("Something went wrong, please try again");
        }
    };
}

pub fn is_real_file_or_dir(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

/// Checks if a given file contains a specific item (string).
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path to the file.
/// * `item` - The string item to search for in the file.
///
/// # Returns
///
/// * `bool` - Returns `true` if the item is found in the file, otherwise `false`.
pub fn does_file_contain_item(file_path: &str, item: &str) -> bool {
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            println!("Could not open file: {}", file_path);
            return false;
        }
    };
    let reader = BufReader::new(file);

    let mut contains_package_name = false;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains(item) {
            contains_package_name = true;
            break;
        }
    }

    contains_package_name
}
