use std::char::from_u32_unchecked;
#[allow(unused_imports)]
use std::env;
use std::format as f;
use std::os::unix::process::CommandExt;
use std::{
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

                let command_parts: Vec<&str> = command.trim().split(" ").collect();
                let mut command_found: Option<String> = None;

                if (command_parts[0] != "type") {
                    command_found = locate_program(command_parts[0].to_string());
                } else {
                    command_found = Some("/usr/bin/type".to_string());
                }

                match command_found {
                    Some(command_path) => {
                        let args = command.replacen(command_parts[0], "", 1).trim().to_owned();
                        let mut cmd = Command::new(command_path);

                        if args.len() > 0 {
                            cmd.args([args]);
                        }

                        io::stdout()
                            .write_all(&cmd.output().unwrap().stdout)
                            .unwrap();
                        io::stderr().write(&cmd.output().unwrap().stderr).unwrap();
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

            for path_item in paths {
                // Check if the specified command exists in path
                let command_path = f!("{path_item}/{}", command_name.trim());

                if fs::metadata(&command_path).is_ok() {
                    return Some(command_path);
                }
            }
            None
        }
        Err(_) => None,
    }
}
