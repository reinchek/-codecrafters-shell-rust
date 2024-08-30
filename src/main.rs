use std::char::from_u32_unchecked;
#[allow(unused_imports)]
use std::env;
use std::format as f;
use std::{
    fs,
    io::{self, Write},
    process::Command,
};

const ENV_PATH: &str = "PATH";
const BUILTIN_CMD: [&str;3] = [
    "echo",
    "exit",
    "type"
];

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

                let command_parts: Vec<&str> = command.trim().split_whitespace().collect();

                if command_parts[0] == "type" {
                    if command_parts.len() > 1 {
                        let command_name = command_parts[1];
                        if BUILTIN_CMD.contains(&command_name) {
                            println!("{} is a shell builtin", command_name);
                        } else {
                            match locate_program(command_name.to_string()) {
                                Some(path) => println!("{} is {}", command_name, path),
                                None => println!("{}: not found", command_name),
                            }
                        }
                    } else {
                        println!("type: expected an argument");
                    }
                } else if command_parts[0] == "exit" {
                    std::process::exit(0);
                } else {
                    match locate_program(command_parts[0].to_string()) {
                        Some(command_path) => {
                            let args = &command_parts[1..];
                            let mut cmd = Command::new(command_path);
                            if !args.is_empty() {
                                cmd.args(args);
                            }

                            let output = cmd.output().expect("Failed to execute command");
                            io::stdout().write_all(&output.stdout).unwrap();
                            io::stderr().write_all(&output.stderr).unwrap();
                        }
                        None => {
                            println!("{}: command not found", command_parts[0]);
                        }

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
