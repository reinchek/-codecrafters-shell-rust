#[allow(unused_imports)]
use std::io::{self, Write};

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

        println!("{}: command not found", command);
    }
}
