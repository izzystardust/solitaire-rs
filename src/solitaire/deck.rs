use solitaire::card::*;

use std::char;
use std::cmp;
use std::iter::range_inclusive;

#[deriving(PartialEq, Show)]
pub struct Deck {
    pub d: Vec<Card>,
}

impl Iterator<uint> for Deck {
    fn next(&mut self) -> Option<uint> {
        Some(self.gen_keystream_letter())
    }
}

impl Deck {
    pub fn new() -> Deck {
        let per_suit = 13;
        let mut deck = Deck { d: Vec::new() };
        for i in range_inclusive(1, per_suit) {
            deck.d.push(Clubs(i));
        }
        for i in range_inclusive(1, per_suit) {
            deck.d.push(Diamonds(i));
        }
        for i in range_inclusive(1, per_suit) {
            deck.d.push(Hearts(i));
        }
        for i in range_inclusive(1, per_suit) {
            deck.d.push(Spades(i));
        }
        
        deck.d.push(JokerA);
        deck.d.push(JokerB);
        return deck;
    }

    pub fn with_key(key: &str) -> Deck {
        let mut deck = Deck::new();
        let a = 'A' as uint - 1;
        let k = key.chars()
                   .filter(|x| char::is_alphabetic(*x))
                   .map(|x| char::to_uppercase(x))
                   .collect::<String>();
        for e in k.as_bytes().iter() {
            deck.with_key_helper(*e as uint - a);
        }
            
        return deck;
    }

    fn with_key_helper(&mut self, i: uint) {
        self.move_joker(JokerA, 1);
        self.move_joker(JokerB, 2);
        self.triple_cut();
        self.count_cut();
        let mut new = Vec::with_capacity(self.d.len());
        new.push_all(self.d.slice(i, self.d.len()-1));
        new.push_all(self.d.slice(0, i));
        new.push(*self.d.last().unwrap());
        self.d = new;
    }
    
    pub fn encrypt(&mut self, plain: &str) -> String {
        let a = 'A' as u8;
        plain.chars().map(|x|
                          match char::is_alphabetic(x) {
                              true => (((x as u8 - a + self.gen_keystream_letter() as u8) % 26) + a) as char,
                              false => x,
                          }).collect::<String>()
    }

    pub fn gen_keystream_letter(&mut self) -> uint {
        self.move_joker(JokerA, 1);
        self.move_joker(JokerB, 2);
        self.triple_cut();
        self.count_cut();
        let i = self.d[0].count_index();
        match self.d[i].count_index() {
            53 => self.gen_keystream_letter(),
            i => i,
        }
    }

    pub fn move_joker(&mut self, j: Card, count: int) {
        let mut i = self.d.as_slice().position_elem(&j).unwrap();
        for _ in range(0i, count) {
            if i < self.d.len()-1 {
                self.d.as_mut_slice().swap(i, i+1);
            } else {
                // if card is the last card in the deck, move below the first card of the deck
                // every card needs to be moved... maybe this should have been a linked list
                let mut new = Vec::with_capacity(self.d.len());
                new.push(self.d[0]);
                new.push(*self.d.last().unwrap());
                new.push_all(self.d.slice(1, self.d.len()-1));
                self.d = new;
            }
            i = if i != self.d.len()-1 {
                i + 1
            } else {
                1
            };
        }
    }

    pub fn triple_cut(&mut self) {
        let mut new = Vec::with_capacity(self.d.len());
        let ai = self.d.as_slice().position_elem(&JokerA).unwrap();
        let bi = self.d.as_slice().position_elem(&JokerB).unwrap();
        let first = cmp::min(ai, bi);
        let secnd = cmp::max(ai, bi);
        new.push_all(self.d.slice(secnd+1, self.d.len()));
        new.push_all(self.d.slice(first, secnd + 1));
        new.push_all(self.d.slice(0, first));
        self.d = new;
    }

