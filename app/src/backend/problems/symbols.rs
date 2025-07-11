use rand::{self, seq::IndexedRandom};
use crate::Result;
struct Symbol {
    char: char,
    weight: u8,
}

impl Symbol {
    const fn new(char: char, weight: u8) -> Symbol {
        Symbol { char, weight }
    }
}

fn get_random(symbols: &[Symbol]) -> Result<char> {
let mut rng = rand::rng();
        symbols
            .choose_weighted(&mut rng, |symbol| symbol.weight)
            .map(|symbol| symbol.char)
            .map_err(|_| crate::Error::EmptyStatic)

}

pub struct Unknowns;
/// UNKNOWNS are used in equations and expressions - y is fine here
static UNKNOWNS: [Symbol; 12] = [
    Symbol::new('a', 3),
    Symbol::new('b', 1),
    Symbol::new('c', 1),
    Symbol::new('d', 1),
    Symbol::new('k', 2),
    Symbol::new('p', 1),
    Symbol::new('q', 1),
    Symbol::new('r', 1),
    Symbol::new('t', 3),
    Symbol::new('x', 30),
    Symbol::new('y', 2),
    Symbol::new('z', 1),
];

impl Unknowns {
    pub fn get_unknown() -> Result<char> {
        let mut rng = rand::rng();
        UNKNOWNS
            .choose_weighted(&mut rng, |symbol| symbol.weight)
            .map(|symbol| symbol.char)
            .map_err(|_| crate::Error::EmptyStatic)
    }
}

pub struct FunctionNames;
/// Function names are used for the form f(x) - note that y is not included here
/// since it is uncommon in Sweden to explicitly write y(x).
static FUNCTION_NAMES: [Symbol; 5] = [
    Symbol::new('f', 20),
    Symbol::new('g', 5),
    Symbol::new('h', 4),
    Symbol::new('s', 2),
    Symbol::new('v', 5),
];
impl FunctionNames {
    pub fn get_function_name() -> Result<char> {
        let mut rng = rand::rng();
        FUNCTION_NAMES
            .choose_weighted(&mut rng, |symbol| symbol.weight)
            .map(|symbol| symbol.char)
            .map_err(|_| crate::Error::EmptyStatic)
    }
}
