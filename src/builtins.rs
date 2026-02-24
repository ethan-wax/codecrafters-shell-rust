use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use std::path::Path;

pub fn exec_echo(args: &[String]) {
    for arg in args {
        print!("{arg} ");
    }
    println!("");
}

pub fn exec_type(args: &[String]) {
    if args.is_empty() {
        println!("Type expects an argument");
        return;
    }

    for arg in args {
        match arg.as_str() {
            "echo" | "exit" | "type" | "pwd" => println!("{arg} is a shell builtin"),
            _ => search_path(arg),
        }
    }
}

pub fn search_path(arg: &str) {
    if let Some(path) = find_executable(arg) {
        println!("{arg} is {path}")
    } else {
        println!("{arg}: not found")
    }
}

pub fn find_executable(exec: &str) -> Option<String> {
    let path = env::var("PATH").expect("PATH is not set");
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

pub fn exec_command(command: &str, args: &[String]) {
    if find_executable(command).is_none() {
        println!("{command}: command not found");
        return;
    }

    Command::new(command)
        .args(args)
        .status()
        .expect("Error executing command");
}

pub fn exec_pwd(args: &[String]) {
    if !args.is_empty() {
        println!("pwd: too many arguments");
        return;
    }

    let current_dir = &env::current_dir().expect("Error reading current directory");
    let current_dir_disp = current_dir.display();
    println!("{current_dir_disp}")
}

pub fn exec_cd(args: &[String]) {
    if args.len() > 1 {
        println!("cd: too many arguments");
        return;
    }

    let loc = if args.is_empty() { "~" } else { args[0].as_str() };
    let home = env::var("HOME").expect("Error reading from HOME");
    let path = if loc == "~" {Path::new(home.as_str()) } else {Path::new(loc)};
    if env::set_current_dir(path).is_err() {
        println!("cd: {loc}: No such file or directory");
        return;
    };
}