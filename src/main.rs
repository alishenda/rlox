use std::env;
use std::process::exit;
use std::io;

mod lexer;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(path: &String) {

    //TODO: Read a file
}

fn run_prompt() {

    loop {
        let mut line = String::new();
        print!("> ");
        io::stdin().read_line(&mut line).expect("Failed to read line");
        if line.is_empty() { break; };
        println!("{}", &mut line);
    }
}
