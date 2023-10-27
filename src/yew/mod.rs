#[cfg(unix)]
use std::os::unix::prelude::PermissionsExt;
use std::{fs, path::Path};

use colored::Colorize;

use crate::{
    assets::Asset,
    build::get_build_script,
    utils::{
        commands::my_commands,
        files::{create_dir_and_file, create_file, does_file_contain_item, find_and_replace_file},
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

fn create_read_me(input_path: &str, output_path: &str) {
    let script = format!("./{} -i {} -o {}", TAILWIND_CSS, &input_path, &output_path);

    create_file(INFO_MD);

    Asset::locate_target_and_write_assets(INFO_MD, INFO_MD);

    find_and_replace_file(INFO_MD, "@path", &script);
    find_and_replace_file(INFO_MD, "@output", output_path);
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
    if cfg!(unix) {
        #[cfg(unix)]
        {
            match fs::set_permissions(
                format!("./{build_executable}"),
                fs::Permissions::from_mode(0o755),
            ) {
                Ok(_) => (),
                Err(e) => println!("Something went wrong setting permissions: {}", e),
            }

            my_commands("mv", &[build_executable, TAILWIND_CSS]);
        }
    } else {
        fs::rename(build_executable, TAILWIND_CSS).unwrap();
    }
}

fn setup_index_html(output_path: &str) {
    let is_html_present = Path::new(INDEX_HTML).is_file();

    if !is_html_present {
        create_file(INDEX_HTML);
        Asset::locate_target_and_write_assets(INDEX_HTML, INDEX_HTML);

        find_and_replace_file(INDEX_HTML, "@output", output_path);
        return;
    }

    let link = format!("<link data-trunk rel=\"css\" href=\"{}\">", output_path);

    let is_link_present = does_file_contain_item(INDEX_HTML, &link);

    if is_link_present {
        let message = format!("{} already exists in index.html", link).yellow();
        println!("{}", message);
        return;
    }

    let is_title_present = does_file_contain_item(INDEX_HTML, "<title>");

    if is_title_present {
        find_and_replace_file(INDEX_HTML, "</title>", &format!("</title>\n\t\t{}", link));

        return;
    }

    let is_meta_charset = does_file_contain_item(INDEX_HTML, "<meta charset=\"utf-8\" />");

    if is_meta_charset {
        find_and_replace_file(
            INDEX_HTML,
            "<meta charset=\"utf-8\" />",
            &format!("<meta charset=\"utf-8\" />\n\t\t{}", link),
        );

        return;
    }

    let is_head = does_file_contain_item(INDEX_HTML, "<head>");

    if is_head {
        find_and_replace_file(INDEX_HTML, "<head>", &format!("<head>\n\t\t{}", link))
    }
}
