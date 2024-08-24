#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut command = String::new();
    stdin.read_line(&mut command).unwrap();
    command.pop();

    println!("{}: command not found", command);
}
