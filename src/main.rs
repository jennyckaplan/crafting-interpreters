extern crate exitcode;

use std::{
    env,
    fs::read_to_string,
    io::{self, Write},
    process, str,
};

mod scanner;
mod token;

use scanner::Scanner;

#[derive(Debug)]
enum AppError {
    IoError(io::Error),
    ScannerError(scanner::ScannerError),
}

fn scan(source: &str) -> Result<(), AppError> {
    let mut scanner = Scanner::new(source.to_string());
    let tokens = match scanner.scan_tokens() {
        Ok(tokens) => tokens,
        Err(err) => return Err(AppError::ScannerError(err)),
    };
    for token in tokens {
        println!("{}", token);
    }

    Ok(())
}

fn scan_prompt() -> Result<(), AppError> {
    loop {
        print!("> ");
        match io::stdout().flush() {
            Ok(_) => {}
            Err(err) => return Err(AppError::IoError(err)),
        }

        let stdin = io::stdin();
        let mut user_input = String::new();
        match stdin.read_line(&mut user_input) {
            Ok(_) => {}
            Err(err) => return Err(AppError::IoError(err)),
        }

        if user_input.trim().is_empty() {
            break;
        }

        scan(&user_input)?
    }

    Ok(())
}

fn scan_file(path: &str) -> Result<(), AppError> {
    let source = match read_to_string(path) {
        Ok(source) => source,
        Err(err) => return Err(AppError::IoError(err)),
    };

    scan(&source)?;

    Ok(())
}

fn run() -> Result<(), AppError> {
    // Skip the first argument, which is the path used to call the program
    let args: Vec<String> = env::args().skip(1).collect();

    match args.len() {
        0 => scan_prompt()?,
        1 => scan_file(&args[0])?,
        _ => {
            println!("Usage: jlox [script]");
            process::exit(exitcode::USAGE);
        }
    }

    Ok(())
}

fn main() {
    match run() {
        Ok(_) => process::exit(exitcode::OK),
        Err(err) => match err {
            AppError::IoError(err) => {
                println!(
                    "Uh oh, something went wrong reading your file or input. {}",
                    err
                );
                process::exit(exitcode::IOERR);
            }
            AppError::ScannerError(err) => {
                println!("{}", err);
                process::exit(exitcode::DATAERR);
            }
        },
    }
}
