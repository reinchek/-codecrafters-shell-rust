#[allow(unused_imports)]
use std::env;
use std::fmt;
use std::format as f;
use std::iter::FromIterator;
use std::{
    fs,
    io::{self, Write},
    process::Command,
};

enum MyCommand {
    Exit,
    Type,
    Echo,
    Pwd,
    Cd,
}

struct MyCommandContainer {
    commands: Vec<MyCommand>,
}

impl MyCommandContainer {
    fn new() -> Self {
        MyCommandContainer {
            commands: Vec::new(),
        }
    }

    fn as_refs(&self) -> Vec<&MyCommand> {
        self.commands.iter().collect()
    }
}

impl MyCommand {
    fn from_str(command: &str) -> Option<MyCommand> {
        match command {
            "exit" => Some(MyCommand::Exit),
            "type" => Some(MyCommand::Type),
            "echo" => Some(MyCommand::Echo),
            "pwd" => Some(MyCommand::Pwd),
            "cd" => Some(MyCommand::Cd),
            _ => None,
        }
    }
}

impl fmt::Display for MyCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyCommand::Cd => write!(f, "cd"),
            MyCommand::Pwd => write!(f, "pwd"),
            MyCommand::Exit => write!(f, "exit"),
            MyCommand::Type => write!(f, "type"),
            MyCommand::Echo => write!(f, "echo"),
        }
    }
}

impl<'a> FromIterator<&'a str> for MyCommandContainer {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut container = MyCommandContainer::new();
        for s in iter {
            if let Some(cmd) = MyCommand::from_str(s) {
                container.commands.push(cmd)
            }
        }
        container
    }
}

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

                let command_parts: Vec<&str> = command.trim().split_whitespace().collect();

                match MyCommand::from_str(command_parts[0]) {
                    Some(MyCommand::Type) => {
                        // Check if command_parts is greater then 1
                        if command_parts.len() > 1 {
                            // Obtain command name, second parameter passed to type command
                            let command_name = command_parts[1];
                            // Check if command_name is one of BUILTIN_CMD commands
                            if let Some(builtin_command) = MyCommand::from_str(command_name) {
                                println!("{} is a shell builtin", builtin_command.to_string());
                            } else {
                                // Otherwise locate program calling locate_program function
                                match locate_program(command_name.to_string()) {
                                    Some(path) => println!("{} is {}", command_name, path),
                                    None => println!("{}: not found", command_name),
                                }
                            }
                        } else {
                            println!("type: expected an argument");
                        }
                    }
                    Some(MyCommand::Exit) => {
                        // Call exit with status code = 0
                        std::process::exit(0);
                    }
                    Some(MyCommand::Pwd) => {
                        let current_dir = env::current_dir().unwrap();
                        println!("{}", current_dir.display());
                    }
                    Some(MyCommand::Cd) => {
                        if command_parts.len() > 0 {
                            match std::env::set_current_dir(command_parts[1]) {
                                Err(_) => {
                                    println!("cd: {}: No such file or directory", command_parts[1])
                                }
                                Ok(_) => (),
                            }
                        }
                    }
                    _ => {
                        // If all other cases, find and execute the command
                        match locate_program(command_parts[0].to_string()) {
                            Some(command_path) => {
                                // Get a slice starting from index 1 to the array's end
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
