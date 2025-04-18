use std::{env, io};
use std::path::{Path, PathBuf};

mod token;
mod scanner;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        len if len > 1 => {
            println!("Usage: rlox [script]");
            return ExitCode::from(64);
        }
        1 => run_file(args[0]),
        _ => run_prompt(),
    }
}

fn run_file(path: String) {
    let path_buf = PathBuf::from(path);
    let bytes = fs::read(path_buf)?;

    run(String::from_utf8(bytes)?);
    // Need some kind of mechanism for returning errors
    // ExitCode::from(65)
}

fn run_prompt() {
    while(true) {
        print("> ");
        let mut buffer = String::new();
        let line = io::stdin().read_line(&mut buffer)?;
        if line.is_none() { break; }

        run(line);
    }
}

fn run(source: String) {
    let scanner = Scanner { source: source };
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!(token)
    }
}

fn error(line: u32, message: String) {
    report(line, "", message);
}

fn report(line: u32, location: &str, message: String) {
    println!("[line {}] Error{}: {}", line, location, message);
    let had_error = true;
}