    pub fn count_cut(&mut self) {
        // cut after c
        let i = self.d.last().unwrap().count_index();
        let mut new = Vec::with_capacity(self.d.len());
        new.push_all(self.d.slice(i, self.d.len()-1));
        new.push_all(self.d.slice(0, i));
        new.push(*self.d.last().unwrap());
        self.d = new;
    }

}

#[test]
fn new_deck() {
    let d = Deck::new();
    assert_eq!(d, Deck { d: vec![Clubs(1), Clubs(2), Clubs(3), Clubs(4), Clubs(5), Clubs(6), Clubs(7), Clubs(8), Clubs(9), Clubs(10), Clubs(11), Clubs(12), Clubs(13), Diamonds(1), Diamonds(2), Diamonds(3), Diamonds(4), Diamonds(5), Diamonds(6), Diamonds(7), Diamonds(8), Diamonds(9), Diamonds(10), Diamonds(11), Diamonds(12), Diamonds(13), Hearts(1), Hearts(2), Hearts(3), Hearts(4), Hearts(5), Hearts(6), Hearts(7), Hearts(8), Hearts(9), Hearts(10), Hearts(11), Hearts(12), Hearts(13), Spades(1), Spades(2), Spades(3), Spades(4), Spades(5), Spades(6), Spades(7), Spades(8), Spades(9), Spades(10), Spades(11), Spades(12), Spades(13), JokerA, JokerB] });
}

#[test]
fn move_joker() {
    let mut d = Deck{ d: vec![Clubs(1), Clubs(2), JokerA] };
    d.move_joker(JokerA, 1);
    assert_eq!(d.d, vec![Clubs(1), JokerA, Clubs(2)]);

    let mut d = Deck{ d: vec![Clubs(1), Clubs(2), Hearts(3), JokerA, JokerB] };
    d.move_joker(JokerA, 1);
    d.move_joker(JokerB, 2);
    assert_eq!(d.d, vec![Clubs(1), JokerB, Clubs(2), Hearts(3), JokerA]);
}

#[test]
fn triple_cut() {
    let tests = vec![
        (vec![Clubs(1), JokerA, Clubs(2), JokerB, Clubs(3)], vec![Clubs(3), JokerA, Clubs(2), JokerB, Clubs(1)]),
        (vec![Clubs(1), Hearts(1), JokerA, Clubs(2), JokerB, Clubs(3), Hearts(3)], vec![Clubs(3), Hearts(3), JokerA, Clubs(2), JokerB, Clubs(1), Hearts(1)]),
        (vec![JokerA, JokerB], vec![JokerA, JokerB]),
        ];

    for test in tests.iter() {
        let (initial, expect) = test.clone();
        let mut got = Deck{ d: initial };
        got.triple_cut();
        assert_eq!(got.d, expect);
    }
}

#[test]
fn gen_keystream_letter() {
    let mut d = Deck::new();
    let expects = vec![4, 49, 10, 24, 8, 51, 44, 6, 4, 33];
    for e in expects.iter() {
        assert_eq!(d.gen_keystream_letter(), *e);
    }
}

#[test]
fn count_cut() {
    let mut d = Deck{ d: vec![Clubs(7), Clubs(3), Clubs(13), Clubs(14), Clubs(15), Clubs(16), Clubs(17), Clubs(18), Clubs(4), Clubs(5), Clubs(8), Clubs(9)]};
    let expect = Deck{ d: vec![Clubs(5), Clubs(8), Clubs(7), Clubs(3), Clubs(13), Clubs(14), Clubs(15), Clubs(16), Clubs(17), Clubs(18), Clubs(4), Clubs(9)] };
    d.count_cut();
    assert_eq!(d, expect);
}

#[test]
fn key() {
    let mut d = Deck::with_key("FOO");
    let expects = vec![8, 19, 7, 25, 20, 9, 8, 22, 32, 43, 5, 26, 17, 38, 48];
    for e in expects.iter() {
        assert_eq!(d.gen_keystream_letter(), *e);
    }
}
