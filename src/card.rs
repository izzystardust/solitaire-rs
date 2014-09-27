#[deriving(PartialEq, Show, Clone)]
pub enum Card {
    JokerA,
    JokerB,
    Clubs(uint),
    Diamonds(uint),
    Hearts(uint),
    Spades(uint),
}

impl Card {
    pub fn count_index(&self) -> uint {
        match *self {
            Clubs(i) => i,
            Diamonds(i) => i + 13,
            Hearts(i) => i + 26,
            Spades(i) => i + 39,
            JokerA => 53,
            JokerB => 53,
        }
    }
}

#[test]
fn test_count_cut_index() {
    let a = JokerA;
    assert_eq!(a.count_index(), 53);
    let b = Hearts(4);
    assert_eq!(b.count_index(), 30);
}
