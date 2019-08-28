use std::process::{Command};
use std::process::{exit};

// use std::io;
use std::path::Path;
use std::fs;

// use std::result::{Result};
// use std::mem::{discriminant};

// NON STANDARD
extern crate prettytable;
extern crate rustyline;

use prettytable::{Table, Row, Cell, Attr, color};

use rustyline::error::ReadlineError;
use rustyline::Editor;



#[derive(Debug)]
enum EAction {
    EXIT,
    ERROR,
    NONE
}

fn show_directory_info(directory: &Path) -> EAction {
    let mut table = Table::new();
    // Add a row per time
    table.add_row(
        Row::new(vec![
            Cell::new("File")
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::GREEN)),
            Cell::new("Type")
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::GREEN)),
            Cell::new("Created")
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::GREEN)),
            Cell::new("Modified")
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::GREEN))
        ])
    );

    if directory.is_dir() {
        let elms = fs::read_dir(directory).unwrap();
        for entry in elms {
            let file = entry.unwrap();
            // println!("{:?}", file.path());
            let pb: std::path::PathBuf = file.path();
            let meta = pb.metadata().unwrap();
            let meta_filetype = meta.file_type();
            let created: String = format!("{:?}", meta.created().unwrap().elapsed().unwrap());
            let modified: String = format!("{:?}", meta.modified().unwrap().elapsed().unwrap());
            let filetype = if meta_filetype.is_dir() {
                "directtory"
            } else if meta_filetype.is_symlink() {
                "symlink"
            } else {
                "file"
            };
            let file_format = if meta_filetype.is_dir() {
                Attr::ForegroundColor(color::BLUE)
            } else if meta_filetype.is_symlink() {
                Attr::ForegroundColor(color::BRIGHT_CYAN)
            } else {
                Attr::Dim
            };
            // 
            table.add_row(
                Row::new(vec![
                    Cell::new(pb.to_str().unwrap())
                        .with_style(file_format),
                    Cell::new(filetype),
                    Cell::new(created.trim()),
                    Cell::new(modified.trim())
                ])
            );
        }
    }
    // Print the table to stdout
    table.printstd();

    EAction::NONE
}

fn list_directory(params: Vec<&str>) -> EAction {
    if params.len() == 0 {
        show_directory_info(Path::new("."));
    } else {
        for pth in params {
            show_directory_info(Path::new(pth));
        }
    }
    EAction::NONE
}
fn run_subprocess(command: &str) -> EAction {
    let mut return_value = EAction::NONE;
    let child = Command::new(command).spawn();
    if child.is_ok() {
        let res = child.unwrap().wait();
        if res.is_err() {
            println!("\nError running command: {}", res.unwrap_err());
            return_value = EAction::ERROR;
        }
    } else {
        println!("\nFailure running command \"{}\": {}", command, child.unwrap_err());
        return_value = EAction::ERROR;
    }
    return_value
}

fn execute_command(command: &str, params: Vec<&str>) -> EAction {
    let cmd = String::from(command);
    if cmd.trim().len() == 0 {
        return EAction::NONE;
    }
    let return_value = match cmd.trim().as_ref() {
        "quit" => EAction::EXIT,
        "exit" => EAction::EXIT,
        "ls" => list_directory(params),
        _ => run_subprocess(command),
    };
    return_value
}

fn main() {
    if cfg!(windows) {
        println!("this is windows");
    } else if cfg!(unix) {
        println!("this is unix alike");
    }
    let prompt = ">>> ";
    print!("💻  Shelly Terminal\n");

    let mut rl = Editor::<()>::new();
    if rl.load_history(".history").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(prompt);
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let mut parts = line.trim().split_whitespace();
                let command = parts.next().unwrap();

                let args = parts;
                let params: Vec<&str> = args
                    .filter( |wd| !wd.starts_with("--") && !wd.starts_with("-") )
                    .collect()
                    ;
                let out = execute_command(command, params);
                match out {
                    EAction::EXIT => {
                        print!("\nGood Bye!\n");
                        exit(0);
                    },
                    _ => (/* Nothing to do... */),
                }
            },
            Err(ReadlineError::Interrupted) => {
                print!("\nGood Bye!\n");
                break
            },
            Err(ReadlineError::Eof) => {
                print!("\nGood Bye!\n");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history(".history").unwrap();
}
