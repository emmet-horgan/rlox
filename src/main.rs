use std::fs;
use std::io::Write;
mod scanner;
mod token;
use scanner::Scanner;

enum Error {
    Err,
    RtErr
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => {
            run_prompt();
        },

        2 => {
            run_file(&args[1]);
        },

        _ => {
            println!("Usage rlox [script]");
            std::process::exit(65);
        }
    }
}

fn run_file(file: &str) {
    let path = std::path::Path::new(file);
    let file_content = fs::read_to_string(path);
    match file_content {
        Ok(script) => {
            match run(&script) {
                Ok(_) => {},
                Err(e) => {
                    match e {
                        Error::Err => {
                            println!("General error occured");
                            std::process::exit(-1);
                        },
                        Error::RtErr => {
                            println!("Runtime error occured");
                            std::process::exit(-2);
                        }
                    }
                }
            }
            
        },
        Err(_) => {
            println!("Failed to find file: {}", file);
            std::process::exit(-1);
        }
    }
}

fn run_prompt() {
    let mut input_buffer = String::new();
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input_buffer).expect("Unable to read input");
        _ = run(&input_buffer);
    }
}

fn run(lox: &str) -> Result<(), Error> {
    Ok(())
}