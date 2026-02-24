use std::io::{self, Write};

mod builtins;
use builtins::{exec_cd, exec_command, exec_echo, exec_pwd, exec_type};

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
    let (command, args) = process_line(line);
    match command {
        "echo" => exec_echo(&args),
        "type" => exec_type(&args),
        "pwd" => exec_pwd(&args),
        "cd" => exec_cd(&args),
        _ => exec_command(command, &args),
    }
}

fn process_line(line: &str) -> (&str, Vec<String>) {
    let first_space = line.find(' ').unwrap_or(line.len());
    let command = &line[..first_space];

    if first_space >= line.len() {
        return (command, vec![]);
    }

    let mut args: Vec<String> = Vec::new();
    let line_chars: Vec<char> = line.chars().collect();
    let mut i = first_space + 1;
    let mut quote = None;
    let mut stack = String::new();
    while i < line.len() {
        let c = line_chars[i];
        if c == '\'' {
            if quote.is_some() && quote.unwrap() != '\'' {
                stack.push(c);
                i += 1;
                continue;
            }
            if i < line.len() - 1 && line_chars[i + 1] == '\'' {
                i += 2;
                continue;
            } else if quote.is_none() {
                if !stack.is_empty() {
                    args.push(stack.clone());
                }
                stack.clear();
                quote = Some('\'');
                i += 1;
            } else {
                if !stack.is_empty() {
                    args.push(stack.clone());
                }
                stack.clear();
                quote = None;
                i += 1;
            }
        } else if c == '\"' {
            if quote.is_some() && quote.unwrap() != '\"' {
                stack.push(c);
                i += 1;
                continue;
            }
            if i < line.len() - 1 && line_chars[i + 1] == '\"' {
                i += 2;
                continue;
            } else if quote.is_none() {
                if !stack.is_empty() {
                    args.push(stack.clone());
                }
                stack.clear();
                quote = Some('\"');
                i += 1;
            } else {
                if !stack.is_empty() {
                    args.push(stack.clone());
                }
                stack.clear();
                quote = None;
                i += 1;
            }
        } else if c == ' ' && quote.is_none() {
            if !stack.is_empty() {
                args.push(stack.clone());
            }
            stack.clear();
            i += 1;
        } else {
            stack.push(c);
            i += 1;
        }
    }

    if !stack.is_empty() {
        args.push(stack.clone());
    }

    return (command, args);
}
