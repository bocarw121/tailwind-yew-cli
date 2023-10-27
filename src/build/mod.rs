use inquire::Select;

/// Asks user for their os version and returns a tuple
/// with the selected os version and the executable name
pub fn get_build_script() -> (String, String) {
    let os_versions = vec![
        "linux-arm64",
        "linux-armv7",
        "linux-x64",
        "macos-arm64",
        "macos-x64",
        "windows-x64.exe",
    ];

    let selected_os = Select::new("What is your os version", os_versions)
        .prompt()
        .unwrap_or_else(|_| std::process::exit(1));

    let build_executable = format!("tailwindcss-{}", selected_os);

    (selected_os.to_string(), build_executable)
}
