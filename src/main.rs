#![feature(globs)]

extern crate solitaire;

fn main() {
    let message = "AAAAA AAAAA";
    let mut d = solitaire::Deck::with_key("FOO");
    let y = d.encrypt(message);
    println!("{}", y);
}
