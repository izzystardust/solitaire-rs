#![feature(globs)]

use std::char;

mod solitaire;

fn main() {
    let message = "AAAAA AAAAA";
    let mut d = solitaire::Deck::new();
    let y = message.chars().map(|x|
                                match char::is_alphabetic(x) {
                                    true => {
                                        (((x as u8 - 63 + d.gen_keystream_letter() as u8) % 26) + 63) as char
                                    },
                                    false => x,
                                }
                                ).collect::<String>();
    println!("{}", y);
}
