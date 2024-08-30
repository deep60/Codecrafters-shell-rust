#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    loop  {
        print!("$ ");
        io::stdout().flush().unwrap();
        // println!("{}: command not found", input.strip_suffix("\n").unwrap());
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let command = input.trim();

        if command.is_empty() {
            continue;
        } else if command == "exit 0" {
            break;
        } else {
            println!("{}: command not found", command);
        }
        
    }
}
