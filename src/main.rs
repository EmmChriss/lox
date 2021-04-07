extern crate thiserror;

mod error;

use error::Result;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Read};

fn main() -> Result<()> {
	let args: Vec<String> = env::args().collect();
	if args.len() > 2 {
		eprintln!("usage: lox [file]");
		std::process::exit(1);
	} else if args.len() == 2 {
		run_file(&args[1])
	} else {
		run_prompt()
	}
}

fn run_file(path: impl AsRef<Path>) -> Result<()> {
	let mut contents = String::new();
	let mut reader = BufReader::new(File::open(path)?);
	reader.read_to_string(&mut contents)?;
	
	Ok(())
}

fn run_prompt() -> Result<()> {
	let stdin = std::io::stdin();
	let mut buf = String::new();
	
	loop {
		print!("> ");
		if stdin.read_line(&mut buf)? == 0 {
			break;
		}
		
		
	}
	
	Ok(())
}

fn run(src: &str) -> Result<()> {
	let toks: Vec<Token> = Lexer::new(src).into_tokens();
	
	todo!();
}
