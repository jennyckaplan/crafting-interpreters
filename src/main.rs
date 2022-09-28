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

fn run(source: &str) {
    let mut scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens().unwrap();
    for token in tokens {
        println!("{}", token);
    }
}

fn run_prompt() -> Result<(), io::Error> {
    loop {
        print!("> ");
        io::stdout().flush()?;

        let stdin = io::stdin();
        let mut user_input = String::new();
        stdin.read_line(&mut user_input)?;

        if user_input.trim().is_empty() {
            break;
        }

        run(&user_input)
    }

    Ok(())
}

fn run_file(path: &str) -> Result<(), io::Error> {
    let source = read_to_string(path)?;

    run(&source);

    Ok(())
}

fn main() -> Result<(), io::Error> {
    // Skip the first argument, which is the path used to call the program
    let args: Vec<String> = env::args().skip(1).collect();

    match args.len() {
        0 => run_prompt()?,
        1 => run_file(&args[0])?,
        _ => {
            println!("Usage: jlox [script]");
            process::exit(exitcode::USAGE);
        }
    }

    Ok(())
}
