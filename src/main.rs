use std::{
    error::Error, fs, io::{stdout, Write}, str
};

mod lexer;
mod token;
mod error;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() > 2 {
        println!("Usage: {} [script]", args[0]);
    } else if args.len() == 2 {
        run_file(args[1].as_str());
    } else {
        run_prompt();
    }
}

fn run_file(path: &str) -> Result<(), Box<dyn Error>> {
    let byte_content = fs::read(path)?;
    let content = str::from_utf8(&byte_content)?;
    run(content)?;
    Ok(())
}

fn run_prompt() -> Result<(), Box<dyn Error>> {
    loop {
        print!("> ");
        stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if(input.is_empty()) {
            return Ok(()); // EOF
        }

        match run(input.as_str()) {
            Ok(_) => {},
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn run(source: &str) -> Result<(), Box<dyn Error>> {
    let lexer = lexer::Lexer::new(source);
    for token in lexer {
        match token {
            Ok(token) => println!("{:?}", token),
            Err(e) => eprintln!("{}", e),
        }
    }
    Ok(())
}

fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, location: &str, message: &str) {
    eprintln!("[line {}] Error {}: {}", line, location, message);
}