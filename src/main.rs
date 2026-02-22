#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Issue reading from stdin");
        
        let line = input.trim();

        if line.starts_with("exit") {
            break;
        }

        println!("{}: command not found", line);
    }
}
