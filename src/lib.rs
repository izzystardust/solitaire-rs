#![feature(globs)]
use card::*;

use std::rand;
use std::rand::Rng;

pub mod card;

#[deriving(PartialEq, Show)]
pub struct Deck {
    d: Vec<Card>,
}


impl Deck {
    pub fn new() -> Deck {
        let per_suit = 13;
        let mut deck = Deck { d: Vec::new() };
        for i in range(0i, per_suit) {
            deck.d.push(Clubs(i+1));
        }
        for i in range(0i, per_suit) {
            deck.d.push(Diamonds(i+1));
        }
        for i in range(0i, per_suit) {
            deck.d.push(Hearts(i+1));
        }
        for i in range(0i, per_suit) {
            deck.d.push(Spades(i+1));
        }
        
        deck.d.push(JokerA);
        deck.d.push(JokerB);
        return deck;
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::task_rng();
        rng.shuffle(self.d.as_mut_slice());
    }
}

#[test]
fn new_deck() {
    let d = Deck::new();
    assert_eq!(d, Deck { d: vec![Clubs(1), Clubs(2), Clubs(3), Clubs(4), Clubs(5), Clubs(6), Clubs(7), Clubs(8), Clubs(9), Clubs(10), Clubs(11), Clubs(12), Clubs(13), Diamonds(1), Diamonds(2), Diamonds(3), Diamonds(4), Diamonds(5), Diamonds(6), Diamonds(7), Diamonds(8), Diamonds(9), Diamonds(10), Diamonds(11), Diamonds(12), Diamonds(13), Hearts(1), Hearts(2), Hearts(3), Hearts(4), Hearts(5), Hearts(6), Hearts(7), Hearts(8), Hearts(9), Hearts(10), Hearts(11), Hearts(12), Hearts(13), Spades(1), Spades(2), Spades(3), Spades(4), Spades(5), Spades(6), Spades(7), Spades(8), Spades(9), Spades(10), Spades(11), Spades(12), Spades(13), JokerA, JokerB] });
}

    
