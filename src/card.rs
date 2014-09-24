#[deriving(PartialEq, Show)]
pub enum Card {
    JokerA,
    JokerB,
    Clubs(int),
    Diamonds(int),
    Hearts(int),
    Spades(int),
}

impl Card {
    pub fn count_cut_index(&self) -> int {
        match *self {
            Clubs(i) => i,
            Diamonds(i) => i + 13,
            Hearts(i) => i + 26,
            Spades(i) => i + 39,
            JokerA => 53,
            JokerB => 53,
        }
    }

    pub fn is_joker(&self) -> bool {
        match *self {
            JokerA => true,
            JokerB => true,
            _ => false,
        }
    }

    pub fn is_red(&self) -> bool {
        match *self {
            Diamonds(_) => true,
            Hearts(_) => true,
            _ => false,
        }
    }
    
}

#[test]
fn test_count_cut_index() {
    let a = JokerA;
    assert_eq!(a.count_cut_index(), 53);
    let b = Hearts(4);
    assert_eq!(b.count_cut_index(), 30);
}
