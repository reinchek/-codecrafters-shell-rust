#[allow(unused_imports)]
use std::env;
use std::format as f;
use std::{
    fmt::format,
    fs,
    io::{self, Write},
    process::Command,
};

const ENV_PATH: &str = "PATH";

fn main() {
    // Standard input handler
    let stdin = io::stdin();
    let mut command = String::new();

    match env::var(ENV_PATH) {
        Ok(path) => {
            loop {
                // Uncomment this block to pass the first stage
                print!("$ ");
                io::stdout().flush().unwrap();

                command = String::new();
                stdin.read_line(&mut command).unwrap();
                command.pop();

                if command.is_empty() || command.len() == 0 {
                    break;
                }

                let command_parts: Vec<&str> = command.split(" ").collect();

                match locate_program(command_parts[0].to_string()) {
                    Some(command_path) => {
                        let output = Command::new(command_path)
                            .args([command.replacen(command_parts[0], "", 1).trim()])
                            .output()
                            .expect("Failed to execute process");
                        io::stdout().write_all(&output.stdout).unwrap();
                    }
                    None => {
                        println!("Command not found");
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {e}");
        }
    }
}

// Locate program. If exists return Some(program's path)
fn locate_program(command_name: String) -> Option<String> {
    // Check for PATH environment variable
    match env::var(ENV_PATH) {
        Ok(path) => {
            let paths: Vec<&str> = path.split(":").collect();
            let mut command_found: Option<String> = None;

            for path_item in paths {
                // Check if the specified command exists in path
                let command_path = f!("{path_item}/{command_name}");

                if fs::metadata(&command_path).is_ok() {
                    return Some(command_path);
                }
            }
            None
        }
        Err(_) => None,
    }
}
