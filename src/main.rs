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
    let first_space = line.find(' ').unwrap_or(line.len());
    let command = &line[..first_space];
    let args = if first_space < line.len() {
        Some(&line[first_space+1..])   
    } else {
        None
    };
    match command {
        "echo" => exec_echo(args),
        "type" => exec_type(args),
        _ => println!("{}: command not found", line)
    }
}

fn exec_echo(args: Option<&str>) {
    if let Some(arg) = args {
        println!("{arg}")
    } else {
        println!("")
    }
}

fn exec_type(args: Option<&str>) {
    let Some(type_args) = args else {
        println!("Type expects an argument");
        return
    };

    match type_args {
        "echo" | "exit" | "type" => println!("{type_args} is a shell builtin"),
        _ => println!("{type_args}: not found")
    }
}