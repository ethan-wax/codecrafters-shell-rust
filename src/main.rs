#[allow(unused_imports)]
use std::io::{self, Write};


fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Issue reading from stdin");
        
        let line = input.trim();

        // Hardcode exit to get out of loop
        if line.starts_with("exit") {
            break;
        }

        eval(line);
    }
}

fn eval(line: &str) {
    if line.starts_with("echo ") {
        let text = &line[5..];
        println!("{}", text);
    } else {
        println!("{}: command not found", line);
    }
}