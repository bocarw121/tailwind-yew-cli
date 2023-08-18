use std::process::{Command, ExitStatus};

pub fn my_commands(program: &str, inputs: &[&str]) {
    let mut command = Command::new(program);

    for input in inputs.iter() {
        command.arg(input);
    }

    let mut child_process = command.spawn().expect(&format!(
        "Failed to start the process. The executable '{}' may not exist or is not accessible.",
        program
    ));

    let status: ExitStatus = child_process
        .wait()
        .expect("Failed to wait for the process");

    if !status.success() {
        println!("Command failed with exit code: {:?}", status.code());
    }
}
