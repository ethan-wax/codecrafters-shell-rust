use std::env;
use std::fs;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use std::path::Path;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Issue reading from stdin");

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
        Some(&line[first_space + 1..])
    } else {
        None
    };
    match command {
        "echo" => exec_echo(args),
        "type" => exec_type(args),
        "pwd" => exec_pwd(),
        _ => exec_command(command, args),
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
        return;
    };

    match type_args {
        "echo" | "exit" | "type" | "pwd" => println!("{type_args} is a shell builtin"),
        _ => search_path(type_args),
    }
}

fn search_path(type_args: &str) {
    if let Some(path) = find_executable(type_args) {
        println!("{type_args} is {path}")
    } else {
        println!("{type_args}: not found")
    }
}

fn find_executable(exec: &str) -> Option<String> {
    let path = env::var("PATH").expect("PATH is not set.");
    let dirs = path.split(':');

    for dir in dirs {
        let full = dir.to_owned() + "/" + exec;
        let p = Path::new(&full);
        let exists = p.try_exists().expect("Error searching path");
        if exists {
            let meta = fs::metadata(&full).expect("Error reading file metadata");
            let mode = meta.permissions().mode();
            let executable = mode & 0o111 != 0;
            if executable {
                return Some(full);
            }
        }
    }
    return None;
}

fn exec_command(command: &str, args: Option<&str>) {
    let Some(_) = find_executable(command) else {
        println!("{}: command not found", command);
        return;
    };

    let mut arg_vec: Vec<&str> = Vec::new();

    if let Some(arg_str) = args {
        for arg in arg_str.split(' ') {
            arg_vec.push(arg);
        }
    }

    Command::new(command)
        .args(arg_vec)
        .status()
        .expect("Error executing command.");
}

fn exec_pwd() {
    let borrowed_current_dir = &env::current_dir().expect("Error reading current directory");
    let current_dir = borrowed_current_dir.display();
    println!("{current_dir}")
}