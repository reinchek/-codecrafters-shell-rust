#[allow(unused_imports)]
use std::io::{self, Write};

const CMD_EXIT: &str = "exit";
const CMD_ECHO: &str = "echo";

fn main() {
    // Standard input handler
    let stdin = io::stdin();
    let mut command = String::new();

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
                println!("{}", echo_string.trim_start());
            }
            _ => {
                println!("{}: command not found", command);
            }
        }
    }
}
