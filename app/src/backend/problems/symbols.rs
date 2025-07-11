use rand::{self, seq::IndexedRandom};
struct Symbol {
    char: char,
    weight: u8,
}

impl Symbol {
    const fn new(char: char, weight: u8) -> Symbol {
        Symbol { char, weight }
    }
}

pub struct Unknowns;

/// UNKNOWNS are used in equations and expressions - y is fine here
static UNKNOWNS: [Symbol; 13] = [
    Symbol::new('a', 3),
    Symbol::new('b', 1),
    Symbol::new('c', 1),
    Symbol::new('d', 1),
    Symbol::new('k', 2),
    Symbol::new('p', 1),
    Symbol::new('q', 1),
    Symbol::new('r', 1),
    Symbol::new('t', 3),
    Symbol::new('v', 1),
    Symbol::new('x', 30),
    Symbol::new('y', 2),
    Symbol::new('z', 1),
];

impl Unknowns {
    pub fn get_unknown() -> crate::Result<char> {
        let mut rng = rand::rng();
        UNKNOWNS
            .choose_weighted(&mut rng, |symbol| symbol.weight)
            .map(|symbol| symbol.char)
            .map_err(|_| crate::Error::EmptyStatic)
    }
}
