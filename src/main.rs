#[allow(unused_imports)]
use std::env;
use std::format as f;
use std::{
    fmt::format,
    fs,
    io::{self, Write},
};

const ENV_PATH: &str = "PATH";
const CMD_EXIT: &str = "exit";
const CMD_ECHO: &str = "echo";
const CMD_TYPE: &str = "type";

const COMMANDS: [&str; 3] = [CMD_ECHO, CMD_EXIT, CMD_TYPE];

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

                match command_parts[0] {
                    CMD_EXIT => {
                        let mut exit_code = 0;
                        if command_parts.len() > 1 {
                            exit_code = command_parts[1].parse::<i32>().unwrap_or(0);
                        }

                        std::process::exit(exit_code);
                    }
                    CMD_ECHO => {
                        let echo_string = command.replacen(CMD_ECHO, "", 1);
                        println!("{}", echo_string.trim());
                    }
                    CMD_TYPE => {
                        if command_parts.len() > 1 {
                            let paths: Vec<&str> = path.split(":").collect();
                            let mut type_output: String = String::new();

                            // Before: checks for builtin ones
                            if COMMANDS.contains(&command_parts[1]) {
                                println!("{} is a shell builtin", command_parts[1]);
                            } else {
                                for path_item in paths {
                                    // Check if the specified command exists in path
                                    let command_path = f!("{path_item}/{}", command_parts[1]);
                                    if fs::metadata(&command_path).is_ok() {
                                        type_output =
                                            f!("{} is {}", command_parts[1], command_path);
                                        break;
                                    } else {
                                        type_output = f!("{}: not found", command_parts[1]);
                                    }
                                }
                                println!("{type_output}");
                            }
                        }
                    }
                    _ => {
                        println!("{}: command not found", command);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {e}");
        }
    }
}
