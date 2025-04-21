use std::{env, fs, io};
use std::io::Write;
use std::process::ExitCode;
use crate::scanner::Scanner;

mod token;
mod scanner;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        len if len > 2 => {
            return ExitCode::from(64);
        }
        2 => {
            run_file(&args[0]);
            ExitCode::SUCCESS
        }
        _ => {
            run_prompt();
            ExitCode::SUCCESS
        }
    }
}

fn run_file(path: &String) {
    match fs::read_to_string(path) {
        Ok(content) => run(content),
        Err(e) => eprintln!("Error reading file: {}", e)
    }
    // Need some kind of mechanism for returning errors
    // ExitCode::from(65)
}

fn run_prompt() {
    loop {
        print!("> "); io::stdout().flush().unwrap();
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(_) => {
                if line == "" { break; }

                run(line);
            }
            Err(_) => {
                println!("Error reading line");
                break;
            }
        }
    }
}

fn run(source: String) {
    let mut scanner = Scanner { 
        source: source,
        tokens: Vec::new(),
        start: 0,
        current: 0,
        line: 1
    };
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token)
    }
}

pub fn error(line: u32, message: String) {
    report(line, "", message);
}

pub fn report(line: u32, location: &str, message: String) {
    println!("[line {}] Error{}: {}", line, location, message);
    let had_error = true;
}
