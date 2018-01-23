pub mod interpreter;
pub mod lexer;

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process;
//use interpreter::interpreter::interpreter;
use lexer::Lexer;

fn main() {
    
    let contents = get_file_contents();
    let mut lexer = Lexer::new(&contents);
    lexer.analyze();
    // lexer.build_token_tree();
    // lexer.print_token_tree();

    // loop {
    //     lexer.search_token_tree();
    // }
}

fn get_file_contents() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("ERROR: Must specify file name");
        process::exit(1);
    }

    let file_name = &args[1];
    println!("Opening file: {}", file_name);
    let mut f = File::open(file_name).expect("File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something went wrong when reading the file");

    contents
}


