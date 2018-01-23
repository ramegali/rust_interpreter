extern crate regex;

use std::io;
use self::regex::Regex;
use std::collections::BTreeMap;

pub struct Lexer<'a> {
    contents: &'a str,
    tokens: Vec<Regex>,
    pub token_tree: BTreeMap<&'a str, i32>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        // tokens
        let ID              = Regex::new(r"[a-zA-Z]\w*").unwrap();
        let INT_LITERAL     = Regex::new(r"[0-9]+").unwrap();
        let OPEN_BRACE      = Regex::new(r"\{").unwrap();
        let CLOSE_BRACE     = Regex::new(r"\}").unwrap();
        let OPEN_PAREN      = Regex::new(r"\(").unwrap();
        let CLOSE_PAREN     = Regex::new(r"\)").unwrap();
        let SEMICOLON       = Regex::new(r";").unwrap();
        //let INT             = Regex::new(r"int").unwrap();
        //let RETURN          = Regex::new(r"return").unwrap();

        Lexer { contents: input,
                token_tree: BTreeMap::new(),
                tokens: [ ID, INT_LITERAL, OPEN_BRACE, CLOSE_BRACE, 
                        OPEN_PAREN, CLOSE_PAREN, SEMICOLON ].to_vec(), 
            }
    }

    pub fn build_token_tree(&mut self) -> BTreeMap<&'a str, i32> {
        //let mut temp_contents = self.contents.to_owned();

        for token in &self.tokens {
            if token.is_match(self.contents) {
                for cap in token.captures_iter(self.contents) {
                    *self.token_tree.entry(cap.get(0).unwrap().as_str())
                        .or_insert(0) += 1;
                }
                // remove regex matches from string as we go for efficiency
                //temp_contents = (*token.replace_all(self.contents, " ")).to_owned();
                //self.contents = &token.replace_all(self.contents, " ");
            }
        }
        self.token_tree.clone()
    }

    pub fn analyze(&mut self) {
        for c in self.contents.chars() {
            // remove the first item
            //let c = self.contents.to_string().remove(0);
            if c.is_alphabetic() {
                //&mut self.get_id(c);
                println!("Got a letter: {:?}", c);
            } else if c.is_numeric() {
                println!("Got a number: {:?}", c);
            } else {
                println!("Got something else: {:?}", c);
            }
        }
    }

    fn get_id(&mut self, c: char) {
        let mut identifier = String::new();

        while c.is_alphabetic() {
            identifier.push(c);
            self.contents.to_string().remove(0);
            println!("{:?}", c);
        }
        println!("Identifier: {:?}", identifier);
    }


    pub fn print_token_tree(&self) {
        for (token, count) in &self.token_tree {
            println!("{} : {}", token, count);
        }
    }

    pub fn search_token_tree(&self) {
        println!(">> Search AST for: ");

        let mut search_tok = String::new();
        io::stdin().read_line(&mut search_tok)
            .expect("Failed to read line");

        if self.token_tree.contains_key(&search_tok.as_str().trim()) {
            println!("AST contains {} instance(s) of {}", self.token_tree[&*search_tok.as_str().trim()], search_tok);
        } else {
            println!("Token not found!");
        }
    }

}