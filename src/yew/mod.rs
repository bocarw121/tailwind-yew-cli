use std::{fs, os::unix::prelude::PermissionsExt, path::Path, process::Command};

use colored::Colorize;

use crate::{
    assets::Asset,
    build::get_build_script,
    file_utils::{create_dir_and_file, create_file},
};

pub fn yew_setup(path: &str, input_name: &str, output_name: &str) {
    create_file("./tailwind.config.js");

    // Update tailwind config with sample
    Asset::locate_target_and_write_assets("./tailwind.config.js", "yew.js");

    let (selected_os, build_executable) = get_build_script();

    curl_down_tailwind_build(&selected_os);

    set_permissions_and_rename_executable(&build_executable);

    // tailwind css input and out paths
    let input_path = format!("{}/{}", path, input_name);

    let output_path = format!("{}/{}", path, output_name);

    // Create the directory with the input css file
    create_dir_and_file(path, input_name);

    // Add tailwindcss classes to input css file
    Asset::locate_target_and_write_assets(&input_path, "tailwind.css");

    setup_index_html(&output_path);

    create_read_me(&input_path, &output_path);

    let success_message = "Congrats! Tailwindcss has been setup successfully 🚀.".green();

    println!("{}", success_message)
}

fn replace_placeholders(symbol: &str, path: &str, file_path: &str) {
    let content = Asset::handle_sample(file_path);
    let replaced = content.replace(symbol, path);
    fs::write(file_path, replaced).unwrap();
}

fn create_read_me(input_path: &str, output_path: &str) {
    let script = format!("./tailwindcss -i {} -o {}", &input_path, &output_path);

    create_file("tailwind_info.md");

    replace_placeholders("@path", &script, "tailwind_info.md");
}

fn curl_down_tailwind_build(selected_os: &str) {
    let command_url = format!(
        "https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-{}",
        selected_os
    );

    let args = ["-sLO", &command_url];

    Command::new("curl")
        .args(args)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

fn set_permissions_and_rename_executable(build_executable: &str) {
    fs::set_permissions(
        format!("./{build_executable}"),
        fs::Permissions::from_mode(0o755),
    )
    .unwrap();

    Command::new("mv")
        .arg(build_executable)
        .arg("tailwindcss")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

fn setup_index_html(output_path: &str) {
    let is_html_present = Path::new("index.html").is_file();

    if !is_html_present {
        create_file("index.html");
    }
    Asset::locate_target_and_write_assets("./index.html", "index.html");

    replace_placeholders("@output", output_path, "index.html");
}
