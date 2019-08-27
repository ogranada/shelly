use std::io::{stdin, stdout, Write};
use std::process::{Command};
// use std::result::{Result};

#[derive(Debug)]
enum EAction {
    EXIT,
    ERROR,
    NONE
}

fn execute_command(command: &str) -> EAction {
    let cmd = String::from(command);
    if cmd.trim().len() == 0 {
        return EAction::NONE;
    }
    let mut return_value = match cmd.trim().as_ref() {
        "exit" => EAction::EXIT,
        _ => EAction::NONE,
    };
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

fn main() {
    let prompt = ">>> ";
    print!("💻  Shelly Terminal\n");
    loop {
        let mut input = String::new();
        print!("\n{}", prompt);
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        let command = input.trim();
        let out = execute_command(command);
        match out {
            EAction::ERROR => {
                print!("\nGood Bye!\n");
                std::process::exit(0);
            },
            _ => (/* Nothing to do... */),
        }
    }
}
