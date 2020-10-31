use anyhow::Result;
use salal::scanner::Scanner;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process::exit;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        println!("Usage: salal [script]");
        exit(64);
    } else if args.len() == 1 {
        run_file(&args[0])?;
    } else {
        run_prompt()?;
    }
    Ok(())
}

fn run_file(filename: &str) -> Result<()> {
    let mut file = File::open(filename)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    run(buffer)?;
    Ok(())
}
fn run_prompt() -> Result<()> {
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        run(buffer)?;
    }
}
fn run(source: String) -> Result<()> {
    println!("{}", source);
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan()?;
    println!("{:#?}", tokens);
    Ok(())
}
