// use std::io::{self, Split, Write};

// fn main() {
//     let stdin = io::stdin();

//     let path_env = std::env::var("PATH").unwrap();

//     loop {
//         print!("$ ");
//         io::stdout().flush().unwrap();

//         let mut input = String::new();
//         stdin.read_line(&mut input).unwrap();

//         let argv = input.split_whitespace().collect::<Vec<&str>>();

//         if argv.is_empty() {
//             continue;
//         }

//         let builtins = ["exit", "echo", "type"];

//         match argv[0] {
//             "exit" => break,
//             "echo" => {
//                 println!("{}", argv[1..].join(" "));
//             }
//             "type" => {
//                 if argv.len() != 2 {
//                     println!("type: expected 1 argument, got {}", argv.len() - 1);
//                     continue;
//                 }

//                 let cmd = argv[1];
//                 if builtins.contains(&cmd) {
//                     println!("{} is a shell builtin", cmd);
//                 } else {
//                     //println!("{} not found", cmd);
//                     let mut found = false;
//                     for path in path_env.split(':') {
//                         let full_path = format!("{}/{}", path, cmd);
//                         if std::fs::metadata(&full_path).is_ok() {
//                             println!("{cmd} is {full_path}");
//                             found = true;
//                             break;
//                         }
//                     }
//                     if !found {
//                         println!("{cmd}: not found")
//                     }
//                     // let split = &mut path_env.split(':');
//                     // if let Some(path) = split.find(|path| std::fs::metadata(format!("{}/{}", path, cmd)).is_ok()) {
//                     //     println!("{cmd} is {path}/{cmd}");
//                     // } else {
//                     //     println!("{cmd} not found");
//                     // }
//                 }
//             }
//             _ => {
//                 println!("{}: command not found", argv[0])
//             }
//         }
//     }
// }

use std::env;
use std::env::set_current_dir;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

fn get_path() -> Vec<String> {
    let key = "PATH";
    match env::var_os(key) {
        Some(paths) => env::split_paths(&paths)
            .map(|p| p.to_str().unwrap().to_string())
            .collect(),
        None => Vec::new(),
    }
}

fn file_on_path(file: &str) -> Option<String> {
    let paths = get_path();
    for path in paths {
        let file_check = format!("{}/{}", path, file);
        if Path::new(&file_check).exists() {
            return Some(file_check);
        }
    }

    None
}

fn get_home() -> Option<String> {
    let key = "HOME";
    match env::var_os(key) {
        Some(path) => {
            let Ok(str_path) = path.into_string() else {
                return None;
            };
            Some(str_path)
        }
        None => None,
    }
}

fn main() {
    get_path();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let (command, argument) = input
            .trim()
            .split_once(" ")
            .unwrap_or_else(|| (input.trim(), ""));

        match command {
            "exit" => break,
            "echo" => println!("{}", argument),
            "type" => {
                if ["exit", "echo", "type", "pwd"].contains(&argument) {
                    println!("{} is a shell builtin", argument)
                } else if let Some(file) = file_on_path(argument) {
                    println!("{} is {}", argument, file)
                } else {
                    println!("{}: not found", argument)
                }
            }
            "pwd" => {
                let path = match env::current_dir() {
                    Ok(t) => t,
                    Err(e) => {
                        println!("Error: {:?}", e);
                        continue;
                    }
                };
                println!("{}", path.display());
            }
            "cd" => {
                let path = if argument.contains("~") {
                    let Some(home) = get_home() else {
                        println!("cd: {}: No such file or directory", argument);
                        continue;
                    };
                    argument.replace("~", &home)
                } else {
                    argument.to_string()
                };

                match set_current_dir(path) {
                    Ok(_) => continue,
                    Err(_) => println!("cd: {}: No such file or directory", argument),
                }
            }
            _ => {
                if let Some(_) = file_on_path(command) {
                    let output = Command::new("sh").arg("-c").arg(&input).output().unwrap();

                    let fmt_output = output
                        .stdout
                        .into_iter()
                        .map(|c| c as char)
                        .collect::<String>();
                    println!("{}", fmt_output.trim());
                } else {
                    println!("{}: command not found", command)
                }
            }
        };
    }
}