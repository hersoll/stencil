use anyhow::{Context, Result, anyhow};
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

/// Get a random unknown - the x in 3x + 1 = 10 or (2x + 1) - (x + 2)
pub fn get_unknown() -> Result<char> {
    get_random(&UNKNOWNS)
}

pub fn get_unknown_with_exclusions<T: Into<Vec<char>>>(exclusions_primitive: T) -> Result<char> {
    let exclusions: Vec<char> = exclusions_primitive.into();
    if exclusions.len() == UNKNOWNS.len() {
        return Err(anyhow!("Too many exclusions when getting unknown"));
    }
    while let Ok(chosen_char) = get_random(&UNKNOWNS) {
        if !exclusions.contains(&chosen_char) {
            return Ok(chosen_char);
        }
    }
    return Err(anyhow!("Too many exclusions when getting unknown"));
}

pub fn get_two_unknowns() -> Result<(char, char)> {
    let mut rng = rand::rng();
    [
        ('a', 'b'),
        ('j', 'k'),
        ('m', 'n'),
        ('p', 'q'),
        ('t', 'u'),
        ('x', 'y'),
    ]
    .choose(&mut rng)
    .ok_or(anyhow!("The get_two_unknowns array is somehow empty?"))
    .copied()
}

// pub fn get_three_unknowns() -> Result<(char, char, char)> {
//     let mut rng = rand::rng();
//     [
//         ('a', 'b', 'c'),
//         ('p', 'q', 'r'),
//         ('t', 'u', 'v'),
//         ('x', 'y', 'z'),
//     ]
//     .choose(&mut rng)
//     .ok_or(crate::Error::EmptyStatic)
//     .copied()
// }

/// Get a random function name - the f in f(x).
pub fn get_function_name() -> Result<char> {
    get_random(&FUNCTION_NAMES)
}

/// Get a random **function** variable, i.e. the x in f(x).
pub fn get_variable() -> Result<char> {
    get_random(&VARIABLES)
}

fn get_random(symbols: &[Symbol]) -> Result<char> {
    let mut rng = rand::rng();
    symbols
        .choose_weighted(&mut rng, |symbol| symbol.weight)
        .context("The symbols array is somehow empty")
        .map(|symbol| symbol.char)
}

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

/// Function names are used for the form f(x) - note that y is not included here
/// since it is uncommon in Sweden to explicitly write y(x).
static FUNCTION_NAMES: [Symbol; 5] = [
    Symbol::new('f', 20),
    Symbol::new('g', 5),
    Symbol::new('h', 4),
    Symbol::new('s', 2),
    Symbol::new('v', 5),
];

static VARIABLES: [Symbol; 2] = [Symbol::new('x', 7), Symbol::new('t', 1)];
