use std::{fs, os::unix::prelude::PermissionsExt, path::Path};

use colored::Colorize;

use crate::{
    assets::Asset,
    build::get_build_script,
    utils::{
        commands::my_commands,
        files::{create_dir_and_file, create_file},
    },
};

const INFO_MD: &str = "tailwind_info.md";
const CONFIG_JS: &str = "./tailwind.config.js";
const INDEX_HTML: &str = "index.html";
const TAILWIND_CSS: &str = "tailwindcss";

pub fn yew_setup(path: &str, input_name: &str, output_name: &str) {
    create_file(CONFIG_JS);

    // Update tailwind config with sample
    Asset::locate_target_and_write_assets(CONFIG_JS, "config.js");

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

fn replace_placeholders(symbol: &str, replacement_content: &str, file_path: &str) {
    let content = Asset::handle_sample(file_path);
    let replaced = content.replace(symbol, replacement_content);
    fs::write(file_path, replaced).unwrap();
}

fn create_read_me(input_path: &str, output_path: &str) {
    let script = format!("./{} -i {} -o {}", TAILWIND_CSS, &input_path, &output_path);

    create_file(INFO_MD);

    replace_placeholders("@path", &script, INFO_MD);
    replace_placeholders("@output", &output_path, INFO_MD);
}

fn curl_down_tailwind_build(selected_os: &str) {
    let command_url = format!(
        "https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-{}",
        selected_os
    );

    let args = ["-sLO", &command_url];

    my_commands("curl", &args);
}

fn set_permissions_and_rename_executable(build_executable: &str) {
    fs::set_permissions(
        format!("./{build_executable}"),
        fs::Permissions::from_mode(0o755),
    )
    .unwrap();

    my_commands("mv", &[build_executable, TAILWIND_CSS]);
}

fn setup_index_html(output_path: &str) {
    let is_html_present = Path::new(INDEX_HTML).is_file();

    if !is_html_present {
        create_file(INDEX_HTML);
    }
    Asset::locate_target_and_write_assets(INDEX_HTML, INDEX_HTML);

    replace_placeholders("@output", output_path, INDEX_HTML);
}
