use std::io::{self, prelude::*};

const GREETING: &str = "Welcome in Pasifal, it is tiny\
                       chess engine in Rust. Press Enter for\
                       starting game, if you want to choose you side\
                       just type it here, by default you will play\
                       in white side. For moves you should use\
                       PGN format, yet I can't recognize other way.\
                       You can check it here - https://en.wikipedia.org/wiki\
                       /Portable_Game_Notation\
                        ";

fn main() {
    loop {
        let mut input = String::new();
        println!("{:?}", GREETING);
        let stdin = io::stdin();
        stdin.lock().read_line(&mut input).unwrap();
        println!("You typed: {}", input);
    }
}
