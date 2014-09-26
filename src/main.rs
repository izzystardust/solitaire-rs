#![feature(globs)]

mod solitaire;

fn main() {
    let message = "AAAAA AAAAA";
    let mut d = solitaire::Deck::new();
    let y = d.encrypt(message);
    println!("{}", y);
}
