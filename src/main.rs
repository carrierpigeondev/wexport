use std::env;
use std::process::{self, Command};

fn is_already_in_path(input_path: &String) -> bool {
    let output = Command::new("powershell")
        .arg("-Command")
        .arg("[System.Environment]::GetEnvironmentVariable('Path', [System.EnvironmentVariableTarget]::Machine)")
        .output()
        .expect("Failed to retrieve PATH");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("PowerShell Error:\n{}", stderr);
        process::exit(1);
    }

    let current_path = String::from_utf8_lossy(&output.stdout);

    if current_path.split(';').any(|p| p.trim() == input_path) {
        println!("The directory is already in the PATH.");
        return true;
    }

    return false;
}

fn add_input_to_path(input_path: &String) {
    let command = format!(
        "[System.Environment]::SetEnvironmentVariable('Path', $env:Path + ';{}', [System.EnvironmentVariableTarget]::Machine)",
        input_path
    );

    let output = Command::new("powershell")
        .arg("-Command")
        .arg(command)
        .output()
        .expect("Failed to execute PowerShell command");

    if output.status.success() {
        println!("Successfully updated PATH.");
    }
    else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("PowerShell Error:\n{}", stderr);
        process::exit(1);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        process::exit(1);
    }
    let input_path = &args[1];

    if is_already_in_path(&input_path) {
        println!("No changes made to PATH.");
        process::exit(0);
    }
    else {
        add_input_to_path(&input_path);
    }
}
