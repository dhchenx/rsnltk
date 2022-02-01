use std::io;
use std::io::prelude::*;
use crate::native::toksiter::*;
use crate::native::chardata::*;

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use crate::native::word2vec::*;
    use crate::native::toksiter::*;
    use crate::native::chardata::*;
    use crate::native::token::get_token_list;

    # [test]
    fn token_analyze(){
        let mut s="hello world!";

        let mut chs = s.chars();
        let mut chds = CharDataIter::new(&mut chs);
        let mut toks = TokenIter::new(&mut chds);

        // Run the tokenizer, dump debug info for each token:
        loop {
            match toks.next() {
                Some(tok) => { println!("{:?}", tok) },
                None => { println!("<END_OF_TEXT>"); break; }
            }
        }
    }

    # [test]
    fn test_get_token_list(){
        let s="Hello, Rust. How are you?";
        let result=get_token_list(s);
        for r in result{
            println!("{}\t{:?}",r.text,r);
        }
    }

}

pub fn get_token_list(s:&str)->Vec<Token>{

    let mut chs = s.chars();
    let mut chds = CharDataIter::new(&mut chs);
    let mut toks = TokenIter::new(&mut chds);
    let mut list_token:Vec<Token>=Vec::new();
    // Run the tokenizer, dump debug info for each token:
    loop {
        match toks.next() {
            Some(tok) => {
                println!("{:?}", tok);
                list_token.push(tok);
            },
            None => { println!("<END_OF_TEXT>"); break; }
        }
    }
    list_token

}



fn main() {
    // Get stdin into a string
    let stdin = io::stdin();
    let mut s = String::new();
    stdin.lock().read_to_string(&mut s).unwrap();
    println!("{}", s);

    // Construct a tokenizer by adapting some more primitive iterators
    let mut chs = s.chars();
    let mut chds = CharDataIter::new(&mut chs);
    let mut toks = TokenIter::new(&mut chds);

    // Run the tokenizer, dump debug info for each token:
    loop {
        match toks.next() {
            Some(tok) => { println!("{:?}", tok) },
            None => { println!("<END_OF_TEXT>"); break; }
        }
    }
}
